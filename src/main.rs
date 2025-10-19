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
    } else {
        launch(app);
    }
}


#[allow(non_snake_case)]
fn app() -> Element {
    let open_dropdown  = use_signal(|| String::new());
    let active_tab     = use_signal(|| "geometry".to_string());
    let draw_rect_mode = use_signal(|| false);
    let draw_source_mode = use_signal(|| false);
    let draw_probe_mode = use_signal(|| false);
    let first_point    = use_signal(|| None::<(f32, f32)>);
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
                            &cfg.sources.cylindical,
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
                                        &cfg.sources.cylindical, m.sizex, m.sizey,
                                    );

                                    rectangles.set(Arc::new(normalized_rects));
                                    probes.set(Arc::new(normalized_probes));
                                    sources.set(Arc::new(normalized_sources));

                                    let _ = std::fs::remove_file(&temp_file);
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



    // Показываем основной интерфейс
    rsx!(
        rect { content:"flex", direction:"vertical", width:"100%", height:"100%",
            MenuBar { open_dropdown: open_dropdown.clone() }
            ButtonBar {
                active_tab: active_tab.clone(),
                draw_rect_mode: draw_rect_mode.clone(),
                draw_source_mode: draw_source_mode.clone(),
                draw_probe_mode: draw_probe_mode.clone(),
                on_open: on_open.clone(),
                on_open_project_settings: on_open_project_settings.clone()
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
                                    draw_rect_mode: draw_rect_mode.clone(),
                                    draw_source_mode: draw_source_mode.clone(),
                                    draw_probe_mode: draw_probe_mode.clone(),
                                    first_point: first_point.clone(),
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