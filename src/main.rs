use freya::prelude::*;
use components::{MenuBar, ButtonBar, MySidebar, TabsBar, TabsContent, Footer};
mod components;

mod functions;
use functions::{select_toml_file, load_config, rectangles_m_to_normalized};

fn main() {
    launch(app);
}

#[allow(non_snake_case)]
fn app() -> Element {
    let open_dropdown  = use_signal(|| String::new());
    let active_tab     = use_signal(|| "geometry".to_string());
    let draw_rect_mode = use_signal(|| false);
    let first_point    = use_signal(|| None::<(f32, f32)>);
    let rectangles     = use_signal(|| Vec::<((f32, f32),(f32, f32))>::new());
    let modelling      = use_signal(|| None::<functions::Modelling>);

    // Этот node сигнал привяжем к панели, где рисуем
    let (canvas_ref, canvas_size) = use_node_signal();

    // Обработчик "Open Folder"
    let on_open = {
        let mut rectangles = rectangles.clone();
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

                        // очищаем старые прямоугольники
                        rectangles.set(vec![]);

                        // Конвертируем из метров в нормализованные (0..1) — НОВОЕ:
                        let normalized = rectangles_m_to_normalized(
                            &cfg.geometry.rectangle,
                            m.sizex,
                            m.sizey,
                        );

                        println!("Прямоугольники (нормализованные): {:#?}", normalized);

                        // Устанавливаем нормализованные координаты — канва перерисует автоматически
                        rectangles.set(normalized);
                    }
                    Err(e) => {
                        eprintln!("Ошибка загрузки конфигурации: {:?}", e);
                    }
                }
            }
        }
    };

    rsx!(
        rect { content:"flex", direction:"vertical", width:"100%", height:"100%",
            MenuBar { open_dropdown }
            ButtonBar {
                active_tab: active_tab.clone(),
                draw_rect_mode: draw_rect_mode.clone(),
                on_open: on_open.clone()
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
                                    first_point: first_point.clone(),
                                    rectangles: rectangles.clone(),
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