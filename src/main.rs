use freya::prelude::*;
use std::env;
use std::process::{Command, Stdio};
use std::sync::Arc;

mod components;
mod dialog_window;
mod fdtd;
mod functions;

use components::{ButtonBar, Footer, MenuBar, MySidebar, SidebarSelection, TabsBar, TabsContent, WaveType};
use dialog_window::{CONFIG_START_MARKER, CONFIG_END_MARKER};
use fdtd::{Fdtd2dTe, Fdtd2dTm, FdtParams};
use functions::{generate_toml_config, load_config, save_config_to_file, save_toml_file_dialog, select_toml_file, Modelling};
use std::path::PathBuf;

/// Парсит вывод диалога и извлекает данные конфигурации
fn parse_dialog_output(output: &str) -> Option<String> {
    let start_idx = output.find(CONFIG_START_MARKER)?;
    let end_idx = output.find(CONFIG_END_MARKER)?;
    
    if start_idx < end_idx {
        let config_start = start_idx + CONFIG_START_MARKER.len();
        let config_data = output[config_start..end_idx].trim();
        Some(config_data.to_string())
    } else {
        None
    }
}

/// Парсит TOML-конфигурацию из stdout диалога настроек проекта
fn parse_project_config(toml_str: &str) -> Option<functions::Config> {
    toml::from_str(toml_str).ok()
}

/// Парсит данные одного объекта из stdout диалога (Rectangle/Source/Probe)
fn parse_object_data(data: &str) -> Option<ObjectData> {
    let parts: Vec<&str> = data.split('|').collect();
    if parts.is_empty() {
        return None;
    }
    
    match parts[0] {
        "RECTANGLE" if parts.len() >= 7 => {
            Some(ObjectData::Rectangle {
                x1: parts[1].parse().ok()?,
                y1: parts[2].parse().ok()?,
                x2: parts[3].parse().ok()?,
                y2: parts[4].parse().ok()?,
                eps: parts[5].parse().ok()?,
                mu: parts[6].parse().ok()?,
            })
        }
        "SOURCE" if parts.len() >= 3 => {
            Some(ObjectData::Source {
                x: parts[1].parse().ok()?,
                y: parts[2].parse().ok()?,
            })
        }
        "PROBE" if parts.len() >= 3 => {
            Some(ObjectData::Probe {
                x: parts[1].parse().ok()?,
                y: parts[2].parse().ok()?,
            })
        }
        _ => None
    }
}

/// Данные объекта, полученные из диалога
#[allow(dead_code)]
enum ObjectData {
    Rectangle { x1: f32, y1: f32, x2: f32, y2: f32, eps: f32, mu: f32 },
    Source { x: f32, y: f32 },
    Probe { x: f32, y: f32 },
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|a| a == "--add-dialog") {
        // Запускаем отдельное окно диалога добавления объекта
        dialog_window::launch_dialog_app();
    } else if args.iter().any(|a| a == "--rectangle-dialog") {
        // Запускаем диалог создания прямоугольника
        dialog_window::launch_rectangle_dialog();
    } else if args.iter().any(|a| a == "--source-dialog") {
        // Запускаем диалог создания источника
        dialog_window::launch_source_dialog();
    } else if args.iter().any(|a| a == "--probe-dialog") {
        // Запускаем диалог создания зонда
        dialog_window::launch_probe_dialog();
    } else {
        // Запускаем основное приложение
        launch_cfg(
            app,
            LaunchConfig::<()>::new()
                .with_size(1600.0, 900.0)
        );
    }
}

#[allow(non_snake_case)]
fn app() -> Element {
    let open_dropdown = use_signal(|| String::new());
    let active_tab = use_signal(|| "geometry".to_string());

    // Нормализованные координаты для отображения (0..1)
    let rectangles = use_signal(|| Arc::<Vec<((f32, f32), (f32, f32))>>::new(Vec::new()));
    let sources = use_signal(|| Arc::<Vec<(f32, f32)>>::new(Vec::new()));
    let probes = use_signal(|| Arc::<Vec<(f32, f32)>>::new(Vec::new()));

    // Параметры моделирования
    let modelling = use_signal(|| None::<Modelling>);
    let running = use_signal(|| false);
    let resuming = use_signal(|| false);
    let step_counter = use_signal(|| 0_usize);

    // Счётчики для уникальных номеров объектов
    let next_rect_id = use_signal(|| 1_usize);
    let next_source_id = use_signal(|| 1_usize);
    let next_probe_id = use_signal(|| 1_usize);

    // Векторы ID для отслеживания порядковых номеров объектов
    let rect_ids = use_signal(|| Arc::<Vec<usize>>::new(Vec::new()));
    let source_ids = use_signal(|| Arc::<Vec<usize>>::new(Vec::new()));
    let probe_ids = use_signal(|| Arc::<Vec<usize>>::new(Vec::new()));

    // Путь к текущему открытому файлу (для функции сохранения)
    let current_file_path = use_signal(|| None::<PathBuf>);

    // Общий сигнал для выделения объектов (синхронизация между боковой панелью и холстом)
    let sidebar_selection = use_signal(|| None::<SidebarSelection>);

    // Тип волны и симуляции FDTD для TE и TM
    let wave_type = use_signal(|| WaveType::TE);
    let sim_te = use_signal(|| Fdtd2dTe::new(FdtParams::default()));
    let sim_tm = use_signal(|| Fdtd2dTm::new(FdtParams::default()));
    let field_data = use_signal(|| {
        let s = Fdtd2dTe::new(FdtParams::default());
        let (sx, sy) = s.size();
        (sx, sy, s.ey().to_vec())
    });

    let (canvas_ref, canvas_size) = use_node_signal();

    // Обработчик "Open Folder" - загрузка конфигурации из файла
    let on_open = {
        let mut rectangles = rectangles.clone();
        let mut sources = sources.clone();
        let mut probes = probes.clone();
        let mut modelling = modelling.clone();
        let canvas_size = canvas_size.clone();
        let mut next_rect_id = next_rect_id.clone();
        let mut next_source_id = next_source_id.clone();
        let mut next_probe_id = next_probe_id.clone();
        let mut rect_ids = rect_ids.clone();
        let mut source_ids = source_ids.clone();
        let mut probe_ids = probe_ids.clone();
        let mut current_file_path = current_file_path.clone();

        move |_| {
            if let Some(path) = select_toml_file() {
                println!("Выбран файл: {:?}", path);
                match load_config(&path) {
                    Ok(cfg) => {
                        println!("Загружена конфигурация:\n{:#?}", cfg);

                        // Сохраняем путь к открытому файлу для последующего сохранения
                        current_file_path.set(Some(path));

                        let m = cfg.modelling;
                        modelling.set(Some(m.clone()));

                        let area = canvas_size.peek().area;
                        println!(
                            "Холст (px): {:.0}×{:.0}; область (m): {}×{}",
                            area.width(),
                            area.height(),
                            m.sizex,
                            m.sizey
                        );

                        // Конвертируем в нормализованные координаты
                        let norm_rects: Vec<((f32, f32), (f32, f32))> = cfg
                            .geometry
                            .rectangle
                            .iter()
                            .map(|r| {
                                (
                                    (r.x1 / m.sizex, r.y1 / m.sizey),
                                    (r.x2 / m.sizex, r.y2 / m.sizey),
                                )
                            })
                            .collect();

                        let norm_sources: Vec<(f32, f32)> = cfg
                            .sources
                            .cylindrical
                            .iter()
                            .map(|s| (s.x / m.sizex, s.y / m.sizey))
                            .collect();

                        let norm_probes: Vec<(f32, f32)> = cfg
                            .probes
                            .probe
                            .iter()
                            .map(|p| (p.x / m.sizex, p.y / m.sizey))
                            .collect();

                        // Генерируем ID
                        let rect_count = norm_rects.len();
                        let source_count = norm_sources.len();
                        let probe_count = norm_probes.len();

                        rect_ids.set(Arc::new((1..=rect_count).collect()));
                        source_ids.set(Arc::new((1..=source_count).collect()));
                        probe_ids.set(Arc::new((1..=probe_count).collect()));

                        next_rect_id.set(rect_count + 1);
                        next_source_id.set(source_count + 1);
                        next_probe_id.set(probe_count + 1);

                        rectangles.set(Arc::new(norm_rects));
                        sources.set(Arc::new(norm_sources));
                        probes.set(Arc::new(norm_probes));
                    }
                    Err(e) => {
                        eprintln!("Ошибка загрузки конфигурации: {:?}", e);
                    }
                }
            }
        }
    };

    // Обработчик открытия диалога настроек проекта (отдельное окно)
    let on_open_project_settings = {
        let mut rectangles = rectangles.clone();
        let mut sources = sources.clone();
        let mut probes = probes.clone();
        let mut modelling = modelling.clone();
        let mut next_rect_id = next_rect_id.clone();
        let mut next_source_id = next_source_id.clone();
        let mut next_probe_id = next_probe_id.clone();
        let mut rect_ids = rect_ids.clone();
        let mut source_ids = source_ids.clone();
        let mut probe_ids = probe_ids.clone();
        
        move |_| {
            if let Ok(current_exe) = std::env::current_exe() {
                // Запускаем диалог и захватываем stdout
                match Command::new(current_exe)
                    .arg("--add-dialog")
                    .stdout(Stdio::piped())
                    .spawn()
                {
                    Ok(child) => {
                        match child.wait_with_output() {
                            Ok(output) => {
                                let stdout = String::from_utf8_lossy(&output.stdout);
                                
                                // Парсим конфигурацию из stdout
                                if let Some(config_str) = parse_dialog_output(&stdout) {
                                    if let Some(cfg) = parse_project_config(&config_str) {
                                        let m = cfg.modelling;
                                        modelling.set(Some(m.clone()));

                                        let norm_rects: Vec<_> = cfg.geometry.rectangle.iter()
                                            .map(|r| ((r.x1 / m.sizex, r.y1 / m.sizey), (r.x2 / m.sizex, r.y2 / m.sizey)))
                                            .collect();
                                        let norm_sources: Vec<_> = cfg.sources.cylindrical.iter()
                                            .map(|s| (s.x / m.sizex, s.y / m.sizey))
                                            .collect();
                                        let norm_probes: Vec<_> = cfg.probes.probe.iter()
                                            .map(|p| (p.x / m.sizex, p.y / m.sizey))
                                            .collect();

                                        rect_ids.set(Arc::new((1..=norm_rects.len()).collect()));
                                        source_ids.set(Arc::new((1..=norm_sources.len()).collect()));
                                        probe_ids.set(Arc::new((1..=norm_probes.len()).collect()));

                                        next_rect_id.set(norm_rects.len() + 1);
                                        next_source_id.set(norm_sources.len() + 1);
                                        next_probe_id.set(norm_probes.len() + 1);

                                        rectangles.set(Arc::new(norm_rects));
                                        sources.set(Arc::new(norm_sources));
                                        probes.set(Arc::new(norm_probes));
                                    }
                                }
                            }
                            Err(e) => eprintln!("Ошибка при ожидании процесса: {e:?}"),
                        }
                    }
                    Err(e) => {
                        eprintln!("Не удалось запустить диалоговое окно: {e:?}");
                    }
                }
            }
        }
    };

    // Обработчик создания прямоугольника (отдельное окно)
    let on_create_rectangle = {
        let modelling_check = modelling.clone();
        let mut rectangles = rectangles.clone();
        let mut next_rect_id = next_rect_id.clone();
        let mut rect_ids = rect_ids.clone();
        
        move |_| {
            let m = match modelling_check.read().clone() {
                Some(m) => m,
                None => {
                    eprintln!("Ошибка: Сначала настройте параметры рабочей области через кнопку 'Add'");
                    return;
                }
            };
            
            if let Ok(current_exe) = std::env::current_exe() {
                match Command::new(current_exe)
                    .arg("--rectangle-dialog")
                    .stdout(Stdio::piped())
                    .spawn()
                {
                    Ok(child) => {
                        match child.wait_with_output() {
                            Ok(output) => {
                                let stdout = String::from_utf8_lossy(&output.stdout);
                                
                                if let Some(data_str) = parse_dialog_output(&stdout) {
                                    if let Some(ObjectData::Rectangle { x1, y1, x2, y2, eps: _, mu: _ }) = parse_object_data(&data_str) {
                                        // Нормализуем координаты
                                        let nx1 = x1 / m.sizex;
                                        let ny1 = y1 / m.sizey;
                                        let nx2 = x2 / m.sizex;
                                        let ny2 = y2 / m.sizey;
                                        
                                        // Добавляем прямоугольник
                                        let mut rects = rectangles.read().as_ref().clone();
                                        rects.push(((nx1, ny1), (nx2, ny2)));
                                        rectangles.set(Arc::new(rects));
                                        
                                        // Обновляем ID
                                        let mut ids = rect_ids.read().as_ref().clone();
                                        let new_id = *next_rect_id.read();
                                        ids.push(new_id);
                                        rect_ids.set(Arc::new(ids));
                                        next_rect_id.set(new_id + 1);
                                    }
                                }
                            }
                            Err(e) => eprintln!("Ошибка при ожидании процесса: {e:?}"),
                        }
                    }
                    Err(e) => {
                        eprintln!("Не удалось запустить диалог создания прямоугольника: {e:?}");
                    }
                }
            }
        }
    };

    // Обработчик создания источника (отдельное окно)
    let on_create_source = {
        let modelling_check = modelling.clone();
        let mut sources = sources.clone();
        let mut next_source_id = next_source_id.clone();
        let mut source_ids = source_ids.clone();
        
        move |_| {
            let m = match modelling_check.read().clone() {
                Some(m) => m,
                None => {
                    eprintln!("Ошибка: Сначала настройте параметры рабочей области через кнопку 'Add'");
                    return;
                }
            };
            
            if let Ok(current_exe) = std::env::current_exe() {
                match Command::new(current_exe)
                    .arg("--source-dialog")
                    .stdout(Stdio::piped())
                    .spawn()
                {
                    Ok(child) => {
                        match child.wait_with_output() {
                            Ok(output) => {
                                let stdout = String::from_utf8_lossy(&output.stdout);
                                
                                if let Some(data_str) = parse_dialog_output(&stdout) {
                                    if let Some(ObjectData::Source { x, y }) = parse_object_data(&data_str) {
                                        // Нормализуем координаты
                                        let nx = x / m.sizex;
                                        let ny = y / m.sizey;
                                        
                                        // Добавляем источник
                                        let mut srcs = sources.read().as_ref().clone();
                                        srcs.push((nx, ny));
                                        sources.set(Arc::new(srcs));
                                        
                                        // Обновляем ID
                                        let mut ids = source_ids.read().as_ref().clone();
                                        let new_id = *next_source_id.read();
                                        ids.push(new_id);
                                        source_ids.set(Arc::new(ids));
                                        next_source_id.set(new_id + 1);
                                    }
                                }
                            }
                            Err(e) => eprintln!("Ошибка при ожидании процесса: {e:?}"),
                        }
                    }
                    Err(e) => {
                        eprintln!("Не удалось запустить диалог создания источника: {e:?}");
                    }
                }
            }
        }
    };

    // Обработчик создания датчика (отдельное окно)
    let on_create_probe = {
        let modelling_check = modelling.clone();
        let mut probes = probes.clone();
        let mut next_probe_id = next_probe_id.clone();
        let mut probe_ids = probe_ids.clone();
        
        move |_| {
            let m = match modelling_check.read().clone() {
                Some(m) => m,
                None => {
                    eprintln!("Ошибка: Сначала настройте параметры рабочей области через кнопку 'Add'");
                    return;
                }
            };
            
            if let Ok(current_exe) = std::env::current_exe() {
                match Command::new(current_exe)
                    .arg("--probe-dialog")
                    .stdout(Stdio::piped())
                    .spawn()
                {
                    Ok(child) => {
                        match child.wait_with_output() {
                            Ok(output) => {
                                let stdout = String::from_utf8_lossy(&output.stdout);
                                
                                if let Some(data_str) = parse_dialog_output(&stdout) {
                                    if let Some(ObjectData::Probe { x, y }) = parse_object_data(&data_str) {
                                        // Нормализуем координаты
                                        let nx = x / m.sizex;
                                        let ny = y / m.sizey;
                                        
                                        // Добавляем датчик
                                        let mut prbs = probes.read().as_ref().clone();
                                        prbs.push((nx, ny));
                                        probes.set(Arc::new(prbs));
                                        
                                        // Обновляем ID
                                        let mut ids = probe_ids.read().as_ref().clone();
                                        let new_id = *next_probe_id.read();
                                        ids.push(new_id);
                                        probe_ids.set(Arc::new(ids));
                                        next_probe_id.set(new_id + 1);
                                    }
                                }
                            }
                            Err(e) => eprintln!("Ошибка при ожидании процесса: {e:?}"),
                        }
                    }
                    Err(e) => {
                        eprintln!("Не удалось запустить диалог создания зонда: {e:?}");
                    }
                }
            }
        }
    };

    // Обработчик "Save" - сохранение конфигурации в файл
    let on_save = {
        let modelling = modelling.clone();
        let rectangles = rectangles.clone();
        let sources = sources.clone();
        let probes = probes.clone();
        let mut current_file_path = current_file_path.clone();

        move |_| {
            // Проверяем, что есть данные для сохранения
            let m = match modelling.read().clone() {
                Some(m) => m,
                None => {
                    eprintln!("Ошибка: Нет данных для сохранения. Сначала настройте параметры рабочей области.");
                    return;
                }
            };

            // Определяем путь для сохранения
            // Сначала читаем текущий путь во временную переменную
            let existing_path = current_file_path.read().clone();
            
            let save_path = if let Some(path) = existing_path {
                // Если файл уже открыт, сохраняем в него
                path
            } else {
                // Иначе показываем диалог выбора файла
                match save_toml_file_dialog() {
                    Some(path) => {
                        // Сохраняем путь для последующих сохранений
                        current_file_path.set(Some(path.clone()));
                        path
                    }
                    None => {
                        println!("Сохранение отменено пользователем");
                        return;
                    }
                }
            };

            // Генерируем TOML-контент
            let rects = rectangles.read();
            let srcs = sources.read();
            let prbs = probes.read();
            
            let toml_content = generate_toml_config(&m, &rects, &srcs, &prbs);

            // Сохраняем в файл
            match save_config_to_file(&save_path, &toml_content) {
                Ok(()) => {
                    println!("Конфигурация успешно сохранена в: {:?}", save_path);
                }
                Err(e) => {
                    eprintln!("Ошибка сохранения конфигурации: {:?}", e);
                }
            }
        }
    };

    rsx!(
        rect {
            content: "flex",
            direction: "vertical",
            width: "100%",
            height: "100%",

            MenuBar { open_dropdown: open_dropdown.clone() }
            ButtonBar {
                active_tab: active_tab.clone(),
                on_open: on_open.clone(),
                on_save: on_save.clone(),
                on_open_project_settings: on_open_project_settings.clone(),
                on_create_rectangle: on_create_rectangle.clone(),
                on_create_source: on_create_source.clone(),
                on_create_probe: on_create_probe.clone(),
                on_start: {
                    let mut running = running.clone();
                    let mut resuming = resuming.clone();
                    move |_| {
                        if *running.read() {
                            return;
                        }
                        resuming.set(false);
                        running.set(true);
                    }
                },
                on_stop: {
                    let mut running = running.clone();
                    move |_| running.set(false)
                },
                on_resume: {
                    let mut running = running.clone();
                    let mut resuming = resuming.clone();
                    move |_| {
                        if *running.read() {
                            return;
                        }
                        resuming.set(true);
                        running.set(true);
                    }
                },
            }

            rect {
                width: "100%",
                height: "flex(1)",
                ResizableContainer {
                    direction: "horizontal",
                    ResizablePanel {
                        initial_size: 20.0,
                        min_size: 10.0,
                        MySidebar {
                            rectangles: rectangles.clone(),
                            sources: sources.clone(),
                            probes: probes.clone(),
                            next_rect_id: next_rect_id.clone(),
                            next_source_id: next_source_id.clone(),
                            next_probe_id: next_probe_id.clone(),
                            rect_ids: rect_ids.clone(),
                            source_ids: source_ids.clone(),
                            probe_ids: probe_ids.clone(),
                            selected: sidebar_selection.clone(),
                        }
                    }
                    ResizablePanel {
                        initial_size: 100.0,
                        min_size: 50.0,
                        rect {
                            reference: canvas_ref,
                            content: "flex",
                            direction: "vertical",
                            rect {
                                height: "40",
                                TabsBar { active_tab: active_tab.clone() }
                            }
                            rect {
                                height: "flex(1)",
                                TabsContent {
                                    active_tab: active_tab.clone(),
                                    rectangles: rectangles.clone(),
                                    sources: sources.clone(),
                                    probes: probes.clone(),
                                    running: running.clone(),
                                    resuming: resuming.clone(),
                                    step_counter: step_counter.clone(),
                                    sim_te: sim_te.clone(),
                                    sim_tm: sim_tm.clone(),
                                    wave_type: wave_type.clone(),
                                    field_data: field_data.clone(),
                                    sidebar_selection: sidebar_selection.clone(),
                                }
                            }
                        }
                    }
                }
            }

            Footer { step_counter: step_counter.clone(), wave_type: wave_type.clone() }
        }
    )
}
