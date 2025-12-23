use crate::{
    fdtd::{Fdtd2dTe, Fdtd2dTm},
    functions::{ActiveDialog, Modelling, ObjectType, ProjectObject, ProjectSettings, SidebarItemType},
};
use freya::{
    core::custom_attributes::CanvasRunnerContext,
    hooks::{use_canvas, use_canvas_with_deps, use_focus},
    plot::{
        plotters::{
            chart::ChartBuilder,
            prelude::{DiscreteRanged, IntoDrawingArea, IntoLinspace, PathElement},
            series::LineSeries,
            style::{BLUE, WHITE},
        },
        SkiaBackend,
    },
    prelude::*,
};
use skia_safe::{Color, Paint, Rect};
use std::f64::consts::PI;
use std::sync::Arc;

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

import_image!(Resume, "../assets/images/resume.png", {
    width: "100%",
    height: "100%"
});

/// Тип волны для моделирования
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaveType {
    TE,
    TM,
}

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
    on_open: EventHandler<MouseEvent>,
    on_save: EventHandler<MouseEvent>,
    on_open_project_settings: EventHandler<MouseEvent>,
    on_create_rectangle: EventHandler<MouseEvent>,
    on_create_source: EventHandler<MouseEvent>,
    on_create_probe: EventHandler<MouseEvent>,
    on_start: EventHandler<MouseEvent>,
    on_stop: EventHandler<MouseEvent>,
    on_resume: EventHandler<MouseEvent>,
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
                is_active: None,
            }
            // Сохранить
            ButtonIcon {
                tooltip: "Save".to_string(),
                icon: rsx!(Save {}),
                onclick: on_save.clone(),
                is_active: None,
            }
            // Отменить
            ButtonIcon {
                tooltip: "Undo".to_string(),
                icon: rsx!(Undo {}),
                onclick: move |_| println!("Undo clicked"),
                is_active: None,
            }
            // Повторить
            ButtonIcon {
                tooltip: "Redo".to_string(),
                icon: rsx!(Redo {}),
                onclick: move |_| println!("Redo clicked"),
                is_active: None,
            }
            // Добавить
            ButtonIcon {
                tooltip: "Add".to_string(),
                icon: rsx!(Add {}),
                onclick: on_open_project_settings.clone(),
                is_active: None,
            }
            // Цвет
            ButtonIcon {
                tooltip: "Choose color".to_string(),
                icon: rsx!(RgbColorWheel {}),
                onclick: move |_| println!("Color clicked"),
                is_active: None,
            }
            // Курсор
            ButtonIcon {
                tooltip: "Cursor".to_string(),
                icon: rsx!(Cursor {}),
                onclick: move |_| println!("Cursor clicked"),
                is_active: None,
            }
            // Создание прямоугольника
            ButtonIcon {
                tooltip: "Rectangle".to_string(),
                icon: rsx!(Rectangle {}),
                onclick: on_create_rectangle.clone(),
                is_active: None,
            }
            // Остальные кнопки без логики
            ButtonIcon {
                tooltip: "Ellipse".to_string(),
                icon: rsx!(Ellipse {}),
                onclick: |_| {},
                is_active: None,
            }
            ButtonIcon {
                tooltip: "Source".to_string(),
                icon: rsx!(Source {}),
                onclick: on_create_source.clone(),
                is_active: None,
            }
            ButtonIcon {
                tooltip: "Probe".to_string(),
                icon: rsx!(Probe {}),
                onclick: on_create_probe.clone(),
                is_active: None,
            }
            ButtonIcon {
                tooltip: "Line".to_string(),
                icon: rsx!(Line {}),
                onclick: |_| {},
                is_active: None,
            }
            ButtonIcon {
                tooltip: "Start".to_string(),
                icon: rsx!(Start {}),
                onclick: on_start.clone(),
                is_active: None,
            }
            ButtonIcon {
                tooltip: "Stop".to_string(),
                icon: rsx!(Stop {}),
                onclick: on_stop.clone(),
                is_active: None,
            }
            ButtonIcon {
                tooltip: "Resume".to_string(),
                icon: rsx!(Resume {}),
                onclick: on_resume.clone(),
                is_active: None,
            }
        }
    )
}

#[component]
pub fn ButtonIcon(
    tooltip: String,
    icon: Element,
    onclick: EventHandler<MouseEvent>,
    is_active: Option<bool>,
) -> Element {
    let background_color = if is_active.unwrap_or(false) {
        "rgb(100, 150, 255)" // синий цвет для активной кнопки
    } else {
        "transparent"
    };

    rsx!(
        rect {
            width: "40",
            height: "40",
            margin: "0 5 0 0",
            background: background_color,
            corner_radius: "4",
            onclick: onclick.clone(),
            {icon}
        }
    )
}

#[component]
pub fn TabsContent(
    active_tab: Signal<String>,
    rectangles: Signal<Arc<Vec<((f32, f32), (f32, f32))>>>,
    sources: Signal<Arc<Vec<(f32, f32)>>>,
    probes: Signal<Arc<Vec<(f32, f32)>>>,
    running: Signal<bool>,
    resuming: Signal<bool>,
    step_counter: Signal<usize>,
    sim_te: Signal<Fdtd2dTe>,
    sim_tm: Signal<Fdtd2dTm>,
    wave_type: Signal<WaveType>,
    field_data: Signal<(usize, usize, Vec<f64>)>,
    // Общий сигнал выделения (для синхронизации с боковой панелью)
    sidebar_selection: Signal<Option<SidebarSelection>>,
) -> Element {
    let cur = active_tab.read().clone();
    rsx!(
        rect { width: "100%", height: "100%",
            match cur.as_str() {
                "geometry" => rsx!(
                    CanvasDrawArea {
                        rectangles: rectangles.clone(),
                        sources: sources.clone(),
                        probes: probes.clone(),
                        sidebar_selection: sidebar_selection.clone(),
                    }
                ),
                "field" => rsx!(
                    rect {
                        width: "100%",
                        height: "100%",
                        padding: "10",
                        background: "rgb(200,200,200)",
                            FieldTab {
                                running: running.clone(),
                                resuming: resuming.clone(),
                                step_counter: step_counter.clone(),
                                sim_te: sim_te.clone(),
                                sim_tm: sim_tm.clone(),
                                wave_type: wave_type.clone(),
                                field_data: field_data.clone(),
                            }
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

/// Выбранный элемент в боковой панели
#[derive(Clone, PartialEq, Debug)]
pub struct SidebarSelection {
    pub item_type: SidebarItemType,
    pub index: usize,
}

#[component]
pub fn MySidebar(
    rectangles: Signal<Arc<Vec<((f32, f32), (f32, f32))>>>,
    sources: Signal<Arc<Vec<(f32, f32)>>>,
    probes: Signal<Arc<Vec<(f32, f32)>>>,
    // Счётчики для уникальных номеров объектов
    next_rect_id: Signal<usize>,
    next_source_id: Signal<usize>,
    next_probe_id: Signal<usize>,
    // ID объектов (для отслеживания порядковых номеров)
    rect_ids: Signal<Arc<Vec<usize>>>,
    source_ids: Signal<Arc<Vec<usize>>>,
    probe_ids: Signal<Arc<Vec<usize>>>,
    // Сигнал выделения (общий с холстом)
    selected: Signal<Option<SidebarSelection>>,
) -> Element {
    let mut focus = use_focus();
    
    // Обработчик клавиш для удаления
    let onkeydown = {
        let mut rectangles = rectangles.clone();
        let mut sources = sources.clone();
        let mut probes = probes.clone();
        let mut rect_ids = rect_ids.clone();
        let mut source_ids = source_ids.clone();
        let mut probe_ids = probe_ids.clone();
        let mut selected = selected.clone();
        let focus = focus.clone();
        
        move |evt: KeyboardEvent| {
            use freya::events::keyboard::Key;
            if focus.is_focused() && (evt.key == Key::Delete || evt.key == Key::Backspace) {
                // Сначала извлекаем данные
                let sel_opt = { selected.read().clone() };
                if let Some(sel) = sel_opt {
                    match sel.item_type {
                        SidebarItemType::Rectangle => {
                            let mut rects = (*rectangles.read()).as_ref().clone();
                            let mut ids = (*rect_ids.read()).as_ref().clone();
                            if sel.index < rects.len() {
                                rects.remove(sel.index);
                                ids.remove(sel.index);
                                rectangles.set(Arc::new(rects));
                                rect_ids.set(Arc::new(ids));
                            }
                        }
                        SidebarItemType::Source => {
                            let mut srcs = (*sources.read()).as_ref().clone();
                            let mut ids = (*source_ids.read()).as_ref().clone();
                            if sel.index < srcs.len() {
                                srcs.remove(sel.index);
                                ids.remove(sel.index);
                                sources.set(Arc::new(srcs));
                                source_ids.set(Arc::new(ids));
                            }
                        }
                        SidebarItemType::Probe => {
                            let mut prbs = (*probes.read()).as_ref().clone();
                            let mut ids = (*probe_ids.read()).as_ref().clone();
                            if sel.index < prbs.len() {
                                prbs.remove(sel.index);
                                ids.remove(sel.index);
                                probes.set(Arc::new(prbs));
                                probe_ids.set(Arc::new(ids));
                            }
                        }
                    }
                    selected.set(None);
                }
            }
        }
    };
    
    rsx!(
        rect {
            a11y_id: focus.attribute(),
            width: "100%",
            height: "100%",
            background: "rgb(249, 249, 249)",
            padding: "10",
            onkeydown: onkeydown.clone(),
            onclick: move |_| {
                focus.request_focus();
            },
            DynamicTreeView {
                rectangles: rectangles.clone(),
                sources: sources.clone(),
                probes: probes.clone(),
                rect_ids: rect_ids.clone(),
                source_ids: source_ids.clone(),
                probe_ids: probe_ids.clone(),
                selected: selected.clone(),
            }
        }
    )
}

#[component]
pub fn DynamicTreeView(
    rectangles: Signal<Arc<Vec<((f32, f32), (f32, f32))>>>,
    sources: Signal<Arc<Vec<(f32, f32)>>>,
    probes: Signal<Arc<Vec<(f32, f32)>>>,
    rect_ids: Signal<Arc<Vec<usize>>>,
    source_ids: Signal<Arc<Vec<usize>>>,
    probe_ids: Signal<Arc<Vec<usize>>>,
    selected: Signal<Option<SidebarSelection>>,
) -> Element {
    rsx!(
        rect {
            direction: "vertical",
            width: "100%",
            // Объекты (прямоугольники)
            DynamicTreeCategory {
                title: "Объекты".to_string(),
                item_type: SidebarItemType::Rectangle,
                count: rectangles.read().len(),
                ids: rect_ids.clone(),
                selected: selected.clone(),
            }
            // Источники
            DynamicTreeCategory {
                title: "Источники".to_string(),
                item_type: SidebarItemType::Source,
                count: sources.read().len(),
                ids: source_ids.clone(),
                selected: selected.clone(),
            }
            // Датчики
            DynamicTreeCategory {
                title: "Датчики".to_string(),
                item_type: SidebarItemType::Probe,
                count: probes.read().len(),
                ids: probe_ids.clone(),
                selected: selected.clone(),
            }
        }
    )
}

#[component]
pub fn DynamicTreeCategory(
    title: String,
    item_type: SidebarItemType,
    count: usize,
    ids: Signal<Arc<Vec<usize>>>,
    selected: Signal<Option<SidebarSelection>>,
) -> Element {
    let mut expanded = use_signal(|| true);
    let ids_vec = ids.read().clone();
    
    // Генерируем имена для элементов
    let item_names: Vec<(usize, String)> = (0..count)
        .map(|idx| {
            let id = ids_vec.get(idx).copied().unwrap_or(idx + 1);
            let name = match item_type {
                SidebarItemType::Rectangle => format!("Прямоугольник {}", id),
                SidebarItemType::Source => format!("Источник {}", id),
                SidebarItemType::Probe => format!("Датчик {}", id),
            };
            (idx, name)
        })
        .collect();
    
    rsx!(
        rect {
            direction: "vertical",
            margin: "5 0",
            width: "100%",
            // Заголовок категории
            rect {
                direction: "horizontal",
                padding: "3",
                corner_radius: "3",
                onclick: move |_| {
                    let current = *expanded.read();
                    expanded.set(!current);
                },
                label {
                    font_weight: "bold",
                    if *expanded.read() { "▽ " } else { "▷ " }
                    "{title}"
                }
            }
            // Дочерние элементы
            if *expanded.read() {
                rect {
                    padding: "0 0 0 15",
                    direction: "vertical",
                    width: "100%",
                    for (idx, name) in item_names {
                        DynamicTreeItem {
                            item_type: item_type,
                            index: idx,
                            name: name,
                            selected: selected.clone(),
                        }
                    }
                }
            }
        }
    )
}

#[component]
pub fn DynamicTreeItem(
    item_type: SidebarItemType,
    index: usize,
    name: String,
    selected: Signal<Option<SidebarSelection>>,
) -> Element {
    let is_selected = selected.read().as_ref().map_or(false, |sel| {
        sel.item_type == item_type && sel.index == index
    });
    
    let background = if is_selected {
        "rgb(173, 214, 255)"  // Светло-синий для выделения
    } else {
        "transparent"
    };
    
    let mut selected = selected.clone();
    
    rsx!(
        rect {
            direction: "horizontal",
            padding: "4 8",
            margin: "1 0",
            corner_radius: "3",
            background: background,
            width: "100%",
            onclick: move |_| {
                selected.set(Some(SidebarSelection {
                    item_type,
                    index,
                }));
            },
            label {
                "{name}"
            }
        }
    )
}

#[component]
pub fn TabsBar(active_tab: Signal<String>) -> Element {
    let tabs = [
        ("Геометрия", "geometry"),
        ("Поле", "field"),
        ("Временные сигналы", "signals"),
    ];
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
    rectangles: Signal<Arc<Vec<((f32, f32), (f32, f32))>>>,
    sources: Signal<Arc<Vec<(f32, f32)>>>,
    probes: Signal<Arc<Vec<(f32, f32)>>>,
    // Общий сигнал выделения (синхронизация с боковой панелью)
    sidebar_selection: Signal<Option<SidebarSelection>>,
) -> Element {
    let platform = use_platform();
    let (reference, size) = use_node_signal();

    // Фокус для ловли клавиш
    let focus = use_focus();

    // onclick: выбираем прямоугольник/источник/зонд и обновляем общий сигнал выделения
    let onclick = {
        let mut sidebar_selection = sidebar_selection.clone();
        let mut focus = focus.clone();
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

            // режим выбора: ищем попавший прямоугольник, источник или зонд
            // сначала проверяем источники
            let srcs_read = sources.read();
            let mut found_source = None;
            for (idx, &(sx, sy)) in srcs_read.iter().enumerate() {
                let dx = nx - sx;
                let dy = ny - sy;
                let distance = (dx * dx + dy * dy).sqrt();
                let click_radius = 0.02;
                if distance <= click_radius {
                    found_source = Some(idx);
                    break;
                }
            }

            if let Some(idx) = found_source {
                sidebar_selection.set(Some(SidebarSelection {
                    item_type: SidebarItemType::Source,
                    index: idx,
                }));
            } else {
                // проверяем зонды
                let prbs_read = probes.read();
                let mut found_probe = None;
                for (idx, &(px, py)) in prbs_read.iter().enumerate() {
                    let dx = nx - px;
                    let dy = ny - py;
                    let distance = (dx * dx + dy * dy).sqrt();
                    let click_radius = 0.02;
                    if distance <= click_radius {
                        found_probe = Some(idx);
                        break;
                    }
                }

                if let Some(idx) = found_probe {
                    sidebar_selection.set(Some(SidebarSelection {
                        item_type: SidebarItemType::Probe,
                        index: idx,
                    }));
                } else {
                    // ищем прямоугольник
                    let rects_read = rectangles.read();
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
                    
                    if let Some(idx) = found_rect {
                        sidebar_selection.set(Some(SidebarSelection {
                            item_type: SidebarItemType::Rectangle,
                            index: idx,
                        }));
                    } else {
                        // клик в пустое место - снимаем выделение
                        sidebar_selection.set(None);
                    }
                }
            }

            platform.invalidate_drawing_area(size.peek().area);
            platform.request_animation_frame();
        }
    };

    // onkeydown: удаление выбранного по Delete/Backspace (только если фокус на холсте)
    let onkey = {
        let mut rects = rectangles.clone();
        let mut srcs = sources.clone();
        let mut prbs = probes.clone();
        let mut sidebar_selection = sidebar_selection.clone();
        let focus = focus.clone();
        move |evt: KeyboardEvent| {
            use freya::events::keyboard::Key;
            if focus.is_focused() && (evt.key == Key::Delete || evt.key == Key::Backspace) {
                let sel_opt = sidebar_selection.read().clone();
                if let Some(sel) = sel_opt {
                    match sel.item_type {
                        SidebarItemType::Source => {
                            let mut v = (*srcs.read()).as_ref().clone();
                            if sel.index < v.len() {
                                v.remove(sel.index);
                                srcs.set(Arc::new(v));
                            }
                        }
                        SidebarItemType::Probe => {
                            let mut v = (*prbs.read()).as_ref().clone();
                            if sel.index < v.len() {
                                v.remove(sel.index);
                                prbs.set(Arc::new(v));
                            }
                        }
                        SidebarItemType::Rectangle => {
                            let mut v = (*rects.read()).as_ref().clone();
                            if sel.index < v.len() {
                                v.remove(sel.index);
                                rects.set(Arc::new(v));
                            }
                        }
                    }
                    sidebar_selection.set(None);
                    platform.invalidate_drawing_area(size.peek().area);
                    platform.request_animation_frame();
                }
            }
        }
    };

    // Canvas: перерисовываемся при изменении данных или выделения
    let canvas_ref = use_canvas_with_deps(
        &(
            rectangles(),
            sources(),
            probes(),
            sidebar_selection(),
        ),
        move |(
            rects_snapshot,
            srcs_snapshot,
            probes_snapshot,
            selection_opt,
        ): (
            Arc<Vec<((f32, f32), (f32, f32))>>,
            Arc<Vec<(f32, f32)>>,
            Arc<Vec<(f32, f32)>>,
            Option<SidebarSelection>,
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

                // Определяем выбранные индексы на основе sidebar_selection
                let sel_rect_opt = selection_opt.as_ref().and_then(|s| {
                    if s.item_type == SidebarItemType::Rectangle { Some(s.index) } else { None }
                });
                let sel_src_opt = selection_opt.as_ref().and_then(|s| {
                    if s.item_type == SidebarItemType::Source { Some(s.index) } else { None }
                });
                let sel_probe_opt = selection_opt.as_ref().and_then(|s| {
                    if s.item_type == SidebarItemType::Probe { Some(s.index) } else { None }
                });

                // рисуем прямоугольники: нормализованные -> пиксели
                for (idx, &((nx1, ny1), (nx2, ny2))) in rects_snapshot.iter().enumerate() {
                    let rx = nx1.min(nx2) * w;
                    let ry = ny1.min(ny2) * h;
                    let rw = (nx1 - nx2).abs() * w;
                    let rh = (ny1 - ny2).abs() * h;

                    let is_selected = Some(idx) == sel_rect_opt;
                    
                    // Сначала рисуем заливку
                    if is_selected {
                        // Полупрозрачный синий для выбранного прямоугольника
                        paint.set_color(Color::from_argb(128, 0, 0, 255));
                    } else {
                        // Полупрозрачный красный для невыбранного прямоугольника
                        paint.set_color(Color::from_argb(128, 255, 0, 0));
                    }
                    paint.set_style(skia_safe::paint::Style::Fill);
                    ctx.canvas
                        .draw_rect(Rect::from_xywh(rx, ry, rw, rh), &paint);
                    
                    // Затем рисуем обводку
                    if is_selected {
                        paint.set_color(Color::BLUE);
                        paint.set_stroke_width(3.0);
                    } else {
                        paint.set_color(Color::RED);
                        paint.set_stroke_width(2.0);
                    }
                    paint.set_style(skia_safe::paint::Style::Stroke);
                    ctx.canvas
                        .draw_rect(Rect::from_xywh(rx, ry, rw, rh), &paint);
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
                        let color = if is_selected {
                            Color::BLUE
                        } else {
                            Color::BLACK
                        };
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
                        let color = if is_selected {
                            Color::BLUE
                        } else {
                            Color::BLACK
                        };
                        let stroke_width = if is_selected { 3.0 } else { 2.0 };

                        paint.set_stroke_width(stroke_width);
                        paint.set_color(color);

                        // две пересекающиеся линии (крест)
                        ctx.canvas
                            .draw_line((cx - r, cy - r), (cx + r, cy + r), &paint);
                        ctx.canvas
                            .draw_line((cx - r, cy + r), (cx + r, cy - r), &paint);
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

fn field_color(value: f64, vmin: f64, vmax: f64) -> Color {
    // Нормализуем значение в диапазон [-1, 1], где 0 соответствует середине диапазона
    let mid = (vmin + vmax) / 2.0;
    let range = (vmax - vmin) / 2.0;
    
    let normalized = if range.abs() < 1e-9 {
        0.0
    } else {
        (value - mid) / range
    };
    
    // Ограничиваем значение
    let v = normalized.clamp(-1.0, 1.0);
    
    // Порог для белого цвета (близко к нулю)
    let white_threshold = 0.05;
    
    if v.abs() < white_threshold {
        // Белый цвет для значений близких к нулю
        Color::from_argb(255, 255, 255, 255)
    } else if v < 0.0 {
        // Отрицательные значения: градиент от белого к синему
        // v от -white_threshold до -1.0
        let intensity = ((v.abs() - white_threshold) / (1.0 - white_threshold)).min(1.0);
        let r = (255.0 * (1.0 - intensity)) as u8;
        let g = (255.0 * (1.0 - intensity)) as u8;
        let b = 255;
        Color::from_argb(255, r, g, b)
    } else {
        // Положительные значения: градиент от белого к красному
        // v от white_threshold до 1.0
        let intensity = ((v - white_threshold) / (1.0 - white_threshold)).min(1.0);
        let r = 255;
        let g = (255.0 * (1.0 - intensity)) as u8;
        let b = (255.0 * (1.0 - intensity)) as u8;
        Color::from_argb(255, r, g, b)
    }
}

#[component]
pub fn FieldTab(
    running: Signal<bool>,
    resuming: Signal<bool>,
    step_counter: Signal<usize>,
    sim_te: Signal<Fdtd2dTe>,
    sim_tm: Signal<Fdtd2dTm>,
    wave_type: Signal<WaveType>,
    field_data: Signal<(usize, usize, Vec<f64>)>,
) -> Element {
    let platform = use_platform();
    let (reference, size) = use_node_signal();

    // Асинхронный цикл симуляции
    let _ = use_effect({
        let sim_te = sim_te.clone();
        let sim_tm = sim_tm.clone();
        let wave_type = wave_type.clone();
        let running = running.clone();
        let resuming = resuming.clone();
        let step_counter = step_counter.clone();
        let field_data = field_data.clone();
        
        move || {
            let running_val = *running.read();
            let resuming_val = *resuming.read();
            
            if running_val {
                // Запускаем асинхронную задачу для выполнения симуляции
                spawn({
                    let mut sim_te = sim_te.clone();
                    let mut sim_tm = sim_tm.clone();
                    let wave_type = wave_type.clone();
                    let mut running = running.clone();
                    let mut step_counter = step_counter.clone();
                    let mut field_data = field_data.clone();
                    
                    async move {
                        // Читаем тип волны в момент запуска симуляции
                        let wave = *wave_type.read();
                        match wave {
                            WaveType::TE => {
                                // Если это НЕ Resume (т.е. Start), сбрасываем TE‑симуляцию
                                if !resuming_val {
                                    {
                                        let mut sim_mut = sim_te.write();
                                        sim_mut.reset();
                                    }

                                    // Показываем начальное состояние (шаг 0)
                                    {
                                        let sim_read = sim_te.read();
                                        let (sx, sy) = sim_read.size();
                                        let data = sim_read.ey().to_vec();
                                        field_data.set((sx, sy, data));
                                    }
                                    step_counter.set(0);

                                    async_std::task::sleep(std::time::Duration::from_millis(100)).await;
                                }

                                loop {
                                    if !*running.read() {
                                        break;
                                    }

                                    let finished = {
                                        let mut sim_mut = sim_te.write();
                                        sim_mut.step()
                                    };

                                    let current_step = sim_te.read().step_index();

                                    if current_step % 2 == 0 {
                                        {
                                            let sim_read = sim_te.read();
                                            let (sx, sy) = sim_read.size();
                                            let data = sim_read.ey().to_vec();
                                            field_data.set((sx, sy, data));
                                        }
                                        step_counter.set(current_step);

                                        async_std::task::sleep(std::time::Duration::from_millis(20)).await;
                                    }

                                    if finished {
                                        {
                                            let sim_read = sim_te.read();
                                            let (sx, sy) = sim_read.size();
                                            let data = sim_read.ey().to_vec();
                                            field_data.set((sx, sy, data));
                                        }
                                        step_counter.set(current_step);
                                        running.set(false);
                                        break;
                                    }
                                }
                            }
                            WaveType::TM => {
                                // Если это НЕ Resume (т.е. Start), сбрасываем TM‑симуляцию
                                if !resuming_val {
                                    {
                                        let mut sim_mut = sim_tm.write();
                                        sim_mut.reset();
                                    }

                                    // Показываем начальное состояние (шаг 0)
                                    {
                                        let sim_read = sim_tm.read();
                                        let (sx, sy) = sim_read.size();
                                        let data = sim_read.ez().to_vec();
                                        field_data.set((sx, sy, data));
                                    }
                                    step_counter.set(0);

                                    async_std::task::sleep(std::time::Duration::from_millis(100)).await;
                                }

                                loop {
                                    if !*running.read() {
                                        break;
                                    }

                                    let finished = {
                                        let mut sim_mut = sim_tm.write();
                                        sim_mut.step()
                                    };

                                    let current_step = sim_tm.read().step_index();

                                    if current_step % 2 == 0 {
                                        {
                                            let sim_read = sim_tm.read();
                                            let (sx, sy) = sim_read.size();
                                            let data = sim_read.ez().to_vec();
                                            field_data.set((sx, sy, data));
                                        }
                                        step_counter.set(current_step);

                                        async_std::task::sleep(std::time::Duration::from_millis(20)).await;
                                    }

                                    if finished {
                                        {
                                            let sim_read = sim_tm.read();
                                            let (sx, sy) = sim_read.size();
                                            let data = sim_read.ez().to_vec();
                                            field_data.set((sx, sy, data));
                                        }
                                        step_counter.set(current_step);
                                        running.set(false);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                });
            }
        }
    });

    // Canvas только для отрисовки текущего состояния поля
    let canvas = use_canvas_with_deps(
        &field_data(),
        {
            let platform = platform.clone();
            let size_init = size.clone();
            
            move |(sx, sy, data): (usize, usize, Vec<f64>)| {
                platform.invalidate_drawing_area(size_init.peek().area);
                platform.request_animation_frame();

                move |ctx: &mut CanvasRunnerContext<'_>| {
                    let (w, h) = (ctx.area.width(), ctx.area.height());

                    ctx.canvas.save();
                    ctx.canvas.translate((ctx.area.min_x(), ctx.area.min_y()));
                    
                    // Стираем предыдущий кадр - заливаем белым фоном
                    let mut bg_paint = Paint::default();
                    bg_paint.set_anti_alias(false);
                    bg_paint.set_style(skia_safe::paint::Style::Fill);
                    bg_paint.set_color(Color::from_argb(255, 255, 255, 255));
                    ctx.canvas.draw_rect(Rect::from_xywh(0.0, 0.0, w, h), &bg_paint);

                    // Рисуем поле
                    if sx > 0 && sy > 0 && w > 0.0 && h > 0.0 {
                        let cell_w = w / sx as f32;
                        let cell_h = h / sy as f32;

                        // Автоматическое масштабирование: находим максимальное абсолютное значение
                        let max_abs = data.iter().fold(0.0_f64, |acc, &v| acc.max(v.abs()));
                        // Используем симметричный диапазон, минимум 0.01 чтобы не делить на 0
                        let range = max_abs.max(0.01);

                        let mut paint = Paint::default();
                        paint.set_anti_alias(false);
                        paint.set_style(skia_safe::paint::Style::Fill);

                        // Отрисовываем поле по пикселям
                        for x in 0..sx {
                            for y in 0..sy {
                                let idx = x * sy + y;
                                let color = field_color(data[idx], -range, range);
                                paint.set_color(color);
                                let rx = x as f32 * cell_w;
                                let ry = y as f32 * cell_h;
                                ctx.canvas
                                    .draw_rect(Rect::from_xywh(rx, ry, cell_w, cell_h), &paint);
                            }
                        }
                    }

                    ctx.canvas.restore();
                }
            }
        },
    );

    rsx!(
        rect {
            reference,
            canvas_reference: canvas.attribute(),
            width: "100%",
            height: "100%",
            background: "white",
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
                    if v.abs() < 1e-9 {
                        "0".to_string()
                    } else {
                        format!("{:.2}", v)
                    }
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
                if x_data.abs() < eps {
                    x_data = 0.0;
                }
                if y_data.abs() < eps {
                    y_data = 0.0;
                }

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
pub fn Footer(step_counter: Signal<usize>, wave_type: Signal<WaveType>) -> Element {
    let step_text = format!("{}", *step_counter.read());
    let current_wave = *wave_type.read();
    let wave_label = match current_wave {
        WaveType::TE => "TE",
        WaveType::TM => "TM",
    };

    rsx!(
        rect {
            content: "flex",
            direction: "horizontal",
            height: "30",
            background: "rgb(240,240,240)",
            spacing: "5",
            padding: "5 10",
            cross_align: "center",
            rect {
                padding: "5 10",
                border: "1 solid #333",
                label { "Текущий шаг: {step_text}" }
            }
            rect {
                padding: "5 10",
                border: "1 solid #333",
                label { "Гармоника" }
            }
            // Кнопка переключения TE/TM - простой клик меняет тип
            rect {
                padding: "5 10",
                border: "1 solid #333",
                background: "rgb(255,255,255)",
                corner_radius: "2",
                onclick: move |_| {
                    // Переключаем между TE и TM при каждом клике
                    let current = *wave_type.read();
                    match current {
                        WaveType::TE => wave_type.set(WaveType::TM),
                        WaveType::TM => wave_type.set(WaveType::TE),
                    }
                },
                label { "{wave_label}" }
            }
            rect {
                padding: "5 10",
                border: "1 solid #333",
                label { "Ez" }
            }
            rect {
                padding: "5 10",
                border: "1 solid #333",
                label { "Лин." }
            }
            rect {
                padding: "5 10",
                border: "1 solid #333",
                label { "Полное поле" }
            }
        }
    )
}

#[component]
pub fn ProjectSettingsApp(
    project_settings: Signal<ProjectSettings>,
    on_apply: EventHandler<ProjectSettings>,
    on_close: EventHandler<()>,
) -> Element {
    let mut description = use_signal(|| project_settings.read().description.clone());
    let mut sizex = use_signal(|| project_settings.read().sizex);
    let mut sizey = use_signal(|| project_settings.read().sizey);
    let mut dx = use_signal(|| project_settings.read().dx);
    let mut dy = use_signal(|| project_settings.read().dy);
    let mut maxtime = use_signal(|| project_settings.read().maxtime);
    let objects = use_signal(|| project_settings.read().objects.clone());

    let mut selected_object_type = use_signal(|| ObjectType::Rectangle);
    let mut new_object_x1 = use_signal(|| String::new());
    let mut new_object_y1 = use_signal(|| String::new());
    let mut new_object_x2 = use_signal(|| String::new());
    let mut new_object_y2 = use_signal(|| String::new());

    let handle_apply = {
        let description = description.clone();
        let sizex = sizex.clone();
        let sizey = sizey.clone();
        let dx = dx.clone();
        let dy = dy.clone();
        let maxtime = maxtime.clone();
        let objects = objects.clone();
        let on_apply = on_apply.clone();
        let on_close = on_close.clone();
        move |_| {
            let settings = ProjectSettings {
                description: description.read().clone(),
                sizex: *sizex.read(),
                sizey: *sizey.read(),
                dx: *dx.read(),
                dy: *dy.read(),
                maxtime: *maxtime.read(),
                objects: objects.read().clone(),
            };
            on_apply.call(settings);
            // Закрываем окно после применения настроек
            on_close.call(());
        }
    };

    let handle_cancel = {
        let on_close = on_close.clone();
        move |_| {
            on_close.call(());
        }
    };

    let handle_add_object = {
        let mut objects = objects.clone();
        let selected_object_type = selected_object_type.clone();
        let mut new_object_x1 = new_object_x1.clone();
        let mut new_object_y1 = new_object_y1.clone();
        let mut new_object_x2 = new_object_x2.clone();
        let mut new_object_y2 = new_object_y2.clone();
        move |_| {
            let x1 = new_object_x1.read().parse::<f32>().unwrap_or(0.0);
            let y1 = new_object_y1.read().parse::<f32>().unwrap_or(0.0);
            let object_type = *selected_object_type.read();

            let mut current_objects = objects.read().clone();
            let mut new_object = ProjectObject {
                object_type: object_type,
                x1,
                y1,
                x2: None,
                y2: None,
                eps: None,
                mu: None,
            };

            match object_type {
                ObjectType::Rectangle => {
                    let x2 = new_object_x2.read().parse::<f32>().unwrap_or(0.0);
                    let y2 = new_object_y2.read().parse::<f32>().unwrap_or(0.0);
                    new_object.x2 = Some(x2);
                    new_object.y2 = Some(y2);
                }
                _ => {}
            }

            current_objects.push(new_object);
            objects.set(current_objects);

            // Очищаем поля ввода
            new_object_x1.set(String::new());
            new_object_y1.set(String::new());
            new_object_x2.set(String::new());
            new_object_y2.set(String::new());
        }
    };

    rsx!(
        AppTheme {
            rect {
                width: "100%",
                height: "100%",
                direction: "vertical",
                padding: "20",

                // Заголовок
                rect {
                    height: "40",
                    main_align: "center",
                    cross_align: "center",
                    label {
                        font_size: "18",
                        font_weight: "bold",
                        "Настройки проекта"
                    }
                }

                // Параметры проекта
                rect {
                    direction: "vertical",
                    spacing: "10",
                    label {
                        font_size: "14",
                        font_weight: "bold",
                        "Параметры проекта:"
                    }

                    // Описание проекта
                    rect {
                        direction: "horizontal",
                        spacing: "10",
                        cross_align: "center",
                        label {
                            width: "150",
                            "Описание проекта:"
                        }
                        Input {
                            value: description.read().clone(),
                            onchange: move |value: String| description.set(value),
                            placeholder: "Введите описание проекта",
                            width: "400",
                        }
                    }
                }

                // Параметры моделирования
                rect {
                    direction: "vertical",
                    spacing: "10",
                    label {
                        font_size: "14",
                        font_weight: "bold",
                        "Параметры моделирования:"
                    }

                    // Размер области моделирования
                    rect {
                        direction: "horizontal",
                        spacing: "10",
                        cross_align: "center",
                        label {
                            width: "150",
                            "Размер области (м):"
                        }
                        Input {
                            value: sizex.read().to_string(),
                            onchange: move |value: String| {
                                if let Ok(val) = value.parse::<f32>() {
                                    sizex.set(val);
                                }
                            },
                            placeholder: "sizex",
                            width: "100",
                        }
                        Input {
                            value: sizey.read().to_string(),
                            onchange: move |value: String| {
                                if let Ok(val) = value.parse::<f32>() {
                                    sizey.set(val);
                                }
                            },
                            placeholder: "sizey",
                            width: "100",
                        }
                    }

                    // Размер ячейки
                    rect {
                        direction: "horizontal",
                        spacing: "10",
                        cross_align: "center",
                        label {
                            width: "150",
                            "Размер ячейки (м):"
                        }
                        Input {
                            value: dx.read().to_string(),
                            onchange: move |value: String| {
                                if let Ok(val) = value.parse::<f32>() {
                                    dx.set(val);
                                }
                            },
                            placeholder: "dx",
                            width: "100",
                        }
                        Input {
                            value: dy.read().to_string(),
                            onchange: move |value: String| {
                                if let Ok(val) = value.parse::<f32>() {
                                    dy.set(val);
                                }
                            },
                            placeholder: "dy",
                            width: "100",
                        }
                    }

                    // Время моделирования
                    rect {
                        direction: "horizontal",
                        spacing: "10",
                        cross_align: "center",
                        label {
                            width: "150",
                            "Время моделирования (с):"
                        }
                        Input {
                            value: maxtime.read().to_string(),
                            onchange: move |value: String| {
                                if let Ok(val) = value.parse::<f32>() {
                                    maxtime.set(val);
                                }
                            },
                            placeholder: "maxtime",
                            width: "100",
                        }
                    }
                }

                // Новый объект
                rect {
                    direction: "vertical",
                    spacing: "10",
                    label {
                        font_size: "14",
                        font_weight: "bold",
                        "Новый объект:"
                    }

                    // Выбор типа объекта
                    rect {
                        direction: "horizontal",
                        spacing: "10",
                        cross_align: "center",
                        label {
                            width: "150",
                            "Тип объекта:"
                        }
                        rect {
                            width: "200",
                            height: "30",
                            background: "rgb(240, 240, 240)",
                            corner_radius: "4",
                            border: "1 solid #ccc",
                            main_align: "center",
                            cross_align: "center",
                            onclick: move |_| {
                                // Простое переключение между типами
                                let current = *selected_object_type.read();
                                let new_type = match current {
                                    ObjectType::Rectangle => ObjectType::Source,
                                    ObjectType::Source => ObjectType::Probe,
                                    ObjectType::Probe => ObjectType::Rectangle,
                                };
                                selected_object_type.set(new_type);
                            },
                            label {
                                match *selected_object_type.read() {
                                    ObjectType::Rectangle => "Прямоугольник",
                                    ObjectType::Source => "Источник",
                                    ObjectType::Probe => "Зонд",
                                }
                            }
                        }
                    }

                    // Координаты объекта
                    rect {
                        direction: "horizontal",
                        spacing: "10",
                        cross_align: "center",
                        label {
                            width: "150",
                            "Координаты:"
                        }
                        Input {
                            value: new_object_x1.read().clone(),
                            onchange: move |value: String| new_object_x1.set(value),
                            placeholder: "x1",
                            width: "80",
                        }
                        Input {
                            value: new_object_y1.read().clone(),
                            onchange: move |value: String| new_object_y1.set(value),
                            placeholder: "y1",
                            width: "80",
                        }
                        if *selected_object_type.read() == ObjectType::Rectangle {
                            Input {
                                value: new_object_x2.read().clone(),
                                onchange: move |value: String| new_object_x2.set(value),
                                placeholder: "x2",
                                width: "80",
                            }
                            Input {
                                value: new_object_y2.read().clone(),
                                onchange: move |value: String| new_object_y2.set(value),
                                placeholder: "y2",
                                width: "80",
                            }
                        }
                    }

                    // Кнопка добавления объекта
                    rect {
                        direction: "horizontal",
                        spacing: "10",
                        cross_align: "center",
                        rect { width: "150" } // Отступ
                        Button {
                            onclick: handle_add_object,
                            label {
                                "Добавить объект"
                            }
                        }
                    }
                }

                // Список объектов
                rect {
                    direction: "vertical",
                    spacing: "5",
                    height: "100",
                    label {
                        font_size: "14",
                        font_weight: "bold",
                        "Объекты проекта:"
                    }
                    ScrollView {
                        height: "80",
                        rect {
                            direction: "vertical",
                            spacing: "2",
                            for obj in objects.read().iter() {
                                rect {
                                    direction: "horizontal",
                                    spacing: "10",
                                    padding: "5",
                                    background: "rgb(240, 240, 240)",
                                    corner_radius: "4",
                                    label {
                                        width: "100",
                                        match obj.object_type {
                                            ObjectType::Rectangle => "Прямоугольник",
                                            ObjectType::Source => "Источник",
                                            ObjectType::Probe => "Зонд",
                                        }
                                    }
                                    label {
                                        match obj.object_type {
                                            ObjectType::Rectangle => {
                                                if let (Some(x2), Some(y2)) = (obj.x2, obj.y2) {
                                                    format!("({:.2}, {:.2}) - ({:.2}, {:.2})", obj.x1, obj.y1, x2, y2)
                                                } else {
                                                    format!("({:.2}, {:.2})", obj.x1, obj.y1)
                                                }
                                            }
                                            _ => format!("({:.2}, {:.2})", obj.x1, obj.y1),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Кнопки управления
                rect {
                    direction: "horizontal",
                    spacing: "10",
                    main_align: "center",
                    margin: "20 0 0 0",
                    Button {
                        onclick: handle_apply,
                        label {
                            "Задать"
                        }
                    }
                    Button {
                        onclick: handle_cancel,
                        label {
                            "Отмена"
                        }
                    }
                }
            }
        }
    )
}

// ==================== МОДАЛЬНЫЕ ДИАЛОГИ ====================

/// Оверлей для модального диалога
#[component]
pub fn ModalOverlay(
    active_dialog: Signal<ActiveDialog>,
    children: Element,
) -> Element {
    let is_visible = *active_dialog.read() != ActiveDialog::None;
    
    if !is_visible {
        return rsx!(rect {});
    }
    
    rsx!(
        rect {
            position: "absolute",
            width: "100%",
            height: "100%",
            background: "rgb(0, 0, 0, 128)",
            main_align: "center",
            cross_align: "center",
            {children}
        }
    )
}

/// Диалог настроек проекта (в памяти)
#[component]
pub fn ProjectSettingsDialog(
    active_dialog: Signal<ActiveDialog>,
    modelling: Signal<Option<Modelling>>,
) -> Element {
    // Параметры рабочей области
    let current_modelling = modelling.read().clone().unwrap_or(Modelling {
        sizex: 1.0,
        sizey: 1.0,
        dx: 0.01,
        dy: 0.01,
        maxtime: 1.0,
    });
    
    let mut sizex = use_signal(|| current_modelling.sizex);
    let mut sizey = use_signal(|| current_modelling.sizey);
    let mut dx = use_signal(|| current_modelling.dx);
    let mut dy = use_signal(|| current_modelling.dy);
    let mut maxtime = use_signal(|| current_modelling.maxtime);
    
    let handle_apply = {
        let mut modelling = modelling.clone();
        let mut active_dialog = active_dialog.clone();
        let sizex = sizex.clone();
        let sizey = sizey.clone();
        let dx = dx.clone();
        let dy = dy.clone();
        let maxtime = maxtime.clone();
        move |_| {
            modelling.set(Some(Modelling {
                sizex: *sizex.read(),
                sizey: *sizey.read(),
                dx: *dx.read(),
                dy: *dy.read(),
                maxtime: *maxtime.read(),
            }));
            active_dialog.set(ActiveDialog::None);
        }
    };
    
    let handle_cancel = {
        let mut active_dialog = active_dialog.clone();
        move |_| {
            active_dialog.set(ActiveDialog::None);
        }
    };
    
    rsx!(
        rect {
            width: "500",
            height: "350",
            background: "rgb(240, 240, 240)",
            corner_radius: "8",
            padding: "20",
            direction: "vertical",
            
            // Заголовок
            rect {
                height: "40",
                main_align: "center",
                cross_align: "center",
                label { font_size: "18", font_weight: "bold", "Настройки проекта" }
            }
            
            // Параметры
            rect {
                direction: "vertical",
                spacing: "10",
                margin: "10 0",
                
                label { font_size: "14", font_weight: "bold", "Параметры рабочей области (м):" }
                
                rect {
                    direction: "horizontal",
                    spacing: "10",
                    cross_align: "center",
                    label { width: "180", "Размер области (sizex, sizey):" }
                    Input { value: sizex.read().to_string(), onchange: move |v: String| if let Ok(val)=v.parse::<f32>(){ sizex.set(val); }, placeholder: "sizex", width: "80" }
                    Input { value: sizey.read().to_string(), onchange: move |v: String| if let Ok(val)=v.parse::<f32>(){ sizey.set(val); }, placeholder: "sizey", width: "80" }
                }
                
                rect {
                    direction: "horizontal",
                    spacing: "10",
                    cross_align: "center",
                    label { width: "180", "Размер ячейки (dx, dy):" }
                    Input { value: dx.read().to_string(), onchange: move |v: String| if let Ok(val)=v.parse::<f32>(){ dx.set(val); }, placeholder: "dx", width: "80" }
                    Input { value: dy.read().to_string(), onchange: move |v: String| if let Ok(val)=v.parse::<f32>(){ dy.set(val); }, placeholder: "dy", width: "80" }
                }
                
                rect {
                    direction: "horizontal",
                    spacing: "10",
                    cross_align: "center",
                    label { width: "180", "Время моделирования (s):" }
                    Input { value: maxtime.read().to_string(), onchange: move |v: String| if let Ok(val)=v.parse::<f32>(){ maxtime.set(val); }, placeholder: "maxtime", width: "80" }
                }
            }
            
            // Кнопки
            rect {
                direction: "horizontal",
                spacing: "10",
                main_align: "center",
                margin: "20 0 0 0",
                rect { 
                    width: "110", height: "36", 
                    background: "rgb(0, 150, 0)", 
                    corner_radius: "4", 
                    main_align: "center", 
                    cross_align: "center", 
                    onclick: handle_apply,
                    label { color: "white", "Применить" }
                }
                rect { 
                    width: "110", height: "36", 
                    background: "rgb(255, 255, 255)", 
                    corner_radius: "4", 
                    border: "1 solid #ccc",
                    main_align: "center", 
                    cross_align: "center", 
                    onclick: handle_cancel,
                    label { "Отмена" }
                }
            }
        }
    )
}

/// Диалог создания прямоугольника (в памяти)
#[component]
pub fn RectangleDialog(
    active_dialog: Signal<ActiveDialog>,
    modelling: Signal<Option<Modelling>>,
    rectangles: Signal<Arc<Vec<((f32, f32), (f32, f32))>>>,
    next_rect_id: Signal<usize>,
    rect_ids: Signal<Arc<Vec<usize>>>,
) -> Element {
    let mut x1 = use_signal(|| String::new());
    let mut y1 = use_signal(|| String::new());
    let mut x2 = use_signal(|| String::new());
    let mut y2 = use_signal(|| String::new());
    let mut eps = use_signal(|| String::from("4.0"));
    let mut mu = use_signal(|| String::from("1.0"));
    let error_msg = use_signal(|| String::new());
    
    let handle_create = {
        let mut rectangles = rectangles.clone();
        let mut next_rect_id = next_rect_id.clone();
        let mut rect_ids = rect_ids.clone();
        let mut active_dialog = active_dialog.clone();
        let modelling = modelling.clone();
        let x1 = x1.clone();
        let y1 = y1.clone();
        let x2 = x2.clone();
        let y2 = y2.clone();
        let eps = eps.clone();
        let mu = mu.clone();
        let mut error_msg = error_msg.clone();
        
        move |_| {
            let m = match modelling.read().clone() {
                Some(m) => m,
                None => {
                    error_msg.set("Сначала настройте параметры проекта".to_string());
                    return;
                }
            };
            
            // Парсим значения
            let x1_val = match x1.read().trim().parse::<f32>() {
                Ok(v) => v,
                Err(_) => { error_msg.set("X1 должно быть числом".to_string()); return; }
            };
            let y1_val = match y1.read().trim().parse::<f32>() {
                Ok(v) => v,
                Err(_) => { error_msg.set("Y1 должно быть числом".to_string()); return; }
            };
            let x2_val = match x2.read().trim().parse::<f32>() {
                Ok(v) => v,
                Err(_) => { error_msg.set("X2 должно быть числом".to_string()); return; }
            };
            let y2_val = match y2.read().trim().parse::<f32>() {
                Ok(v) => v,
                Err(_) => { error_msg.set("Y2 должно быть числом".to_string()); return; }
            };
            let _eps_val = match eps.read().trim().parse::<f32>() {
                Ok(v) => v,
                Err(_) => { error_msg.set("ε должно быть числом".to_string()); return; }
            };
            let _mu_val = match mu.read().trim().parse::<f32>() {
                Ok(v) => v,
                Err(_) => { error_msg.set("μ должно быть числом".to_string()); return; }
            };
            
            if x2_val <= x1_val || y2_val <= y1_val {
                error_msg.set("X2 должен быть > X1, Y2 > Y1".to_string());
                return;
            }
            
            if _eps_val <= 0.0 || _mu_val <= 0.0 {
                error_msg.set("ε и μ должны быть положительными".to_string());
                return;
            }
            
            // Нормализуем координаты
            let nx1 = x1_val / m.sizex;
            let ny1 = y1_val / m.sizey;
            let nx2 = x2_val / m.sizex;
            let ny2 = y2_val / m.sizey;
            
            // Добавляем в список (нормализованные координаты)
            let mut rects = (*rectangles.read()).as_ref().clone();
            rects.push(((nx1, ny1), (nx2, ny2)));
            rectangles.set(Arc::new(rects));
            
            // Обновляем ID
            let mut ids = (*rect_ids.read()).as_ref().clone();
            let new_id = *next_rect_id.read();
            ids.push(new_id);
            rect_ids.set(Arc::new(ids));
            next_rect_id.set(new_id + 1);
            
            active_dialog.set(ActiveDialog::None);
        }
    };
    
    let handle_cancel = {
        let mut active_dialog = active_dialog.clone();
        move |_| {
            active_dialog.set(ActiveDialog::None);
        }
    };
    
    let err = error_msg.read().clone();
    
    rsx!(
        rect {
            width: "400",
            height: "320",
            background: "rgb(240, 240, 240)",
            corner_radius: "8",
            padding: "20",
            direction: "vertical",
            
            rect {
                height: "36",
                main_align: "center",
                cross_align: "center",
                label { font_size: "18", font_weight: "bold", "Создание прямоугольника" }
            }
            
            rect {
                direction: "vertical",
                spacing: "8",
                margin: "10 0",
                
                rect {
                    direction: "horizontal",
                    spacing: "10",
                    cross_align: "center",
                    label { width: "40", "X1:" }
                    Input { value: x1.read().clone(), onchange: move |v: String| x1.set(v), placeholder: "x1 (м)", width: "100" }
                    label { width: "40", "Y1:" }
                    Input { value: y1.read().clone(), onchange: move |v: String| y1.set(v), placeholder: "y1 (м)", width: "100" }
                }
                
                rect {
                    direction: "horizontal",
                    spacing: "10",
                    cross_align: "center",
                    label { width: "40", "X2:" }
                    Input { value: x2.read().clone(), onchange: move |v: String| x2.set(v), placeholder: "x2 (м)", width: "100" }
                    label { width: "40", "Y2:" }
                    Input { value: y2.read().clone(), onchange: move |v: String| y2.set(v), placeholder: "y2 (м)", width: "100" }
                }
                
                rect {
                    direction: "horizontal",
                    spacing: "10",
                    cross_align: "center",
                    label { width: "40", "ε:" }
                    Input { value: eps.read().clone(), onchange: move |v: String| eps.set(v), placeholder: "eps", width: "100" }
                    label { width: "40", "μ:" }
                    Input { value: mu.read().clone(), onchange: move |v: String| mu.set(v), placeholder: "mu", width: "100" }
                }
                
                if !err.is_empty() {
                    rect {
                        padding: "5",
                        label { color: "red", "{err}" }
                    }
                }
            }
            
            rect {
                direction: "horizontal",
                spacing: "10",
                main_align: "center",
                margin: "15 0 0 0",
                rect { 
                    width: "110", height: "36", 
                    background: "rgb(0, 150, 0)", 
                    corner_radius: "4", 
                    main_align: "center", 
                    cross_align: "center", 
                    onclick: handle_create,
                    label { color: "white", "Создать" }
                }
                rect { 
                    width: "110", height: "36", 
                    background: "rgb(255, 255, 255)", 
                    corner_radius: "4", 
                    border: "1 solid #ccc",
                    main_align: "center", 
                    cross_align: "center", 
                    onclick: handle_cancel,
                    label { "Отмена" }
                }
            }
        }
    )
}

/// Диалог создания источника (в памяти)
#[component]
pub fn SourceDialog(
    active_dialog: Signal<ActiveDialog>,
    modelling: Signal<Option<Modelling>>,
    sources: Signal<Arc<Vec<(f32, f32)>>>,
    next_source_id: Signal<usize>,
    source_ids: Signal<Arc<Vec<usize>>>,
) -> Element {
    let mut x = use_signal(|| String::new());
    let mut y = use_signal(|| String::new());
    let error_msg = use_signal(|| String::new());
    
    let handle_create = {
        let mut sources = sources.clone();
        let mut next_source_id = next_source_id.clone();
        let mut source_ids = source_ids.clone();
        let mut active_dialog = active_dialog.clone();
        let modelling = modelling.clone();
        let x = x.clone();
        let y = y.clone();
        let mut error_msg = error_msg.clone();
        
        move |_| {
            let m = match modelling.read().clone() {
                Some(m) => m,
                None => {
                    error_msg.set("Сначала настройте параметры проекта".to_string());
                    return;
                }
            };
            
            let x_val = match x.read().trim().parse::<f32>() {
                Ok(v) => v,
                Err(_) => { error_msg.set("X должно быть числом".to_string()); return; }
            };
            let y_val = match y.read().trim().parse::<f32>() {
                Ok(v) => v,
                Err(_) => { error_msg.set("Y должно быть числом".to_string()); return; }
            };
            
            // Нормализуем координаты
            let nx = x_val / m.sizex;
            let ny = y_val / m.sizey;
            
            // Добавляем в список (нормализованные координаты)
            let mut srcs = (*sources.read()).as_ref().clone();
            srcs.push((nx, ny));
            sources.set(Arc::new(srcs));
            
            let mut ids = (*source_ids.read()).as_ref().clone();
            let new_id = *next_source_id.read();
            ids.push(new_id);
            source_ids.set(Arc::new(ids));
            next_source_id.set(new_id + 1);
            
            active_dialog.set(ActiveDialog::None);
        }
    };
    
    let handle_cancel = {
        let mut active_dialog = active_dialog.clone();
        move |_| {
            active_dialog.set(ActiveDialog::None);
        }
    };
    
    let err = error_msg.read().clone();
    
    rsx!(
        rect {
            width: "320",
            height: "220",
            background: "rgb(240, 240, 240)",
            corner_radius: "8",
            padding: "20",
            direction: "vertical",
            
            rect {
                height: "36",
                main_align: "center",
                cross_align: "center",
                label { font_size: "18", font_weight: "bold", "Создание источника" }
            }
            
            rect {
                direction: "vertical",
                spacing: "8",
                margin: "10 0",
                
                rect {
                    direction: "horizontal",
                    spacing: "10",
                    cross_align: "center",
                    label { width: "30", "X:" }
                    Input { value: x.read().clone(), onchange: move |v: String| x.set(v), placeholder: "x (м)", width: "100" }
                    label { width: "30", "Y:" }
                    Input { value: y.read().clone(), onchange: move |v: String| y.set(v), placeholder: "y (м)", width: "100" }
                }
                
                if !err.is_empty() {
                    rect {
                        padding: "5",
                        label { color: "red", "{err}" }
                    }
                }
            }
            
            rect {
                direction: "horizontal",
                spacing: "10",
                main_align: "center",
                margin: "15 0 0 0",
                rect { 
                    width: "110", height: "36", 
                    background: "rgb(0, 150, 0)", 
                    corner_radius: "4", 
                    main_align: "center", 
                    cross_align: "center", 
                    onclick: handle_create,
                    label { color: "white", "Создать" }
                }
                rect { 
                    width: "110", height: "36", 
                    background: "rgb(255, 255, 255)", 
                    corner_radius: "4", 
                    border: "1 solid #ccc",
                    main_align: "center", 
                    cross_align: "center", 
                    onclick: handle_cancel,
                    label { "Отмена" }
                }
            }
        }
    )
}

/// Диалог создания датчика (в памяти)
#[component]
pub fn ProbeDialog(
    active_dialog: Signal<ActiveDialog>,
    modelling: Signal<Option<Modelling>>,
    probes: Signal<Arc<Vec<(f32, f32)>>>,
    next_probe_id: Signal<usize>,
    probe_ids: Signal<Arc<Vec<usize>>>,
) -> Element {
    let mut x = use_signal(|| String::new());
    let mut y = use_signal(|| String::new());
    let error_msg = use_signal(|| String::new());
    
    let handle_create = {
        let mut probes = probes.clone();
        let mut next_probe_id = next_probe_id.clone();
        let mut probe_ids = probe_ids.clone();
        let mut active_dialog = active_dialog.clone();
        let modelling = modelling.clone();
        let x = x.clone();
        let y = y.clone();
        let mut error_msg = error_msg.clone();
        
        move |_| {
            let m = match modelling.read().clone() {
                Some(m) => m,
                None => {
                    error_msg.set("Сначала настройте параметры проекта".to_string());
                    return;
                }
            };
            
            let x_val = match x.read().trim().parse::<f32>() {
                Ok(v) => v,
                Err(_) => { error_msg.set("X должно быть числом".to_string()); return; }
            };
            let y_val = match y.read().trim().parse::<f32>() {
                Ok(v) => v,
                Err(_) => { error_msg.set("Y должно быть числом".to_string()); return; }
            };
            
            // Нормализуем координаты
            let nx = x_val / m.sizex;
            let ny = y_val / m.sizey;
            
            // Добавляем в список (нормализованные координаты)
            let mut prbs = (*probes.read()).as_ref().clone();
            prbs.push((nx, ny));
            probes.set(Arc::new(prbs));
            
            let mut ids = (*probe_ids.read()).as_ref().clone();
            let new_id = *next_probe_id.read();
            ids.push(new_id);
            probe_ids.set(Arc::new(ids));
            next_probe_id.set(new_id + 1);
            
            active_dialog.set(ActiveDialog::None);
        }
    };
    
    let handle_cancel = {
        let mut active_dialog = active_dialog.clone();
        move |_| {
            active_dialog.set(ActiveDialog::None);
        }
    };
    
    let err = error_msg.read().clone();
    
    rsx!(
        rect {
            width: "320",
            height: "220",
            background: "rgb(240, 240, 240)",
            corner_radius: "8",
            padding: "20",
            direction: "vertical",
            
            rect {
                height: "36",
                main_align: "center",
                cross_align: "center",
                label { font_size: "18", font_weight: "bold", "Создание датчика" }
            }
            
            rect {
                direction: "vertical",
                spacing: "8",
                margin: "10 0",
                
                rect {
                    direction: "horizontal",
                    spacing: "10",
                    cross_align: "center",
                    label { width: "30", "X:" }
                    Input { value: x.read().clone(), onchange: move |v: String| x.set(v), placeholder: "x (м)", width: "100" }
                    label { width: "30", "Y:" }
                    Input { value: y.read().clone(), onchange: move |v: String| y.set(v), placeholder: "y (м)", width: "100" }
                }
                
                if !err.is_empty() {
                    rect {
                        padding: "5",
                        label { color: "red", "{err}" }
                    }
                }
            }
            
            rect {
                direction: "horizontal",
                spacing: "10",
                main_align: "center",
                margin: "15 0 0 0",
                rect { 
                    width: "110", height: "36", 
                    background: "rgb(0, 150, 0)", 
                    corner_radius: "4", 
                    main_align: "center", 
                    cross_align: "center", 
                    onclick: handle_create,
                    label { color: "white", "Создать" }
                }
                rect { 
                    width: "110", height: "36", 
                    background: "rgb(255, 255, 255)", 
                    corner_radius: "4", 
                    border: "1 solid #ccc",
                    main_align: "center", 
                    cross_align: "center", 
                    onclick: handle_cancel,
                    label { "Отмена" }
                }
            }
        }
    )
}
