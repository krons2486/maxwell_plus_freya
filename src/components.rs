use freya::{
    plot::{
        plotters::{
            chart::ChartBuilder,
            prelude::{IntoDrawingArea, IntoLinspace, PathElement, DiscreteRanged},
            series::LineSeries,
            style::{BLUE, WHITE},
        },
        SkiaBackend,
    },
    prelude::*,
    core::custom_attributes::CanvasRunnerContext,
    hooks::{use_canvas, use_focus, use_canvas_with_deps},
};
use skia_safe::{Color, Paint, Rect};
use std::f64::consts::PI;

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

import_image!(Probe, "../assets/images/x.png", {
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

// AppTheme (основные стили)
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

#[component]
pub fn ButtonBar(
    active_tab: Signal<String>,
    draw_rect_mode: Signal<bool>,
    on_open: EventHandler<MouseEvent>,
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
                onclick: on_open.clone(),
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

#[component]
pub fn TabsContent(
    active_tab: Signal<String>,
    draw_rect_mode: Signal<bool>,
    first_point: Signal<Option<(f32, f32)>>,
    rectangles: Signal<Vec<((f32, f32), (f32, f32))>>,
    //sources: Signal<Vec<(f32, f32)>>,
    //probes: Signal<Vec<(f32, f32)>>,
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
                        //sources: sources.clone(),
                        //probes: probes.clone(),
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
                        SignalsGraph {}
                    }
                ),
                _ => rsx!(rect { width: "100%", height: "100%" }),
            }
        }
    )
}

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
                    if *expanded.read() { "▽ " } else { "▷ " }
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

#[component]
pub fn CanvasDrawArea(
    draw_rect_mode: Signal<bool>,
    first_point: Signal<Option<(f32, f32)>>,
    rectangles: Signal<Vec<((f32, f32), (f32, f32))>>,
) -> Element {
    let platform = use_platform();
    let (reference, size) = use_node_signal();

    // Фокус для ловли клавиш
    let focus = use_focus();
    // Выбранный прямоугольник (индекс)
    let selected = use_signal(|| None::<usize>);

    // onclick: либо рисуем (двумя кликами), либо выбираем прямоугольник
    let onclick = {
        let mut sel = selected.clone();
        let mut rects = rectangles.clone();
        let mut focus = focus.clone(); // <- mutable, чтобы вызвать request_focus()
        move |evt: MouseEvent| {
            // запрашиваем фокус для обработки клавиш
            focus.request_focus();

            // пиксельные координаты клика внутри элемента
            let x_px = evt.element_coordinates.x as f32;
            let y_px = evt.element_coordinates.y as f32;

            let area = size.peek().area;
            let w = area.width();
            let h = area.height();
            if w <= 0.0 || h <= 0.0 {
                return;
            }

            // нормализованные координаты (0..1)
            let nx = x_px / w;
            let ny = y_px / h;

            if *draw_rect_mode.read() {
                // режим рисования прямоугольников: 2 клика
                if first_point.read().is_none() {
                    first_point.set(Some((nx, ny)));
                } else {
                    let p1 = first_point.read().unwrap();
                    // добавляем в сигнал rectangles (нормализованные)
                    let mut v = rects.read().clone();
                    v.push((p1, (nx, ny)));
                    rects.set(v);
                    first_point.set(None);
                    draw_rect_mode.set(false);

                    platform.invalidate_drawing_area(size.peek().area);
                    platform.request_animation_frame();
                }
            } else {
                // режим выбора: ищем попавший прямоугольник
                let rects_snapshot = rects.read().clone(); // работаем с копией, чтобы не держать read при set
                let mut found = None;
                for (idx, &((x1, y1), (x2, y2))) in rects_snapshot.iter().enumerate() {
                    let minx = x1.min(x2);
                    let maxx = x1.max(x2);
                    let miny = y1.min(y2);
                    let maxy = y1.max(y2);
                    if nx >= minx && nx <= maxx && ny >= miny && ny <= maxy {
                        found = Some(idx);
                        break;
                    }
                }
                // теперь безопасно устанавливаем selected
                sel.set(found);
                platform.invalidate_drawing_area(size.peek().area);
                platform.request_animation_frame();
            }
        }
    };

    // onkeydown: удаление выбранного по Delete/Backspace (только если фокус на холсте)
    let onkey = {
        let mut rects = rectangles.clone();
        let mut sel = selected.clone();
        let focus = focus.clone();
        move |evt: KeyboardEvent| {
            use freya::events::keyboard::Key;
            if focus.is_focused() && (evt.key == Key::Delete || evt.key == Key::Backspace) {
                // сначала прочитаем индекс в локальную переменную (избежим одновременного borrow)
                let sel_idx = *sel.read();
                if let Some(idx) = sel_idx {
                    let mut v = rects.read().clone();
                    if idx < v.len() {
                        v.remove(idx);
                        rects.set(v);
                    }
                    sel.set(None);
                    platform.invalidate_drawing_area(size.peek().area);
                    platform.request_animation_frame();
                }
            }
        }
    };

    // Canvas: перерисовываемся при изменении rectangles() или selected()
    let canvas_ref = use_canvas_with_deps(
        &(rectangles(), selected()),
        move |(rects_snapshot, sel_opt): (Vec<((f32, f32),(f32,f32))>, Option<usize>)| {
            platform.invalidate_drawing_area(size.peek().area);
            platform.request_animation_frame();
            move |ctx: &mut CanvasRunnerContext<'_>| {
                ctx.canvas.save();
                ctx.canvas.translate((ctx.area.min_x(), ctx.area.min_y()));

                let border = 2.0;
                let w = ctx.area.width();
                let h = ctx.area.height();

                // фон + внутренняя белая область с учётом рамки
                let mut paint = Paint::default();
                paint.set_anti_alias(true);
                paint.set_color(Color::WHITE);
                paint.set_style(skia_safe::paint::Style::Fill);
                ctx.canvas.draw_rect(
                    Rect::from_xywh(border, border, w - 2.0 * border, h - 2.0 * border),
                    &paint,
                );

                // рамка
                paint.set_color(Color::BLACK);
                paint.set_style(skia_safe::paint::Style::Stroke);
                paint.set_stroke_width(border);
                ctx.canvas.draw_rect(
                    Rect::from_xywh(border / 2.0, border / 2.0, w - border, h - border),
                    &paint,
                );

                // рисуем прямоугольники: нормализованные -> пиксели
                for (idx, &((nx1, ny1), (nx2, ny2))) in rects_snapshot.iter().enumerate() {
                    let rx = nx1.min(nx2) * w;
                    let ry = ny1.min(ny2) * h;
                    let rw = (nx1 - nx2).abs() * w;
                    let rh = (ny1 - ny2).abs() * h;

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
            // связываем focus с нодой, чтобы получать key-события
            a11y_id: focus.attribute(),
            reference,
            canvas_reference: canvas_ref.attribute(),
            width: "100%", height: "100%",
            padding: "10",

            onclick: onclick.clone(),
            onkeydown: onkey.clone(),

            // прозрачный слой чтобы ловить события мыши/клавиш
            rect { width: "100%", height: "100%", background: "transparent" }
        }
    )
}

#[component]
pub fn SignalsGraph() -> Element {
    // Платформа и нода для canvas
    let platform = use_platform();
    let (reference, size) = use_node_signal();

    // Сигнал с пиксельными координатами курсора внутри ноды.
    // (-1.0, -1.0) — означает «скрыть оверлей».
    let cursor = use_signal(|| (-1.0_f64, -1.0_f64));

    // Обработчик движения мыши: сохраняем пиксельные координаты курсора
    let on_move = {
        let mut cursor = cursor.clone();
        let size = size.clone();
        move |evt: MouseEvent| {
            // элементные (локальные) координаты внутри ноды
            let px = evt.element_coordinates.x as f64;
            let py = evt.element_coordinates.y as f64;

            // защитимся — если размер ноды нулевой, игнорируем
            let area = size.peek().area;
            if area.width() <= 0.0 || area.height() <= 0.0 {
                cursor.set((-1.0, -1.0));
                return;
            }

            cursor.set((px, py));
        }
    };

    // Курсор покинул область — скрываем оверлей
    let on_leave = {
        let mut cursor = cursor.clone();
        move |_| {
            cursor.set((-1.0, -1.0));
        }
    };

    // Canvas: рисуем сам график (Plotters/Skia backend)
    let canvas = use_canvas(move || {
        platform.invalidate_drawing_area(size.peek().area);
        platform.request_animation_frame();
        move |ctx| {
            // переводим систему координат в начало ноды
            ctx.canvas.translate((ctx.area.min_x(), ctx.area.min_y()));

            // создаём backend для Plotters
            let backend = SkiaBackend::new(
                ctx.canvas,
                ctx.font_collection,
                ctx.area.size.to_i32().to_tuple(),
            )
            .into_drawing_area();

            // заливаем фон
            backend.fill(&WHITE).ok();

            // диапазоны графика
            let x_min = -PI;
            let x_max = PI;
            let y_min = -1.0;
            let y_max = 1.0;

            // строим оси и график
            let x_axis = (x_min..x_max).step(0.01);
            let y_range = y_min..y_max;

            let mut chart = ChartBuilder::on(&backend)
                .margin(10)
                .x_label_area_size(30)
                .y_label_area_size(30)
                .build_cartesian_2d(x_axis.clone(), y_range.clone())
                .unwrap();

            chart
                .configure_mesh()
                .x_desc("x")
                .y_desc("sin(x)")
                .draw()
                .unwrap();

            chart
                .draw_series(LineSeries::new(
                    x_axis.clone().values().map(|x| (x, x.sin())),
                    &BLUE,
                ))
                .unwrap()
                .label("sin(x)")
                // <<< здесь используем целочисленное смещение 20 (i32)
                .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

            chart.configure_series_labels().draw().unwrap();

            ctx.canvas.restore();
        }
    });

    // Вычисляем текст и позицию оверлея (в пикселях + в координатах графика)
    let (px, py) = *cursor.read();

    // значения по умолчанию — скрыть
    let mut show_overlay = false;
    let mut overlay_x = 0_f32;
    let mut overlay_y = 0_f32;
    let mut coord_text = String::new();

    if px >= 0.0 && py >= 0.0 {
        let area = size.peek().area;
        let w = area.width() as f64;
        let h = area.height() as f64;
        if w > 0.0 && h > 0.0 {
            show_overlay = true;

            // диапазоны (те же, что использовали при рисовании)
            let x_min = -PI;
            let x_max = PI;
            let y_min = -1.0;
            let y_max = 1.0;

            // перевод пикселей -> данные графика
            let x_data = x_min + (px / w) * (x_max - x_min);
            // y: на экране y растёт вниз, а в данных вверх -> инвертируем
            let y_data = y_max - (py / h) * (y_max - y_min);

            coord_text = format!("({:+.3}, {:+.3})", x_data, y_data);

            let text_len = coord_text.len() as f64;
            let overlay_w = (text_len * 7.5 + 12.0).max(40.0); // ширина подсказки
            let overlay_h = 20.0; // высота подсказки

            // позиция оверлея (смещаем немного от курсора чтобы не закрывать)
            let mut ox = px + 12.0;
            let mut oy = py + 12.0;

            // Если вылезает за правую границу — показываем слева от курсора
            if ox + overlay_w > w {
                ox = px - overlay_w - 12.0;
            }
            
            // Если вылезает за нижнюю границу — показываем выше курсора
            if oy + overlay_h > h {
                oy = py - overlay_h - 12.0;
            }

            // Приводим к f32 для rsx
            overlay_x = ox as f32;
            overlay_y = oy as f32;
        }
    }

    rsx!(
        rect {
            reference,
            canvas_reference: canvas.attribute(),
            width: "100%", height: "100%",
            // ловим движение мыши и уход курсора
            onmousemove: on_move.clone(),
            onmouseleave: on_leave.clone(),
            background: "transparent",

            // Сам canvas-слой (прозрачный) — чтобы работало позиционирование поверх
            rect { width: "100%", height: "100%", background: "transparent" }

            // Оверлей с текстом рядом с курсором (отрисовываем только когда надо)
            if show_overlay {
                rect {
                    position: "absolute",
                    offset_x: format!("{:.0}", overlay_x),
                    offset_y: format!("{:.0}", overlay_y),
                    background: "transparent",
                    padding: "4",
                    corner_radius: "4",
                    label { "{coord_text}" }
                }
            }
        }
    )
}

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