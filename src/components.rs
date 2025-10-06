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
use std::sync::Arc;
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

import_image!(Add, "../assets/images/add.png", {
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
    draw_source_mode: Signal<bool>,
    draw_probe_mode: Signal<bool>,
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
            // Добавить
            ButtonIcon {
                tooltip: "Add".to_string(),
                icon: rsx!(Add {}),
                onclick: move |_| println!("Add clicked"),
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
                        // Отключаем другие режимы при включении режима прямоугольников
                        if !cur {
                            draw_source_mode.set(false);
                            draw_probe_mode.set(false);
                        }
                    }
                },
            }
            // Остальные кнопки без логики
            ButtonIcon { tooltip: "Ellipse".to_string(), icon: rsx!(Ellipse {}), onclick: |_| {} }
            ButtonIcon { 
                tooltip: "Source".to_string(), 
                icon: rsx!(Source {}), 
                onclick: move |_| {
                    if *active_tab.read() == "geometry" {
                        let cur = *draw_source_mode.read();
                        draw_source_mode.set(!cur);
                        // Отключаем другие режимы при включении режима источников
                        if !cur {
                            draw_rect_mode.set(false);
                            draw_probe_mode.set(false);
                        }
                    }
                }
            }
            ButtonIcon { 
                tooltip: "Probe".to_string(), 
                icon: rsx!(Probe {}), 
                onclick: move |_| {
                    if *active_tab.read() == "geometry" {
                        let cur = *draw_probe_mode.read();
                        draw_probe_mode.set(!cur);
                        // Отключаем другие режимы при включении режима зондов
                        if !cur {
                            draw_rect_mode.set(false);
                            draw_source_mode.set(false);
                        }
                    }
                }
            }
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
    draw_source_mode: Signal<bool>,
    draw_probe_mode: Signal<bool>,
    first_point: Signal<Option<(f32, f32)>>,
    rectangles: Signal<Arc<Vec<((f32, f32), (f32, f32))>>>,
    sources: Signal<Arc<Vec<(f32, f32)>>>,
    probes: Signal<Arc<Vec<(f32, f32)>>>,
) -> Element {
    let cur = active_tab.read().clone();
    rsx!(
        rect { width: "100%", height: "100%",
            match cur.as_str() {
                "geometry" => rsx!(
                    CanvasDrawArea {
                        draw_rect_mode: draw_rect_mode.clone(),
                        draw_source_mode: draw_source_mode.clone(),
                        draw_probe_mode: draw_probe_mode.clone(),
                        first_point: first_point.clone(),
                        rectangles: rectangles.clone(),
                        sources: sources.clone(),
                        probes: probes.clone(),
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
        rect { content:"flex", direction:"horizontal", width:"100%", height:"100%", background:"rgb(240,240,240)", border:"1 solid #ccc",
            for (label,id) in tabs {
                rect { width:"flex(1)", height:"100%", main_align:"center", cross_align:"center",
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
    draw_source_mode: Signal<bool>,
    draw_probe_mode: Signal<bool>,
    first_point: Signal<Option<(f32, f32)>>,
    rectangles: Signal<Arc<Vec<((f32, f32), (f32, f32))>>>,
    sources: Signal<Arc<Vec<(f32, f32)>>>,
    probes: Signal<Arc<Vec<(f32, f32)>>>,
) -> Element {
    let platform = use_platform();
    let (reference, size) = use_node_signal();

    // Фокус для ловли клавиш
    let focus = use_focus();
    // Выбранный прямоугольник (индекс)
    let selected = use_signal(|| None::<usize>);
    // Выбранный источник (индекс)
    let selected_source = use_signal(|| None::<usize>);
    // Выбранный зонд (индекс)
    let selected_probe = use_signal(|| None::<usize>);

    // onclick: либо рисуем (двумя кликами), либо выбираем прямоугольник/источник/зонд, либо добавляем источник/зонд
    let onclick = {
        let mut sel = selected.clone();
        let mut sel_src = selected_source.clone();
        let mut sel_probe = selected_probe.clone();
        let mut rects = rectangles.clone();
        let mut srcs = sources.clone();
        let mut prbs = probes.clone();
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
                    let mut v = (*rects.read()).as_ref().clone();
                    v.push((p1, (nx, ny)));
                    rects.set(Arc::new(v));
                    first_point.set(None);
                    draw_rect_mode.set(false);

                    platform.invalidate_drawing_area(size.peek().area);
                    platform.request_animation_frame();
                }
            } else if *draw_source_mode.read() {
                // режим рисования источников: один клик
                let mut v = (*srcs.read()).as_ref().clone();
                v.push((nx, ny));
                srcs.set(Arc::new(v));
                draw_source_mode.set(false);

                platform.invalidate_drawing_area(size.peek().area);
                platform.request_animation_frame();
            } else if *draw_probe_mode.read() {
                // режим рисования зондов: один клик
                let mut v = (*prbs.read()).as_ref().clone();
                v.push((nx, ny));
                prbs.set(Arc::new(v));
                draw_probe_mode.set(false);

                platform.invalidate_drawing_area(size.peek().area);
                platform.request_animation_frame();
            } else {
                // режим выбора: ищем попавший прямоугольник, источник или зонд
                // сначала проверяем источники
                let srcs_read = srcs.read();
                let mut found_source = None;
                for (idx, &(sx, sy)) in srcs_read.iter().enumerate() {
                    let dx = nx - sx;
                    let dy = ny - sy;
                    let distance = (dx * dx + dy * dy).sqrt();
                    // радиус для клика по источнику (в нормализованных координатах)
                    let click_radius = 0.02; // примерно 20 пикселей для холста 1000px
                    if distance <= click_radius {
                        found_source = Some(idx);
                        break;
                    }
                }
                
                if found_source.is_some() {
                    // выбрали источник
                    sel_src.set(found_source);
                    sel.set(None); // снимаем выбор с прямоугольника
                    sel_probe.set(None); // снимаем выбор с зонда
                } else {
                    // проверяем зонды
                    let prbs_read = prbs.read();
                    let mut found_probe = None;
                    for (idx, &(px, py)) in prbs_read.iter().enumerate() {
                        let dx = nx - px;
                        let dy = ny - py;
                        let distance = (dx * dx + dy * dy).sqrt();
                        // радиус для клика по зонду (в нормализованных координатах)
                        let click_radius = 0.02; // примерно 20 пикселей для холста 1000px
                        if distance <= click_radius {
                            found_probe = Some(idx);
                            break;
                        }
                    }
                    
                    if found_probe.is_some() {
                        // выбрали зонд
                        sel_probe.set(found_probe);
                        sel.set(None); // снимаем выбор с прямоугольника
                        sel_src.set(None); // снимаем выбор с источника
                    } else {
                        // ищем прямоугольник
                        let rects_read = rects.read();
                        let mut found_rect = None;
                        for (idx, &((x1, y1), (x2, y2))) in rects_read.iter().enumerate() {
                            let minx = x1.min(x2);
                            let maxx = x1.max(x2);
                            let miny = y1.min(y2);
                            let maxy = y1.max(y2);
                            if nx >= minx && nx <= maxx && ny >= miny && ny <= maxy {
                                found_rect = Some(idx);
                                break;
                            }
                        }
                        // устанавливаем выбранный элемент
                        sel.set(found_rect);
                        sel_src.set(None); // снимаем выбор с источника
                        sel_probe.set(None); // снимаем выбор с зонда
                    }
                }
                
                platform.invalidate_drawing_area(size.peek().area);
                platform.request_animation_frame();
            }
        }
    };

    // onkeydown: удаление выбранного по Delete/Backspace (только если фокус на холсте)
    let onkey = {
        let mut rects = rectangles.clone();
        let mut srcs = sources.clone();
        let mut prbs = probes.clone();
        let mut sel = selected.clone();
        let mut sel_src = selected_source.clone();
        let mut sel_probe = selected_probe.clone();
        let focus = focus.clone();
        move |evt: KeyboardEvent| {
            use freya::events::keyboard::Key;
            if focus.is_focused() && (evt.key == Key::Delete || evt.key == Key::Backspace) {
                // проверяем выбранный источник
                let sel_src_idx = *sel_src.read();
                if let Some(idx) = sel_src_idx {
                    let mut v = (*srcs.read()).as_ref().clone();
                    if idx < v.len() {
                        v.remove(idx);
                        srcs.set(Arc::new(v));
                    }
                    sel_src.set(None);
                    platform.invalidate_drawing_area(size.peek().area);
                    platform.request_animation_frame();
                    return;
                }
                
                // проверяем выбранный зонд
                let sel_probe_idx = *sel_probe.read();
                if let Some(idx) = sel_probe_idx {
                    let mut v = (*prbs.read()).as_ref().clone();
                    if idx < v.len() {
                        v.remove(idx);
                        prbs.set(Arc::new(v));
                    }
                    sel_probe.set(None);
                    platform.invalidate_drawing_area(size.peek().area);
                    platform.request_animation_frame();
                    return;
                }
                
                // проверяем выбранный прямоугольник
                let sel_idx = *sel.read();
                if let Some(idx) = sel_idx {
                    let mut v = (*rects.read()).as_ref().clone();
                    if idx < v.len() {
                        v.remove(idx);
                        rects.set(Arc::new(v));
                    }
                    sel.set(None);
                    platform.invalidate_drawing_area(size.peek().area);
                    platform.request_animation_frame();
                }
            }
        }
    };

    // Canvas: перерисовываемся при изменении rectangles(), selected(), sources(), selected_source(), probes() или selected_probe()
    let canvas_ref = use_canvas_with_deps(
        &(rectangles(), selected(), sources(), selected_source(), probes(), selected_probe()),
        move |(rects_snapshot, sel_opt, srcs_snapshot, sel_src_opt, probes_snapshot, sel_probe_opt): (
            Arc<Vec<((f32, f32),(f32,f32))>>,
            Option<usize>,
            Arc<Vec<(f32,f32)>>,
            Option<usize>,
            Arc<Vec<(f32,f32)>>,
            Option<usize>,
        )| {
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

                // рисуем источники (круг с точкой по центру)
                if !srcs_snapshot.is_empty() {
                    let mut paint = Paint::default();
                    paint.set_anti_alias(true);
                    for (idx, &(nx, ny)) in srcs_snapshot.iter().enumerate() {
                        let cx = nx * w;
                        let cy = ny * h;
                        let r = 10.0_f32; // радиус внешнего круга
                        
                        // выбираем цвет в зависимости от того, выбран ли источник
                        let is_selected = Some(idx) == sel_src_opt;
                        let color = if is_selected { Color::BLUE } else { Color::BLACK };
                        let stroke_width = if is_selected { 3.0 } else { 2.0 };
                        
                        // внешний круг (без заполнения)
                        paint.set_style(skia_safe::paint::Style::Stroke);
                        paint.set_stroke_width(stroke_width);
                        paint.set_color(color);
                        ctx.canvas.draw_circle((cx, cy), r, &paint);
                        // внутренняя точка
                        paint.set_style(skia_safe::paint::Style::Fill);
                        paint.set_color(color);
                        ctx.canvas.draw_circle((cx, cy), 3.0, &paint);
                    }
                }

                // рисуем зонды (крест как x.png)
                if !probes_snapshot.is_empty() {
                    let mut paint = Paint::default();
                    paint.set_anti_alias(true);
                    paint.set_style(skia_safe::paint::Style::Stroke);
                    for (idx, &(nx, ny)) in probes_snapshot.iter().enumerate() {
                        let cx = nx * w;
                        let cy = ny * h;
                        let r = 8.0_f32;
                        
                        // выбираем цвет и толщину в зависимости от того, выбран ли зонд
                        let is_selected = Some(idx) == sel_probe_opt;
                        let color = if is_selected { Color::BLUE } else { Color::BLACK };
                        let stroke_width = if is_selected { 3.0 } else { 2.0 };
                        
                        paint.set_stroke_width(stroke_width);
                        paint.set_color(color);
                        
                        // две пересекающиеся линии (крест)
                        ctx.canvas.draw_line((cx - r, cy - r), (cx + r, cy + r), &paint);
                        ctx.canvas.draw_line((cx - r, cy + r), (cx + r, cy - r), &paint);
                    }
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

    // Примечание: для преобразования пикселей в координаты используем те же
    // отступы, что и при построении графика (margin и области подписей).

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

            // диапазоны графика — симметричные относительно нуля
            let x_min = -PI;
            let x_max = PI;
            let y_min = -1.0;
            let y_max = 1.0;

            // строим оси и график; оставляем поля под подписи
            let mut chart = ChartBuilder::on(&backend)
                .margin(10)
                .x_label_area_size(32)
                .y_label_area_size(40)
                .build_cartesian_2d(x_min..x_max, y_min..y_max)
                .unwrap();

            // сетка и подписи; гарантируем подпись нуля и центрирование
            chart
                .configure_mesh()
                .x_desc("x")
                .y_desc("sin(x)")
                .x_labels(9)
                .y_labels(5)
                .x_label_formatter(&|v| {
                    // Явно подписываем ноль
                    if v.abs() < 1e-9 { "0".to_string() } else { format!("{:.2}", v) }
                })
                .draw()
                .unwrap();

            // Границы plot-области вычисляются в обработчике ховера по тем же отступам

            // убрали дополнительные линии по нулю — оси будут только в сетке

            // сам график
            let x_axis = (x_min..x_max).step(0.01);
            chart
                .draw_series(LineSeries::new(
                    x_axis.clone().values().map(|x| (x, x.sin())),
                    &BLUE,
                ))
                .unwrap()
                .label("sin(x)")
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
            // Диапазоны и пиксельные границы plot-области (синхронны с ChartBuilder)
            let x_min = -PI;
            let x_max = PI;
            let y_min = -1.0;
            let y_max = 1.0;

            // Те же параметры, что использованы выше
            let margin = 10.0_f64;
            let x_label_area = 32.0_f64; // снизу
            let y_label_area = 40.0_f64; // слева

            let plot_left = y_label_area + margin;
            let plot_top = margin;
            let plot_right = w - margin;
            let plot_bottom = h - x_label_area - margin;
            let plot_w = (plot_right - plot_left).max(1.0);
            let plot_h = (plot_bottom - plot_top).max(1.0);

            // Показываем координаты только ВНУТРИ plot-области
            let inside = px >= plot_left && px <= plot_right && py >= plot_top && py <= plot_bottom;
            if inside {
                show_overlay = true;

                // Перевод пикселей plot area -> данные графика
                let mut x_data = x_min + ((px - plot_left) / plot_w) * (x_max - x_min);
                let mut y_data = y_max - ((py - plot_top) / plot_h) * (y_max - y_min);

                // Снап к нулю для визуально точного (0,0)
                let eps = 1e-9;
                if x_data.abs() < eps { x_data = 0.0; }
                if y_data.abs() < eps { y_data = 0.0; }

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