use freya::prelude::*;
use freya::core::custom_attributes::CanvasRunnerContext;
use freya::hooks::use_canvas_with_deps;
use freya::hooks::use_focus;
use skia_safe::{Color, Paint, Rect};

// Импорт изображений
import_svg!(OpenedFolder, "../assets/images/opened-folder.svg", {
    width: "100%",
    height: "100%"
});

import_image!(Save, "../assets/images/save.png", {
    width: "100%",
    height: "100%"
});

import_image!(Undo, "../assets/images/undo.png", {
    width: "100%",
    height: "100%"
});

import_image!(Redo, "../assets/images/redo.png", {
    width: "100%",
    height: "100%"
});

import_image!(RgbColorWheel, "../assets/images/rgb-color-wheel.png", {
    width: "100%",
    height: "100%"
});

import_image!(Cursor, "../assets/images/cursor.png", {
    width: "100%",
    height: "100%"
});

import_image!(Rectangle, "../assets/images/rectangle.png", {
    width: "100%",
    height: "100%"
});

import_image!(Ellipse, "../assets/images/ellipse.png", {
    width: "100%",
    height: "100%"
});

import_image!(Source, "../assets/images/source.png", {
    width: "100%",
    height: "100%"
});

import_image!(Probe, "../assets/images/source.png", {
    width: "100%",
    height: "100%"
});

import_image!(Line, "../assets/images/line.png", {
    width: "100%",
    height: "100%"
});

import_image!(Start, "../assets/images/start.png", {
    width: "100%",
    height: "100%"
});

import_image!(Stop, "../assets/images/stop.png", {
    width: "100%",
    height: "100%"
});

import_image!(Field, "../assets/images/field.png", {
    width: "100%",
    height: "100%"
});

import_image!(Signals, "../assets/images/signals.png", {
    width: "100%",
    height: "100%"
});

// Компонент: AppTheme (основные стили)
#[component]
pub fn AppTheme(children: Element) -> Element {
    rsx!(
        rect {
            width: "100%",
            height: "100%",
            background: "rgb(240, 240, 240)",
            direction: "vertical",
            font_family: "Arial, sans-serif",
            {children}
        }
    )
}

// Компонент: MenuBar с кастомными кнопками
#[component]
pub fn MenuBar(open_dropdown: Signal<String>) -> Element {
    let menus = vec!["Файл", "Правка", "Объекты", "Параметры", "Окно", "Справка"];
    rsx!(
        rect {
            content: "flex",
            direction: "horizontal",
            height: "30",
            spacing: "5",
            padding: "0 10",
            background: "rgb(255, 255, 255)",
            border: "1 solid #cccccc",
            cross_align: "center",
            for name in menus {
                rect {
                    width: "auto",
                    height: "100%",
                    padding: "0 2",
                    main_align: "center",
                    cross_align: "center",
                    background: "transparent",
                    onmouseenter: move |_| open_dropdown.set(name.to_string()),
                    onmouseleave: move |_| open_dropdown.set("".to_string()),
                    onclick: move |_| println!("Clicked {}", name),
                    label {
                        "{name}"
                    }
                }
            }
        }
    )
}

// ButtonBar: use flex layout to center icons
#[component]
pub fn ButtonBar(
    active_tab: Signal<String>,
    draw_rect_mode: Signal<bool>,
) -> Element {
    rsx!(
        rect {
            content: "flex",
            direction: "horizontal",
            height: "40",
            spacing: "5",
            padding: "0 10",
            cross_align: "center",

            // Открыть папку
            ButtonIcon {
                tooltip: "Open Folder".to_string(),
                icon: rsx!(OpenedFolder {}),
                onclick: move |_| println!("Open Folder clicked"),
            }
            // Сохранить
            ButtonIcon {
                tooltip: "Save".to_string(),
                icon: rsx!(Save {}),
                onclick: move |_| println!("Save clicked"),
            }
            // Отменить
            ButtonIcon {
                tooltip: "Undo".to_string(),
                icon: rsx!(Undo {}),
                onclick: move |_| println!("Undo clicked"),
            }
            // Повторить
            ButtonIcon {
                tooltip: "Redo".to_string(),
                icon: rsx!(Redo {}),
                onclick: move |_| println!("Redo clicked"),
            }
            // Цвет
            ButtonIcon {
                tooltip: "Choose color".to_string(),
                icon: rsx!(RgbColorWheel {}),
                onclick: move |_| println!("Color clicked"),
            }
            // Курсор
            ButtonIcon {
                tooltip: "Cursor".to_string(),
                icon: rsx!(Cursor {}),
                onclick: move |_| println!("Cursor clicked"),
            }
            // Рисование прямоугольников (только на Геометрии)
            ButtonIcon {
                tooltip: "Rectangle".to_string(),
                icon: rsx!(Rectangle {}),
                onclick: move |_| {
                    if *active_tab.read() == "geometry" {
                        let cur = *draw_rect_mode.read();
                        draw_rect_mode.set(!cur);
                    }
                },
            }
            // Остальные кнопки без логики
            ButtonIcon { tooltip: "Ellipse".to_string(), icon: rsx!(Ellipse {}), onclick: |_| {} }
            ButtonIcon { tooltip: "Source".to_string(), icon: rsx!(Source {}), onclick: |_| {} }
            ButtonIcon { tooltip: "Probe".to_string(), icon: rsx!(Probe {}), onclick: |_| {} }
            ButtonIcon { tooltip: "Line".to_string(), icon: rsx!(Line {}), onclick: |_| {} }
            ButtonIcon { tooltip: "Start".to_string(), icon: rsx!(Start {}), onclick: |_| {} }
            ButtonIcon { tooltip: "Stop".to_string(), icon: rsx!(Stop {}), onclick: |_| {} }
        }
    )
}

// Вспомогательный компонент ButtonIcon
#[component]
pub fn ButtonIcon(
    tooltip: String,
    icon: Element,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx!(
        rect {
            width: "40",
            height: "40",
            margin: "0 5 0 0",
            onclick: onclick.clone(),
            {icon}
        }
    )
}

// Компонент: TabsContent
#[component]
pub fn TabsContent(
    active_tab: Signal<String>,
    draw_rect_mode: Signal<bool>,
    first_point: Signal<Option<(f32, f32)>>,
    rectangles: Signal<Vec<((f32, f32), (f32, f32))>>,
) -> Element {
    let cur = active_tab.read().clone();
    rsx!(
        rect { width: "100%", height: "100%",
            match cur.as_str() {
                "geometry" => rsx!(
                    CanvasDrawArea {
                        draw_rect_mode: draw_rect_mode.clone(),
                        first_point: first_point.clone(),
                        rectangles: rectangles.clone(),
                    }
                ),
                "field" => rsx!(
                    rect {
                        width: "100%",
                        height: "100%",
                        padding: "10",
                        background: "rgb(200,200,200)",
                        Field {}
                    }
                ),
                "signals" => rsx!(
                    rect {
                        width: "100%",
                        height: "100%",
                        padding: "10",
                        background: "rgb(200,200,200)",
                        Signals {}
                    }
                ),
                _ => rsx!(rect { width: "100%", height: "100%" }),
            }
        }
    )
}

// Переименованный компонент Sidebar -> MySidebar
#[component]
pub fn MySidebar() -> Element {
    rsx!(
        rect {
            width: "100%",
            height: "100%",
            background: "rgb(249, 249, 249)",
            padding: "10",
            TreeView {}
        }
    )
}

// Компонент: TreeView
#[component]
pub fn TreeView() -> Element {
    let items = vec![
        ("Объекты", vec!["Прямоугольник 1", "Прямоугольник 2", "Эллипс"]),
        ("Источники", vec!["Цилиндрическая волна 1", "Плоская волна 1"]),
        ("Датчики", vec!["Датчик 1", "Датчик 2", "Датчик 3"]),
    ];

    rsx!(
        rect {
            for (title, children) in items {
                TreeItem {
                    title: title.to_string(),
                    items: children.clone(),
                }
            }
        }
    )
}

// Компонент: TreeItem (исправлена ошибка заимствования)
#[component]
pub fn TreeItem(title: String, items: Vec<&'static str>) -> Element {
    let mut expanded = use_signal(|| true);
    
    rsx!(
        rect {
            direction: "vertical",
            margin: "5 0",
            rect {
                direction: "horizontal",
                onclick: move |_| {
                    let current = *expanded.read();
                    expanded.set(!current);
                },
                label {
                    if *expanded.read() { "▼ " } else { "▶ " }
                    "{title}"
                }
            }
            if *expanded.read() {
                rect {
                    padding: "0 0 0 15",
                    direction: "vertical",
                    for child in items.iter() {
                        rect {
                            padding: "5",
                            label {
                                "{child}"
                            }
                        }
                    }
                }
            }
        }
    )
}

// TabsBar и содержимое
#[component]
pub fn TabsBar(active_tab: Signal<String>) -> Element {
    let tabs = [ ("Геометрия","geometry"),("Поле","field"),("Временные сигналы","signals") ];
    rsx!(
        rect { content:"flex", direction:"horizontal", height:"100%", background:"rgb(240,240,240)", border:"1 solid #ccc",
            for (label,id) in tabs {
                rect { width:"flex(1)", main_align:"center", cross_align:"center",
                    background: if *active_tab.read()==id {"rgb(204,204,204)"} else {"rgb(240,240,240)"},
                    onclick:move |_| active_tab.set(id.to_string()), label { "{label}" }
                }
            }
        }
    )
}

// Компонент: CanvasDrawArea с исправлениями
#[component]
pub fn CanvasDrawArea(
    draw_rect_mode: Signal<bool>,
    first_point: Signal<Option<(f32, f32)>>,
    rectangles: Signal<Vec<((f32, f32), (f32, f32))>>,
) -> Element {
    let platform = use_platform();
    let (reference, size) = use_node_signal();

    // Менеджер фокуса
    let focus = use_focus();
    // Сигнал выбранного прямоугольника
    let selected = use_signal(|| None::<usize>);

    // --- onclick: рисуем или выбираем ---
    let onclick = {
        // клонируем handles, которые будем менять
        let mut sel = selected.clone();
        let mut rects = rectangles.clone();
        let mut focus = focus.clone();
        move |evt: MouseEvent| {
            focus.request_focus();

            let x = evt.element_coordinates.x as f32;
            let y = evt.element_coordinates.y as f32;

            if *draw_rect_mode.read() {
                if first_point.read().is_none() {
                    first_point.set(Some((x, y)));
                } else {
                    let p1 = first_point.read().unwrap();
                    let mut v = rects.read().clone();
                    v.push((p1, (x, y)));
                    rects.set(v);
                    first_point.set(None);
                    draw_rect_mode.set(false);
                    platform.invalidate_drawing_area(size.peek().area);
                    platform.request_animation_frame();
                }
            } else {
                let mut found = None;
                for (idx, &((x1,y1),(x2,y2))) in rects.read().iter().enumerate() {
                    let (minx, maxx) = (x1.min(x2), x1.max(x2));
                    let (miny, maxy) = (y1.min(y2), y1.max(y2));
                    if x >= minx && x <= maxx && y >= miny && y <= maxy {
                        found = Some(idx);
                        break;
                    }
                }
                sel.set(found);
                platform.invalidate_drawing_area(size.peek().area);
                platform.request_animation_frame();
            }
        }
    };

    // --- onkeydown: удаляем по Delete/Backspace ---
    let onkey = {
        let mut rects = rectangles.clone();
        let mut sel  = selected.clone();
        let focus    = focus.clone();
        move |evt: KeyboardEvent| {
            use freya::events::keyboard::Key;
            if focus.is_focused()
                && (evt.key == Key::Delete || evt.key == Key::Backspace)
            {
                // сначала считываем индекс в локальную переменную
                let sel_idx = *sel.read();
                if let Some(idx) = sel_idx {
                    // уже нет borrow от sel.read()
                    let mut v = rects.read().clone();
                    if idx < v.len() {
                        v.remove(idx);
                        rects.set(v);
                    }
                    // теперь можно обновлять sel
                    sel.set(None);
                    platform.invalidate_drawing_area(size.peek().area);
                    platform.request_animation_frame();
                }
            }
        }
    };

    // --- Canvas: перерисовываем при изменении прямоугольников или выделения ---
    let canvas_ref = use_canvas_with_deps(
        &(rectangles(), selected()),
        move |(rects_snapshot, sel_opt)| {
            platform.invalidate_drawing_area(size.peek().area);
            platform.request_animation_frame();
            move |ctx: &mut CanvasRunnerContext<'_>| {
                ctx.canvas.save();
                ctx.canvas.translate((ctx.area.min_x(), ctx.area.min_y()));

                let border = 2.0;
                let w = ctx.area.width();
                let h = ctx.area.height();

                // фон + рамка
                let mut paint = Paint::default();
                paint.set_anti_alias(true)
                     .set_color(Color::WHITE)
                     .set_style(skia_safe::paint::Style::Fill);
                ctx.canvas.draw_rect(
                    Rect::from_xywh(border, border, w - 2.0 * border, h - 2.0 * border),
                    &paint,
                );
                paint.set_color(Color::BLACK)
                     .set_style(skia_safe::paint::Style::Stroke)
                     .set_stroke_width(border);
                ctx.canvas.draw_rect(
                    Rect::from_xywh(border/2.0, border/2.0, w - border, h - border),
                    &paint,
                );

                // прямоугольники
                for (idx, &((x1,y1),(x2,y2))) in rects_snapshot.iter().enumerate() {
                    let rx = x1.min(x2);
                    let ry = y1.min(y2);
                    let rw = (x1 - x2).abs();
                    let rh = (y1 - y2).abs();
                    if Some(idx) == sel_opt {
                        paint.set_color(Color::BLUE);
                        paint.set_stroke_width(3.0);
                    } else {
                        paint.set_color(Color::RED);
                        paint.set_stroke_width(2.0);
                    }
                    paint.set_style(skia_safe::paint::Style::Stroke);
                    ctx.canvas.draw_rect(Rect::from_xywh(rx, ry, rw, rh), &paint);
                }

                ctx.canvas.restore();
            }
        },
    );

    rsx!(
        rect {
            a11y_id: focus.attribute(),
            reference,
            canvas_reference: canvas_ref.attribute(),
            width: "100%", height: "100%",
            padding: "10",

            onclick: onclick.clone(),
            onkeydown: onkey.clone(),

            // прозрачный слой для перехвата событий
            rect { width: "100%", height: "100%", background: "transparent" }
        }
    )
}

// Компонент: Footer
#[component]
pub fn Footer() -> Element {
    let stats = ["Текущий шаг:", "Гармоника", "TM", "Ez", "Лин.", "Полное поле"];
    rsx!(
        rect {
            content: "flex",
            direction: "horizontal",
            height: "30",
            background: "rgb(240,240,240)",
            spacing: "5",
            padding: "5 10",
            cross_align: "center",
            for text in stats {
                rect {
                    padding: "5 10",
                    border: "1 solid #333",
                    label { "{text}" }
                }
            }
        }
    )
}