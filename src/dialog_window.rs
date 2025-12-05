use crate::functions::{
    ensure_temp_config_path, load_config, ObjectType, ProjectObject, ProjectSettings,
};
use freya::prelude::*;
use winit::window::WindowButtons;

/// Отдельное диалоговое окно для добавления объекта
/// Это окно будет запускаться как отдельное приложение
#[component]
pub fn AddObjectDialogApp() -> Element {
    let config_path = ensure_temp_config_path();
    let initial_settings = load_config(&config_path)
        .map(|cfg| ProjectSettings::from_config(&cfg))
        .unwrap_or_else(|_| ProjectSettings::default());
    let project_description = initial_settings.description.clone();
    let initial_objects = initial_settings.objects.clone();

    // Параметры рабочей области (в метрах)
    let mut sizex = use_signal(|| initial_settings.sizex);
    let mut sizey = use_signal(|| initial_settings.sizey);
    let mut dx = use_signal(|| initial_settings.dx);
    let mut dy = use_signal(|| initial_settings.dy);
    let mut maxtime = use_signal(|| initial_settings.maxtime);

    // Список добавленных объектов
    let objects = use_signal(|| initial_objects);

    // Выбор и параметры нового объекта
    let mut selected_object_type = use_signal(|| ObjectType::Rectangle);
    let mut new_object_x1 = use_signal(|| String::new());
    let mut new_object_y1 = use_signal(|| String::new());
    let mut new_object_x2 = use_signal(|| String::new());
    let mut new_object_y2 = use_signal(|| String::new());
    let mut new_object_eps = use_signal(|| String::new());
    let mut new_object_mu = use_signal(|| String::new());

    let handle_add = {
        let mut objects = objects.clone();
        let mut new_object_x1 = new_object_x1.clone();
        let mut new_object_y1 = new_object_y1.clone();
        let mut new_object_x2 = new_object_x2.clone();
        let mut new_object_y2 = new_object_y2.clone();
        let mut new_object_eps = new_object_eps.clone();
        let mut new_object_mu = new_object_mu.clone();
        let selected_object_type = selected_object_type.clone();
        move |_: freya::events::MouseEvent| {
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

            // Проверяем, что поля содержат не только пробелы
            if x1_str.chars().all(|c| c.is_whitespace())
                || y1_str.chars().all(|c| c.is_whitespace())
            {
                println!("Ошибка: Поля X1 и Y1 не могут содержать только пробелы");
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

                // Проверяем, что поля X2 и Y2 содержат не только пробелы
                if x2_str.chars().all(|c| c.is_whitespace())
                    || y2_str.chars().all(|c| c.is_whitespace())
                {
                    println!("Ошибка: Поля X2 и Y2 не могут содержать только пробелы");
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

            // Проверяем разумность значений координат
            if x1.is_nan() || y1.is_nan() || x1.is_infinite() || y1.is_infinite() {
                println!("Ошибка: Координаты X1 и Y1 должны быть конечными числами");
                return;
            }

            let mut new_object = ProjectObject {
                object_type,
                x1,
                y1,
                x2: None,
                y2: None,
                eps: None,
                mu: None,
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

                // Проверяем разумность значений координат X2 и Y2
                if x2.is_nan() || y2.is_nan() || x2.is_infinite() || y2.is_infinite() {
                    println!("Ошибка: Координаты X2 и Y2 должны быть конечными числами");
                    return;
                }

                // Проверяем логичность размеров прямоугольника
                if x2 <= x1 || y2 <= y1 {
                    println!("Ошибка: Для прямоугольника X2 должен быть больше X1, а Y2 больше Y1");
                    return;
                }

                new_object.x2 = Some(x2);
                new_object.y2 = Some(y2);

                // Обрабатываем параметры ε и μ для прямоугольника
                let eps_str = {
                    let eps_value = new_object_eps.read();
                    eps_value.trim().to_string()
                };
                let mu_str = {
                    let mu_value = new_object_mu.read();
                    mu_value.trim().to_string()
                };

                // Проверяем, что поля ε и μ заполнены
                if eps_str.is_empty() || mu_str.is_empty() {
                    println!("Ошибка: Поля ε и μ должны быть заполнены для прямоугольника");
                    return;
                }

                // Парсим значения ε и μ
                let eps = match eps_str.parse::<f32>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Ошибка: ε должно быть числом");
                        return;
                    }
                };

                let mu = match mu_str.parse::<f32>() {
                    Ok(val) => val,
                    Err(_) => {
                        println!("Ошибка: μ должно быть числом");
                        return;
                    }
                };

                // Проверяем разумность значений
                if eps.is_nan() || mu.is_nan() || eps.is_infinite() || mu.is_infinite() {
                    println!("Ошибка: ε и μ должны быть конечными числами");
                    return;
                }

                if eps <= 0.0 || mu <= 0.0 {
                    println!("Ошибка: ε и μ должны быть положительными числами");
                    return;
                }

                new_object.eps = Some(eps);
                new_object.mu = Some(mu);
            }

            let mut v = objects.read().clone();
            v.push(new_object);
            objects.set(v);

            // Очищаем поля ввода
            new_object_x1.set(String::new());
            new_object_y1.set(String::new());
            new_object_x2.set(String::new());
            new_object_y2.set(String::new());
            new_object_eps.set(String::from("4.0")); // Возвращаем значения по умолчанию
            new_object_mu.set(String::from("1.0"));

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
        move |_: freya::events::MouseEvent| {
            // Проверяем, что есть хотя бы один объект
            if objects.read().is_empty() {
                println!("Ошибка: Нельзя создать пустой проект. Добавьте хотя бы один объект.");
                return;
            }

            // Создаем временный TOML файл
            let temp_file = ensure_temp_config_path();

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
                    if let (Some(x2), Some(y2), Some(eps), Some(mu)) =
                        (obj.x2, obj.y2, obj.eps, obj.mu)
                    {
                        toml_content.push_str("  [[geometry.rectangle]]\n");
                        toml_content.push_str(&format!("  x1 = {}\n", obj.x1));
                        toml_content.push_str(&format!("  y1 = {}\n", obj.y1));
                        toml_content.push_str(&format!("  x2 = {}\n", x2));
                        toml_content.push_str(&format!("  y2 = {}\n", y2));
                        toml_content.push_str(&format!("  eps = {}\n", eps));
                        toml_content.push_str(&format!("  mu = {}\n", mu));
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
                    toml_content.push_str("  [[sources.cylindrical]]\n");
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
        move |_: freya::events::MouseEvent| -> () {
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
                    rect {
                        direction: "horizontal",
                        spacing: "8",
                        cross_align: "center",
                        label { width: "50", "ε:" }
                        Input { value: new_object_eps.read().clone(), onchange: move |v: String| new_object_eps.set(v), placeholder: "eps", width: "100" }
                        label { width: "50", "μ:" }
                        Input { value: new_object_mu.read().clone(), onchange: move |v: String| new_object_mu.set(v), placeholder: "mu", width: "100" }
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
                                            if let (Some(x2), Some(y2), Some(eps), Some(mu)) = (obj.x2, obj.y2, obj.eps, obj.mu) {
                                                format!("({:.2}, {:.2}) - ({:.2}, {:.2}) ε={:.2} μ={:.2}", obj.x1, obj.y1, x2, y2, eps, mu)
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
                rect { width: "110", height: "30", background: "rgb(0, 150, 0)", corner_radius: "4", border: "1 solid #ccc", main_align: "center", cross_align: "center", onclick: handle_apply,
                    label { color: "white", "Задать" }
                }
                rect { width: "110", height: "30", background: "rgb(255, 255, 255)", corner_radius: "4", border: "1 solid #ccc", main_align: "center", cross_align: "center", onclick: handle_cancel,
                    label { "Отмена" }
                }
            }
        }
    )
}

/// Диалоговое окно для создания прямоугольника
#[component]
pub fn RectangleDialogApp() -> Element {
    // Параметры прямоугольника
    let mut x1 = use_signal(|| String::new());
    let mut y1 = use_signal(|| String::new());
    let mut x2 = use_signal(|| String::new());
    let mut y2 = use_signal(|| String::new());
    let mut eps = use_signal(|| String::from("4.0"));
    let mut mu = use_signal(|| String::from("1.0"));

    let handle_create = {
        let x1 = x1.clone();
        let y1 = y1.clone();
        let x2 = x2.clone();
        let y2 = y2.clone();
        let eps = eps.clone();
        let mu = mu.clone();
        move |_: freya::events::MouseEvent| {
            // Проверяем, что все поля заполнены
            let x1_str = x1.read().trim().to_string();
            let y1_str = y1.read().trim().to_string();
            let x2_str = x2.read().trim().to_string();
            let y2_str = y2.read().trim().to_string();
            let eps_str = eps.read().trim().to_string();
            let mu_str = mu.read().trim().to_string();

            if x1_str.is_empty()
                || y1_str.is_empty()
                || x2_str.is_empty()
                || y2_str.is_empty()
                || eps_str.is_empty()
                || mu_str.is_empty()
            {
                println!("Ошибка: Все поля должны быть заполнены");
                return;
            }

            // Парсим значения
            let x1_val = match x1_str.parse::<f32>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Ошибка: X1 должно быть числом");
                    return;
                }
            };

            let y1_val = match y1_str.parse::<f32>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Ошибка: Y1 должно быть числом");
                    return;
                }
            };

            let x2_val = match x2_str.parse::<f32>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Ошибка: X2 должно быть числом");
                    return;
                }
            };

            let y2_val = match y2_str.parse::<f32>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Ошибка: Y2 должно быть числом");
                    return;
                }
            };

            let eps_val = match eps_str.parse::<f32>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Ошибка: ε должно быть числом");
                    return;
                }
            };

            let mu_val = match mu_str.parse::<f32>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Ошибка: μ должно быть числом");
                    return;
                }
            };

            // Проверяем разумность значений
            if x1_val.is_nan()
                || y1_val.is_nan()
                || x2_val.is_nan()
                || y2_val.is_nan()
                || eps_val.is_nan()
                || mu_val.is_nan()
            {
                println!("Ошибка: Все значения должны быть конечными числами");
                return;
            }

            if eps_val <= 0.0 || mu_val <= 0.0 {
                println!("Ошибка: ε и μ должны быть положительными числами");
                return;
            }

            if x2_val <= x1_val || y2_val <= y1_val {
                println!("Ошибка: X2 должен быть больше X1, а Y2 больше Y1");
                return;
            }

            // Создаем объект прямоугольника
            let new_object = ProjectObject {
                object_type: ObjectType::Rectangle,
                x1: x1_val,
                y1: y1_val,
                x2: Some(x2_val),
                y2: Some(y2_val),
                eps: Some(eps_val),
                mu: Some(mu_val),
            };

            // Добавляем в временный TOML файл
            add_object_to_temp_toml(new_object);

            println!("Прямоугольник успешно создан");
            std::process::exit(0);
        }
    };

    let handle_cancel = {
        move |_: freya::events::MouseEvent| -> () {
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
            padding: "20",

            // Заголовок
            rect {
                height: "40",
                main_align: "center",
                cross_align: "center",
                label { font_size: "18", font_weight: "bold", "Создание прямоугольника" }
            }

            // Параметры прямоугольника
            rect {
                direction: "vertical",
                spacing: "10",
                label { font_size: "14", font_weight: "bold", "Параметры прямоугольника:" }

                // Координаты
                rect {
                    direction: "horizontal",
                    spacing: "10",
                    cross_align: "center",
                    label { width: "50", "X1:" }
                    Input { value: x1.read().clone(), onchange: move |v: String| x1.set(v), placeholder: "x1", width: "100" }
                    label { width: "50", "Y1:" }
                    Input { value: y1.read().clone(), onchange: move |v: String| y1.set(v), placeholder: "y1", width: "100" }
                }

                    rect {
                        direction: "horizontal",
                    spacing: "10",
                        cross_align: "center",
                        label { width: "50", "X2:" }
                    Input { value: x2.read().clone(), onchange: move |v: String| x2.set(v), placeholder: "x2", width: "100" }
                        label { width: "50", "Y2:" }
                    Input { value: y2.read().clone(), onchange: move |v: String| y2.set(v), placeholder: "y2", width: "100" }
                }

                // Материальные параметры
                    rect {
                        direction: "horizontal",
                    spacing: "10",
                        cross_align: "center",
                        label { width: "50", "ε:" }
                    Input { value: eps.read().clone(), onchange: move |v: String| eps.set(v), placeholder: "eps", width: "100" }
                        label { width: "50", "μ:" }
                    Input { value: mu.read().clone(), onchange: move |v: String| mu.set(v), placeholder: "mu", width: "100" }
                }
            }

            // Кнопки
            rect {
                direction: "horizontal",
                spacing: "10",
                main_align: "center",
                margin: "20 0 0 0",
                rect { width: "110", height: "30", background: "rgb(0, 150, 0)", corner_radius: "4", border: "1 solid #ccc", main_align: "center", cross_align: "center", onclick: handle_create,
                    label { color: "white", "Создать" }
                }
                rect { width: "110", height: "30", background: "rgb(255, 255, 255)", corner_radius: "4", border: "1 solid #ccc", main_align: "center", cross_align: "center", onclick: handle_cancel,
                    label { "Отмена" }
                }
            }
        }
    )
}

/// Диалоговое окно для создания источника
#[component]
pub fn SourceDialogApp() -> Element {
    // Параметры источника
    let mut x = use_signal(|| String::new());
    let mut y = use_signal(|| String::new());

    let handle_create = {
        let x = x.clone();
        let y = y.clone();
        move |_: freya::events::MouseEvent| {
            // Проверяем, что все поля заполнены
            let x_str = x.read().trim().to_string();
            let y_str = y.read().trim().to_string();

            if x_str.is_empty() || y_str.is_empty() {
                println!("Ошибка: Все поля должны быть заполнены");
                return;
            }

            // Парсим значения
            let x_val = match x_str.parse::<f32>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Ошибка: X должно быть числом");
                    return;
                }
            };

            let y_val = match y_str.parse::<f32>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Ошибка: Y должно быть числом");
                    return;
                }
            };

            // Проверяем разумность значений
            if x_val.is_nan() || y_val.is_nan() || x_val.is_infinite() || y_val.is_infinite() {
                println!("Ошибка: Координаты должны быть конечными числами");
                return;
            }

            // Создаем объект источника
            let new_object = ProjectObject {
                object_type: ObjectType::Source,
                x1: x_val,
                y1: y_val,
                x2: None,
                y2: None,
                eps: None,
                mu: None,
            };

            // Добавляем в временный TOML файл
            add_object_to_temp_toml(new_object);

            println!("Источник успешно создан");
            std::process::exit(0);
        }
    };

    let handle_cancel = {
        move |_: freya::events::MouseEvent| -> () {
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
            padding: "20",

            // Заголовок
            rect {
                height: "40",
                main_align: "center",
                cross_align: "center",
                label { font_size: "18", font_weight: "bold", "Создание источника" }
            }

            // Параметры источника
            rect {
                direction: "vertical",
                spacing: "10",
                label { font_size: "14", font_weight: "bold", "Параметры источника:" }

                // Координаты
                rect {
                    direction: "horizontal",
                    spacing: "10",
                    cross_align: "center",
                    label { width: "50", "X:" }
                    Input { value: x.read().clone(), onchange: move |v: String| x.set(v), placeholder: "x", width: "100" }
                    label { width: "50", "Y:" }
                    Input { value: y.read().clone(), onchange: move |v: String| y.set(v), placeholder: "y", width: "100" }
                }
            }

            // Кнопки
            rect {
                direction: "horizontal",
                spacing: "10",
                main_align: "center",
                margin: "20 0 0 0",
                rect { width: "110", height: "30", background: "rgb(0, 150, 0)", corner_radius: "4", border: "1 solid #ccc", main_align: "center", cross_align: "center", onclick: handle_create,
                    label { color: "white", "Создать" }
                }
                rect { width: "110", height: "30", background: "rgb(255, 255, 255)", corner_radius: "4", border: "1 solid #ccc", main_align: "center", cross_align: "center", onclick: handle_cancel,
                    label { "Отмена" }
                }
            }
        }
    )
}

/// Диалоговое окно для создания зонда
#[component]
pub fn ProbeDialogApp() -> Element {
    // Параметры зонда
    let mut x = use_signal(|| String::new());
    let mut y = use_signal(|| String::new());

    let handle_create = {
        let x = x.clone();
        let y = y.clone();
        move |_: freya::events::MouseEvent| {
            // Проверяем, что все поля заполнены
            let x_str = x.read().trim().to_string();
            let y_str = y.read().trim().to_string();

            if x_str.is_empty() || y_str.is_empty() {
                println!("Ошибка: Все поля должны быть заполнены");
                return;
            }

            // Парсим значения
            let x_val = match x_str.parse::<f32>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Ошибка: X должно быть числом");
                    return;
                }
            };

            let y_val = match y_str.parse::<f32>() {
                Ok(val) => val,
                Err(_) => {
                    println!("Ошибка: Y должно быть числом");
                    return;
                }
            };

            // Проверяем разумность значений
            if x_val.is_nan() || y_val.is_nan() || x_val.is_infinite() || y_val.is_infinite() {
                println!("Ошибка: Координаты должны быть конечными числами");
                return;
            }

            // Создаем объект зонда
            let new_object = ProjectObject {
                object_type: ObjectType::Probe,
                x1: x_val,
                y1: y_val,
                x2: None,
                y2: None,
                eps: None,
                mu: None,
            };

            // Добавляем в временный TOML файл
            add_object_to_temp_toml(new_object);

            println!("Зонд успешно создан");
            std::process::exit(0);
        }
    };

    let handle_cancel = {
        move |_: freya::events::MouseEvent| -> () {
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
            padding: "20",

            // Заголовок
            rect {
                height: "40",
                    main_align: "center",
                    cross_align: "center",
                label { font_size: "18", font_weight: "bold", "Создание зонда" }
                }

            // Параметры зонда
                rect {
                direction: "vertical",
                spacing: "10",
                label { font_size: "14", font_weight: "bold", "Параметры зонда:" }

                // Координаты
                rect {
                    direction: "horizontal",
                    spacing: "10",
                    cross_align: "center",
                    label { width: "50", "X:" }
                    Input { value: x.read().clone(), onchange: move |v: String| x.set(v), placeholder: "x", width: "100" }
                    label { width: "50", "Y:" }
                    Input { value: y.read().clone(), onchange: move |v: String| y.set(v), placeholder: "y", width: "100" }
                }
            }

            // Кнопки
            rect {
                direction: "horizontal",
                spacing: "10",
                main_align: "center",
                margin: "20 0 0 0",
                rect { width: "110", height: "30", background: "rgb(0, 150, 0)", corner_radius: "4", border: "1 solid #ccc", main_align: "center", cross_align: "center", onclick: handle_create,
                    label { color: "white", "Создать" }
                }
                rect { width: "110", height: "30", background: "rgb(255, 255, 255)", corner_radius: "4", border: "1 solid #ccc", main_align: "center", cross_align: "center", onclick: handle_cancel,
                    label { "Отмена" }
                }
            }
        }
    )
}

/// Функция для добавления объекта в временный TOML файл
fn add_object_to_temp_toml(object: ProjectObject) {
    let temp_file = ensure_temp_config_path();

    // Читаем существующий файл или создаем новый
    let toml_content = if temp_file.exists() {
        std::fs::read_to_string(&temp_file).unwrap_or_default()
    } else {
        // Создаем базовую структуру файла
        "description = \"Временная конфигурация\"\n\n[modelling]\nsizex = 1.0\nsizey = 1.0\ndx = 0.01\ndy = 0.01\nmaxtime = 1.0\n\n[boundary]\n  [boundary.xmin]\n  type = \"PEC\"\n  param1 = \"...\"\n  param2 = \"...\"\n  [boundary.xmax]\n  type = \"PEC\"\n  param1 = \"...\"\n  param2 = \"...\"\n  [boundary.ymin]\n  type = \"PEC\"\n  param1 = \"...\"\n  param2 = \"...\"\n  [boundary.ymax]\n  type = \"PEC\"\n  param1 = \"...\"\n  param2 = \"...\"\n\n[geometry]\n\n[probes]\n\n[sources]\n".to_string()
    };

    // Создаем строку для добавления объекта
    let object_section = match object.object_type {
        ObjectType::Rectangle => {
            if let (Some(x2), Some(y2), Some(eps), Some(mu)) =
                (object.x2, object.y2, object.eps, object.mu)
            {
                format!(
                    "  [[geometry.rectangle]]\n  x1 = {}\n  y1 = {}\n  x2 = {}\n  y2 = {}\n  eps = {}\n  mu = {}\n  sigma = 0.01\n  color = \"0, 0, 255\"\n",
                    object.x1, object.y1, x2, y2, eps, mu
                )
            } else {
                return; // Недостаточно данных для прямоугольника
            }
        }
        ObjectType::Source => {
            format!(
                "  [[sources.cylindrical]]\n  x = {}\n  y = {}\n  type = \"sin\"\n  param1 = \"...\"\n  param2 = \"...\"\n",
                object.x1, object.y1
            )
        }
        ObjectType::Probe => {
            format!(
                "  [[probes.probe]]\n  x = {}\n  y = {}\n  color = \"0, 255, 255\"\n",
                object.x1, object.y1
            )
        }
    };

    // Определяем целевой раздел
    let target_section = match object.object_type {
        ObjectType::Rectangle => "[geometry]",
        ObjectType::Source => "[sources]",
        ObjectType::Probe => "[probes]",
    };

    // Простая стратегия: добавляем объект в конец соответствующего раздела
    let lines: Vec<&str> = toml_content.lines().collect();
    let mut new_lines = Vec::new();
    let mut found_section = false;
    let mut in_target_section = false;

    for line in lines.iter() {
        new_lines.push(line.to_string());

        if line.trim() == target_section {
            found_section = true;
            in_target_section = true;
        } else if in_target_section && line.starts_with('[') && line.trim() != target_section {
            // Нашли следующий раздел, вставляем объект перед ним
            new_lines.insert(new_lines.len() - 1, object_section.clone());
            in_target_section = false;
        }
    }

    // Если мы все еще в целевом разделе (он последний), добавляем объект
    if in_target_section {
        new_lines.push(object_section.clone());
    }

    // Если раздел не найден, добавляем в конец
    if !found_section {
        new_lines.push(object_section.clone());
    }

    let updated_content = new_lines.join("\n");

    // Записываем обновленный файл
    if let Err(e) = std::fs::write(&temp_file, updated_content) {
        eprintln!("Ошибка записи временного файла: {}", e);
    } else {
        println!("Объект добавлен в временный файл: {:?}", temp_file);
    }
}

fn launch_fixed_dialog(app: fn() -> Element, title: &'static str, size: (f64, f64)) {
    launch_cfg(
        app,
        LaunchConfig::<()>::new()
            .with_title(title)
            .with_size(size.0, size.1)
            .with_window_attributes(|attributes| {
                attributes
                    .with_resizable(false)
                    .with_enabled_buttons(WindowButtons::CLOSE | WindowButtons::MINIMIZE)
            }),
    );
}

/// Запуск диалогового окна с настройками размера
pub fn launch_dialog_app() {
    launch_fixed_dialog(AddObjectDialogApp, "Настройки проекта", (600.0, 675.0));
}

/// Запуск диалогового окна для создания прямоугольника
pub fn launch_rectangle_dialog() {
    launch_fixed_dialog(
        RectangleDialogApp,
        "Создание прямоугольника",
        (400.0, 300.0),
    );
}

/// Запуск диалогового окна для создания источника
pub fn launch_source_dialog() {
    launch_fixed_dialog(SourceDialogApp, "Создание источника", (300.0, 200.0));
}

/// Запуск диалогового окна для создания зонда
pub fn launch_probe_dialog() {
    launch_fixed_dialog(ProbeDialogApp, "Создание зонда", (300.0, 200.0));
}
