use freya::prelude::*;
use components::{MenuBar, ButtonBar, MySidebar, TabsBar, TabsContent, Footer};
mod components;

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

    rsx!(
        rect { content:"flex", direction:"vertical", width:"100%", height:"100%",
            MenuBar { open_dropdown }
            ButtonBar { active_tab: active_tab.clone(), draw_rect_mode: draw_rect_mode.clone() }

            rect { width:"100%", height:"flex(1)",
                ResizableContainer { direction:"horizontal",
                    ResizablePanel { initial_size:20.0, min_size:10.0, MySidebar {} }
                    ResizableHandle {}
                    ResizablePanel { initial_size:100.0, min_size:50.0,
                        rect { content:"flex", direction:"vertical",
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