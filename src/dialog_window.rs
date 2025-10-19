use freya::prelude::*;
use crate::functions::{ProjectObject, ObjectType};

/// Отдельное диалоговое окно для добавления объекта
/// Это окно будет запускаться как отдельное приложение
#[component]
pub fn AddObjectDialogApp() -> Element {
    // Параметры рабочей области (в метрах)
    let mut sizex = use_signal(|| 1.0_f32);
    let mut sizey = use_signal(|| 1.0_f32);
    let mut dx = use_signal(|| 0.01_f32);
    let mut dy = use_signal(|| 0.01_f32);
    let mut maxtime = use_signal(|| 1.0_f32);

    // Список добавленных объектов
    let objects = use_signal(|| Vec::<ProjectObject>::new());

    // Выбор и параметры нового объекта
    let mut selected_object_type = use_signal(|| ObjectType::Rectangle);
    let mut new_object_x1 = use_signal(|| String::new());
    let mut new_object_y1 = use_signal(|| String::new());
    let mut new_object_x2 = use_signal(|| String::new());
    let mut new_object_y2 = use_signal(|| String::new());

    let handle_add = {
        let mut objects = objects.clone();
        let mut new_object_x1 = new_object_x1.clone();
        let mut new_object_y1 = new_object_y1.clone();
        let mut new_object_x2 = new_object_x2.clone();
        let mut new_object_y2 = new_object_y2.clone();
        let selected_object_type = selected_object_type.clone();
        move |_| {
            // Проверяем, что все поля заполнены
            let x1_str = {
                let x1_value = new_object_x1.read();
                x1_value.trim().to_string()
            };
            let y1_str = {
                let y1_value = new_object_y1.read();
                y1_value.trim().to_string()
            };
            
            if x1_str.is_empty() || y1_str.is_empty() {
                println!("Ошибка: Поля X1 и Y1 должны быть заполнены");
                return;
            }
            
            let object_type = *selected_object_type.read();
            
            // Для прямоугольника проверяем также X2 и Y2
            if object_type == ObjectType::Rectangle {
                let x2_str = {
                    let x2_value = new_object_x2.read();
                    x2_value.trim().to_string()
                };
                let y2_str = {
                    let y2_value = new_object_y2.read();
                    y2_value.trim().to_string()
                };
                
                if x2_str.is_empty() || y2_str.is_empty() {
                    println!("Ошибка: Для прямоугольника поля X2 и Y2 должны быть заполнены");
                    return;
                }
            }
            
            // Парсим значения
            let x1 = match x1_str.parse::<f32>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Ошибка: X1 должно быть числом");
                    return;
                }
            };
            
            let y1 = match y1_str.parse::<f32>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Ошибка: Y1 должно быть числом");
                    return;
                }
            };
            
            let mut new_object = ProjectObject {
                object_type,
                x1,
                y1,
                x2: None,
                y2: None,
            };
            
            if object_type == ObjectType::Rectangle {
                let x2_str = {
                    let x2_value = new_object_x2.read();
                    x2_value.trim().to_string()
                };
                let y2_str = {
                    let y2_value = new_object_y2.read();
                    y2_value.trim().to_string()
                };
                
                let x2 = match x2_str.parse::<f32>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Ошибка: X2 должно быть числом");
                        return;
                    }
                };
                
                let y2 = match y2_str.parse::<f32>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Ошибка: Y2 должно быть числом");
                        return;
                    }
                };
                
                new_object.x2 = Some(x2);
                new_object.y2 = Some(y2);
            }
            
            let mut v = objects.read().clone();
            v.push(new_object);
            objects.set(v);

            // Очищаем поля ввода
            new_object_x1.set(String::new());
            new_object_y1.set(String::new());
            new_object_x2.set(String::new());
            new_object_y2.set(String::new());
            
            println!("Объект успешно добавлен в список");
        }
    };

    let handle_apply = {
        let objects = objects.clone();
        let sizex = sizex.clone();
        let sizey = sizey.clone();
        let dx = dx.clone();
        let dy = dy.clone();
        let maxtime = maxtime.clone();
        move |_: ()| {
            // Создаем временный TOML файл
            let temp_file = std::env::temp_dir().join("maxwell_temp_config.toml");
            
            // Формируем содержимое TOML файла
            let mut toml_content = String::new();
            toml_content.push_str("description = \"Временная конфигурация\"\n");
            toml_content.push_str("\n[modelling]\n");
            toml_content.push_str(&format!("sizex = {}\n", *sizex.read()));
            toml_content.push_str(&format!("sizey = {}\n", *sizey.read()));
            toml_content.push_str(&format!("dx = {}\n", *dx.read()));
            toml_content.push_str(&format!("dy = {}\n", *dy.read()));
            toml_content.push_str(&format!("maxtime = {}\n", *maxtime.read()));
            
            // Граничные условия по умолчанию
            toml_content.push_str("\n[boundary]\n");
            toml_content.push_str("  [boundary.xmin]\n");
            toml_content.push_str("  type = \"PEC\"\n");
            toml_content.push_str("  param1 = \"...\"\n");
            toml_content.push_str("  param2 = \"...\"\n");
            toml_content.push_str("  [boundary.xmax]\n");
            toml_content.push_str("  type = \"PEC\"\n");
            toml_content.push_str("  param1 = \"...\"\n");
            toml_content.push_str("  param2 = \"...\"\n");
            toml_content.push_str("  [boundary.ymin]\n");
            toml_content.push_str("  type = \"PEC\"\n");
            toml_content.push_str("  param1 = \"...\"\n");
            toml_content.push_str("  param2 = \"...\"\n");
            toml_content.push_str("  [boundary.ymax]\n");
            toml_content.push_str("  type = \"PEC\"\n");
            toml_content.push_str("  param1 = \"...\"\n");
            toml_content.push_str("  param2 = \"...\"\n");
            
            // Геометрия
            toml_content.push_str("\n[geometry]\n");
            for obj in objects.read().iter() {
                if obj.object_type == ObjectType::Rectangle {
                    if let (Some(x2), Some(y2)) = (obj.x2, obj.y2) {
                        toml_content.push_str("  [[geometry.rectangle]]\n");
                        toml_content.push_str(&format!("  x1 = {}\n", obj.x1));
                        toml_content.push_str(&format!("  y1 = {}\n", obj.y1));
                        toml_content.push_str(&format!("  x2 = {}\n", x2));
                        toml_content.push_str(&format!("  y2 = {}\n", y2));
                        toml_content.push_str("  eps = 4.0\n");
                        toml_content.push_str("  mu = 1.0\n");
                        toml_content.push_str("  sigma = 0.01\n");
                        toml_content.push_str("  color = \"0, 0, 255\"\n");
                    }
                }
            }
            
            // Зонды
            toml_content.push_str("\n[probes]\n");
            for obj in objects.read().iter() {
                if obj.object_type == ObjectType::Probe {
                    toml_content.push_str("  [[probes.probe]]\n");
                    toml_content.push_str(&format!("  x = {}\n", obj.x1));
                    toml_content.push_str(&format!("  y = {}\n", obj.y1));
                    toml_content.push_str("  color = \"0, 255, 255\"\n");
                }
            }
            
            // Источники
            toml_content.push_str("\n[sources]\n");
            for obj in objects.read().iter() {
                if obj.object_type == ObjectType::Source {
                    toml_content.push_str("  [[sources.cylindical]]\n");
                    toml_content.push_str(&format!("  x = {}\n", obj.x1));
                    toml_content.push_str(&format!("  y = {}\n", obj.y1));
                    toml_content.push_str("  type = \"sin\"\n");
                    toml_content.push_str("  param1 = \"...\"\n");
                    toml_content.push_str("  param2 = \"...\"\n");
                }
            }
            
            // Записываем файл
            if let Err(e) = std::fs::write(&temp_file, toml_content) {
                eprintln!("Ошибка записи временного файла: {}", e);
            } else {
                println!("Временный файл создан: {:?}", temp_file);
                // Сигнализируем основному окну о необходимости загрузить файл
                // В реальной реализации здесь будет IPC или другой механизм связи
            }
            
            // Завершаем процесс диалогового окна
            std::process::exit(0);
        }
    };

    let handle_cancel = {
        move |_: ()| {
            std::process::exit(1);
        }
    };

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            background: "rgb(240, 240, 240)",
            direction: "vertical",
            font_family: "Arial, sans-serif",
            padding: "16",
            rect {
                height: "36",
                main_align: "center",
                cross_align: "center",
                label { font_size: "18", font_weight: "bold", "Настройки проекта" }
            }

            // Параметры рабочей области (м)
            rect {
                direction: "vertical",
                spacing: "8",
                label { font_size: "14", font_weight: "bold", "Параметры рабочей области (м):" }
                // Размер области
                rect {
                    direction: "horizontal",
                    spacing: "8",
                    cross_align: "center",
                    label { width: "150", "Размер области (sizex, sizey):" }
                    Input { value: sizex.read().to_string(), onchange: move |v: String| if let Ok(val)=v.parse::<f32>(){ sizex.set(val); }, placeholder: "sizex", width: "100" }
                    Input { value: sizey.read().to_string(), onchange: move |v: String| if let Ok(val)=v.parse::<f32>(){ sizey.set(val); }, placeholder: "sizey", width: "100" }
                }
                // Размер ячейки
                rect {
                    direction: "horizontal",
                    spacing: "8",
                    cross_align: "center",
                    label { width: "150", "Размер ячейки (dx, dy):" }
                    Input { value: dx.read().to_string(), onchange: move |v: String| if let Ok(val)=v.parse::<f32>(){ dx.set(val); }, placeholder: "dx", width: "100" }
                    Input { value: dy.read().to_string(), onchange: move |v: String| if let Ok(val)=v.parse::<f32>(){ dy.set(val); }, placeholder: "dy", width: "100" }
                }
                // Время моделирования
                rect {
                    direction: "horizontal",
                    spacing: "8",
                    cross_align: "center",
                    label { width: "150", "Время моделирования (s):" }
                    Input { value: maxtime.read().to_string(), onchange: move |v: String| if let Ok(val)=v.parse::<f32>(){ maxtime.set(val); }, placeholder: "maxtime", width: "100" }
                }
            }
            rect {
                direction: "vertical",
                spacing: "8",
                margin: "12 0 0 0",
                label { font_size: "14", font_weight: "bold", "Новый объект:" }
                // Выбор типа
                rect {
                    direction: "horizontal",
                    spacing: "8",
                    for (object_type, label) in [
                        (ObjectType::Rectangle, "Прямоугольник"),
                        (ObjectType::Source, "Источник"),
                        (ObjectType::Probe, "Зонд"),
                    ] {
                        rect {
                            width: "120",
                            height: "28",
                            background: if *selected_object_type.read() == object_type { "rgb(100, 150, 255)" } else { "rgb(255, 255, 255)" },
                            corner_radius: "4",
                            border: "1 solid #ccc",
                            main_align: "center",
                            cross_align: "center",
                            onclick: move |_| selected_object_type.set(object_type),
                            label { "{label}" }
                        }
                    }
                }
            }
            rect {
                direction: "vertical",
                spacing: "8",
                margin: "12 0 0 0",
                label { font_size: "14", font_weight: "bold", "Координаты нового объекта:" }
                rect {
                    direction: "horizontal",
                    spacing: "8",
                    cross_align: "center",
                    label { width: "50", "X1:" }
                    Input { value: new_object_x1.read().clone(), onchange: move |v: String| new_object_x1.set(v), placeholder: "x1", width: "100" }
                    label { width: "50", "Y1:" }
                    Input { value: new_object_y1.read().clone(), onchange: move |v: String| new_object_y1.set(v), placeholder: "y1", width: "100" }
                }
                if *selected_object_type.read() == ObjectType::Rectangle {
                    rect {
                        direction: "horizontal",
                        spacing: "8",
                        cross_align: "center",
                        label { width: "50", "X2:" }
                        Input { value: new_object_x2.read().clone(), onchange: move |v: String| new_object_x2.set(v), placeholder: "x2", width: "100" }
                        label { width: "50", "Y2:" }
                        Input { value: new_object_y2.read().clone(), onchange: move |v: String| new_object_y2.set(v), placeholder: "y2", width: "100" }
                    }
                }
            }
            rect {
                direction: "horizontal",
                spacing: "8",
                main_align: "center",
                margin: "12 0 0 0",
                rect { width: "140", height: "30", background: "rgb(100, 150, 255)", corner_radius: "4", border: "1 solid #ccc", main_align: "center", cross_align: "center", onclick: handle_add,
                    label { color: "white", "Добавить объект" }
                }
            }
            
            // Список объектов (прокрутка, минимум 5 строк)
            rect {
                direction: "vertical",
                spacing: "6",
                margin: "12 0 0 0",
                label { font_size: "14", font_weight: "bold", "Объекты проекта:" }
                ScrollView {
                    height: "150",
                    rect {
                        direction: "vertical",
                        spacing: "4",
                        for obj in objects.read().iter() {
                            rect {
                                direction: "horizontal",
                                spacing: "8",
                                padding: "6",
                                background: "rgb(240, 240, 240)",
                                corner_radius: "4",
                                label { width: "100",
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

            // Кнопки управления под списком
            rect {
                direction: "horizontal",
                spacing: "10",
                main_align: "center",
                margin: "12 0 0 0",
                rect { width: "110", height: "30", background: "rgb(0, 150, 0)", corner_radius: "4", border: "1 solid #ccc", main_align: "center", cross_align: "center", onclick: move |_: freya::events::MouseEvent| handle_apply(()),
                    label { color: "white", "Задать" }
                }
                rect { width: "110", height: "30", background: "rgb(255, 255, 255)", corner_radius: "4", border: "1 solid #ccc", main_align: "center", cross_align: "center", onclick: move |_: freya::events::MouseEvent| handle_cancel(()),
                    label { "Отмена" }
                }
            }
        }
    )
}

// Удалена неиспользуемая функция launch_dialog_window

/// Запуск диалогового окна с настройками размера
pub fn launch_dialog_app() {
    launch_with_props(AddObjectDialogApp, "Настройки проекта", (600.0, 645.0));
}