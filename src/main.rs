use freya::prelude::*;
use std::env;
use std::process::Command;
use std::sync::Arc;

mod components;
mod functions;
mod dialog_window;

use components::{MenuBar, ButtonBar, MySidebar, TabsBar, TabsContent, Footer};
use functions::{
    select_toml_file, load_config, rectangles_m_to_normalized, 
    probes_m_to_normalized, cylindrical_sources_m_to_normalized, Modelling,
};

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
        // Проверяем наличие временного файла при запуске
        let temp_file = std::env::temp_dir().join("maxwell_temp_config.toml");
        if temp_file.exists() {
            println!("Обнаружен временный файл конфигурации от предыдущей сессии: {:?}", temp_file);
            println!("Вы можете использовать 'Настройки проекта' для загрузки этой конфигурации");
        }
        
        // Регистрируем обработчик очистки временного файла при закрытии программы
        let cleanup_handler = move || {
            if temp_file.exists() {
                if let Err(e) = std::fs::remove_file(&temp_file) {
                    eprintln!("Не удалось удалить временный файл: {}", e);
                } else {
                    println!("Временный файл удален при закрытии программы");
                }
            }
        };
        
        // Запускаем основное приложение
        launch(app);
        
        // Вызываем очистку при завершении
        cleanup_handler();
    }
}


#[allow(non_snake_case)]
fn app() -> Element {
    let open_dropdown  = use_signal(|| String::new());
    let active_tab     = use_signal(|| "geometry".to_string());
    let rectangles     = use_signal(|| Arc::<Vec<((f32, f32),(f32, f32))>>::new(Vec::new()));
    let sources        = use_signal(|| Arc::<Vec<(f32, f32)>>::new(Vec::new()));
    let probes         = use_signal(|| Arc::<Vec<(f32, f32)>>::new(Vec::new()));
    let modelling      = use_signal(|| None::<Modelling>);

    // Этот node сигнал привяжем к панели, где рисуем
    let (canvas_ref, canvas_size) = use_node_signal();

    // Обработчик "Open Folder"
    let on_open = {
        let mut rectangles = rectangles.clone();
        let mut sources = sources.clone();
        let mut probes = probes.clone();
        let mut modelling  = modelling.clone();
        let canvas_size    = canvas_size.clone();
        move |_| {
            if let Some(path) = select_toml_file() {
                println!("Выбран файл: {:?}", path);
                match load_config(&path) {
                    Ok(cfg) => {
                        println!("Загружена конфигурация:\n{:#?}", cfg);

                        // сохраняем modelling
                        let m = cfg.modelling;
                        modelling.set(Some(m.clone()));

                        // Измеряем реальные размеры холста (для информации)
                        let area = canvas_size.peek().area;
                        let canvas_w = area.width();
                        let canvas_h = area.height();
                        println!("Холст (px): {:.0}×{:.0}; область (m): {}×{}", canvas_w, canvas_h, m.sizex, m.sizey);

                        // очищаем старые данные
                        rectangles.set(Arc::new(Vec::new()));
                        sources.set(Arc::new(Vec::new()));
                        probes.set(Arc::new(Vec::new()));

                        // Конвертируем прямоугольники из метров в нормализованные (0..1)
                        let normalized_rects = rectangles_m_to_normalized(
                            &cfg.geometry.rectangle,
                            m.sizex,
                            m.sizey,
                        );

                        // Конвертируем зонды из метров в нормализованные (0..1)
                        let normalized_probes = probes_m_to_normalized(
                            &cfg.probes.probe,
                            m.sizex,
                            m.sizey,
                        );

                        // Конвертируем цилиндрические источники из метров в нормализованные (0..1)
                        let normalized_sources = cylindrical_sources_m_to_normalized(
                            &cfg.sources.cylindrical,
                            m.sizex,
                            m.sizey,
                        );

                        println!("Прямоугольники (нормализованные): {:#?}", normalized_rects);
                        println!("Зонды (нормализованные): {:#?}", normalized_probes);
                        println!("Источники (нормализованные): {:#?}", normalized_sources);

                        // Устанавливаем нормализованные координаты — канва перерисует автоматически
                        rectangles.set(Arc::new(normalized_rects));
                        probes.set(Arc::new(normalized_probes));
                        sources.set(Arc::new(normalized_sources));
                    }
                    Err(e) => {
                        eprintln!("Ошибка загрузки конфигурации: {:?}", e);
                    }
                }
            }
        }
    };

    // Обработчик открытия окна настроек проекта — теперь открываем отдельное системное окно (второй процесс)
    let on_open_project_settings = {
        let mut rectangles = rectangles.clone();
        let mut sources = sources.clone();
        let mut probes = probes.clone();
        let mut modelling = modelling.clone();
        let canvas_size = canvas_size.clone();
        move |_| {
            if let Ok(current_exe) = std::env::current_exe() {
                match Command::new(current_exe).arg("--add-dialog").spawn() {
                    Ok(mut child) => {
                        // Дожидаемся закрытия диалогового окна
                        let _ = child.wait();

                        // Сразу после закрытия читаем временный файл
                        let temp_file = std::env::temp_dir().join("maxwell_temp_config.toml");
                        if temp_file.exists() {
                            println!("Найден временный файл конфигурации: {:?}", temp_file);
                            match load_config(&temp_file) {
                                Ok(cfg) => {
                                    println!("Загружена временная конфигурация:\n{:#?}", cfg);

                                    let m = cfg.modelling;
                                    modelling.set(Some(m.clone()));

                                    let area = canvas_size.peek().area;
                                    let canvas_w = area.width();
                                    let canvas_h = area.height();
                                    println!("Холст (px): {:.0}×{:.0}; область (m): {}×{}", canvas_w, canvas_h, m.sizex, m.sizey);

                                    rectangles.set(Arc::new(Vec::new()));
                                    sources.set(Arc::new(Vec::new()));
                                    probes.set(Arc::new(Vec::new()));

                                    let normalized_rects = rectangles_m_to_normalized(
                                        &cfg.geometry.rectangle, m.sizex, m.sizey,
                                    );
                                    let normalized_probes = probes_m_to_normalized(
                                        &cfg.probes.probe, m.sizex, m.sizey,
                                    );
                                    let normalized_sources = cylindrical_sources_m_to_normalized(
                                        &cfg.sources.cylindrical, m.sizex, m.sizey,
                                    );

                                    rectangles.set(Arc::new(normalized_rects));
                                    probes.set(Arc::new(normalized_probes));
                                    sources.set(Arc::new(normalized_sources));

                                    // Временный файл сохраняется до закрытия программы
                                    println!("Временный файл сохранен: {:?}", temp_file);
                                }
                                Err(e) => {
                                    eprintln!("Ошибка загрузки временной конфигурации: {:?}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Не удалось запустить диалоговое окно: {e:?}");
                    }
                }
            }
        }
    };

    // Обработчик создания прямоугольника
    let on_create_rectangle = {
        let modelling = modelling.clone();
        let mut rectangles = rectangles.clone();
        let mut sources = sources.clone();
        let mut probes = probes.clone();
        let canvas_size = canvas_size.clone();
        move |_| {
            // Проверяем, что настройки проекта заданы
            if modelling.read().is_none() {
                println!("Ошибка: Сначала настройте параметры рабочей области через кнопку 'Add'");
                return;
            }
            
            if let Ok(current_exe) = std::env::current_exe() {
                match Command::new(current_exe).arg("--rectangle-dialog").spawn() {
                    Ok(mut child) => {
                        let _ = child.wait();
                        println!("Диалог создания прямоугольника закрыт");
                        
                        // Перезагружаем временный файл для обновления холста
                        let temp_file = std::env::temp_dir().join("maxwell_temp_config.toml");
                        if temp_file.exists() {
                            match load_config(&temp_file) {
                                Ok(cfg) => {
                                    let m = cfg.modelling;
                                    let area = canvas_size.peek().area;
                                    let canvas_w = area.width();
                                    let canvas_h = area.height();
                                    println!("Холст (px): {:.0}×{:.0}; область (m): {}×{}", canvas_w, canvas_h, m.sizex, m.sizey);

                                    let normalized_rects = rectangles_m_to_normalized(
                                        &cfg.geometry.rectangle, m.sizex, m.sizey,
                                    );
                                    let normalized_probes = probes_m_to_normalized(
                                        &cfg.probes.probe, m.sizex, m.sizey,
                                    );
                                    let normalized_sources = cylindrical_sources_m_to_normalized(
                                        &cfg.sources.cylindrical, m.sizex, m.sizey,
                                    );

                                    rectangles.set(Arc::new(normalized_rects));
                                    probes.set(Arc::new(normalized_probes));
                                    sources.set(Arc::new(normalized_sources));
                                }
                                Err(e) => {
                                    eprintln!("Ошибка загрузки временной конфигурации: {:?}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Не удалось запустить диалог создания прямоугольника: {e:?}");
                    }
                }
            }
        }
    };

    // Обработчик создания источника
    let on_create_source = {
        let modelling = modelling.clone();
        let mut rectangles = rectangles.clone();
        let mut sources = sources.clone();
        let mut probes = probes.clone();
        let canvas_size = canvas_size.clone();
        move |_| {
            // Проверяем, что настройки проекта заданы
            if modelling.read().is_none() {
                println!("Ошибка: Сначала настройте параметры рабочей области через кнопку 'Add'");
                return;
            }
            
            if let Ok(current_exe) = std::env::current_exe() {
                match Command::new(current_exe).arg("--source-dialog").spawn() {
                    Ok(mut child) => {
                        let _ = child.wait();
                        println!("Диалог создания источника закрыт");
                        
                        // Перезагружаем временный файл для обновления холста
                        let temp_file = std::env::temp_dir().join("maxwell_temp_config.toml");
                        if temp_file.exists() {
                            match load_config(&temp_file) {
                                Ok(cfg) => {
                                    let m = cfg.modelling;
                                    let area = canvas_size.peek().area;
                                    let canvas_w = area.width();
                                    let canvas_h = area.height();
                                    println!("Холст (px): {:.0}×{:.0}; область (m): {}×{}", canvas_w, canvas_h, m.sizex, m.sizey);

                                    let normalized_rects = rectangles_m_to_normalized(
                                        &cfg.geometry.rectangle, m.sizex, m.sizey,
                                    );
                                    let normalized_probes = probes_m_to_normalized(
                                        &cfg.probes.probe, m.sizex, m.sizey,
                                    );
                                    let normalized_sources = cylindrical_sources_m_to_normalized(
                                        &cfg.sources.cylindrical, m.sizex, m.sizey,
                                    );

                                    rectangles.set(Arc::new(normalized_rects));
                                    probes.set(Arc::new(normalized_probes));
                                    sources.set(Arc::new(normalized_sources));
                                }
                                Err(e) => {
                                    eprintln!("Ошибка загрузки временной конфигурации: {:?}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Не удалось запустить диалог создания источника: {e:?}");
                    }
                }
            }
        }
    };

    // Обработчик создания зонда
    let on_create_probe = {
        let modelling = modelling.clone();
        let mut rectangles = rectangles.clone();
        let mut sources = sources.clone();
        let mut probes = probes.clone();
        let canvas_size = canvas_size.clone();
        move |_| {
            // Проверяем, что настройки проекта заданы
            if modelling.read().is_none() {
                println!("Ошибка: Сначала настройте параметры рабочей области через кнопку 'Add'");
                return;
            }
            
            if let Ok(current_exe) = std::env::current_exe() {
                match Command::new(current_exe).arg("--probe-dialog").spawn() {
                    Ok(mut child) => {
                        let _ = child.wait();
                        println!("Диалог создания зонда закрыт");
                        
                        // Перезагружаем временный файл для обновления холста
                        let temp_file = std::env::temp_dir().join("maxwell_temp_config.toml");
                        if temp_file.exists() {
                            match load_config(&temp_file) {
                                Ok(cfg) => {
                                    let m = cfg.modelling;
                                    let area = canvas_size.peek().area;
                                    let canvas_w = area.width();
                                    let canvas_h = area.height();
                                    println!("Холст (px): {:.0}×{:.0}; область (m): {}×{}", canvas_w, canvas_h, m.sizex, m.sizey);

                                    let normalized_rects = rectangles_m_to_normalized(
                                        &cfg.geometry.rectangle, m.sizex, m.sizey,
                                    );
                                    let normalized_probes = probes_m_to_normalized(
                                        &cfg.probes.probe, m.sizex, m.sizey,
                                    );
                                    let normalized_sources = cylindrical_sources_m_to_normalized(
                                        &cfg.sources.cylindrical, m.sizex, m.sizey,
                                    );

                                    rectangles.set(Arc::new(normalized_rects));
                                    probes.set(Arc::new(normalized_probes));
                                    sources.set(Arc::new(normalized_sources));
                                }
                                Err(e) => {
                                    eprintln!("Ошибка загрузки временной конфигурации: {:?}", e);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Не удалось запустить диалог создания зонда: {e:?}");
                    }
                }
            }
        }
    };



    // Показываем основной интерфейс
    rsx!(
        rect { content:"flex", direction:"vertical", width:"100%", height:"100%",
            MenuBar { open_dropdown: open_dropdown.clone() }
            ButtonBar {
                active_tab: active_tab.clone(),
                on_open: on_open.clone(),
                on_open_project_settings: on_open_project_settings.clone(),
                on_create_rectangle: on_create_rectangle.clone(),
                on_create_source: on_create_source.clone(),
                on_create_probe: on_create_probe.clone(),
            }

            rect { width:"100%", height:"flex(1)",
                ResizableContainer { direction:"horizontal",
                    ResizablePanel { initial_size:20.0, min_size:10.0, MySidebar {} }
                    ResizablePanel { initial_size:100.0, min_size:50.0,
                        rect { reference: canvas_ref, content:"flex", direction:"vertical",
                            rect { height:"40", TabsBar { active_tab: active_tab.clone() } }
                            rect { height:"flex(1)",
                                TabsContent {
                                    active_tab: active_tab.clone(),
                                    rectangles: rectangles.clone(),
                                    sources: sources.clone(),
                                    probes: probes.clone(),
                                }
                            }
                        }
                    }
                }
            }

            Footer {}
        }
    )
}