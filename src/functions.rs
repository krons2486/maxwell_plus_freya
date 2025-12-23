use serde::Deserialize;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

const DEFAULT_DESCRIPTION: &str = "Временная конфигурация";

/// Диалог выбора TOML‑файла для открытия
pub fn select_toml_file() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("TOML", &["toml"])
        .set_title("Выберите файл конфигурации")
        .pick_file()
}

/// Диалог выбора места для сохранения TOML‑файла
pub fn save_toml_file_dialog() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("TOML", &["toml"])
        .set_title("Сохранить конфигурацию")
        .set_file_name("config.toml")
        .save_file()
}

/// Генерирует TOML-конфигурацию из данных в памяти
pub fn generate_toml_config(
    modelling: &Modelling,
    rectangles: &[((f32, f32), (f32, f32))],
    sources: &[(f32, f32)],
    probes: &[(f32, f32)],
) -> String {
    let mut toml = String::new();
    
    // Description
    writeln!(toml, "description = \"{}\"", DEFAULT_DESCRIPTION).unwrap();
    writeln!(toml).unwrap();
    
    // Modelling section
    writeln!(toml, "# Параметры моделирования").unwrap();
    writeln!(toml, "[modelling]").unwrap();
    writeln!(toml, "sizex = {}   # Размер области моделирования в метрах", modelling.sizex).unwrap();
    writeln!(toml, "sizey = {}   # Размер области моделирования в метрах", modelling.sizey).unwrap();
    writeln!(toml, "dx = {}   # Размер ячейки в метрах", modelling.dx).unwrap();
    writeln!(toml, "dy = {}   # Размер ячейки в метрах", modelling.dy).unwrap();
    writeln!(toml, "maxtime = {}   # Время моделирования в секундах", modelling.maxtime).unwrap();
    writeln!(toml).unwrap();
    
    // Boundary section (default PEC)
    writeln!(toml, "# Граничные условия").unwrap();
    writeln!(toml, "[boundary]").unwrap();
    for side in &["xmin", "xmax", "ymin", "ymax"] {
        writeln!(toml, "  [boundary.{}]", side).unwrap();
        writeln!(toml, "  type = \"PEC\"").unwrap();
        writeln!(toml, "  param1 = \"...\"").unwrap();
        writeln!(toml, "  param2 = \"...\"").unwrap();
        writeln!(toml).unwrap();
    }
    
    // Geometry section
    writeln!(toml, "# Геометрия моделируемой задачи").unwrap();
    writeln!(toml, "[geometry]").unwrap();
    for ((x1_norm, y1_norm), (x2_norm, y2_norm)) in rectangles {
        // Денормализуем координаты обратно в метры
        let x1 = x1_norm * modelling.sizex;
        let y1 = y1_norm * modelling.sizey;
        let x2 = x2_norm * modelling.sizex;
        let y2 = y2_norm * modelling.sizey;
        
        writeln!(toml, "  [[geometry.rectangle]]").unwrap();
        writeln!(toml, "  x1 = {}", x1).unwrap();
        writeln!(toml, "  y1 = {}", y1).unwrap();
        writeln!(toml, "  x2 = {}", x2).unwrap();
        writeln!(toml, "  y2 = {}", y2).unwrap();
        writeln!(toml, "  eps = 4.0").unwrap();
        writeln!(toml, "  mu = 1.0").unwrap();
        writeln!(toml, "  sigma = 0.01").unwrap();
        writeln!(toml, "  color = \"0, 0, 255\"").unwrap();
        writeln!(toml).unwrap();
    }
    
    // Probes section
    writeln!(toml, "# Список пробников").unwrap();
    writeln!(toml, "[probes]").unwrap();
    for (x_norm, y_norm) in probes {
        let x = x_norm * modelling.sizex;
        let y = y_norm * modelling.sizey;
        
        writeln!(toml, "  [[probes.probe]]").unwrap();
        writeln!(toml, "  x = {}", x).unwrap();
        writeln!(toml, "  y = {}", y).unwrap();
        writeln!(toml, "  color = \"0, 255, 255\"").unwrap();
        writeln!(toml).unwrap();
    }
    
    // Sources section
    writeln!(toml, "# Список источников возбуждения").unwrap();
    writeln!(toml, "[sources]").unwrap();
    for (x_norm, y_norm) in sources {
        let x = x_norm * modelling.sizex;
        let y = y_norm * modelling.sizey;
        
        writeln!(toml, "  [[sources.cylindrical]]").unwrap();
        writeln!(toml, "  x = {}", x).unwrap();
        writeln!(toml, "  y = {}", y).unwrap();
        writeln!(toml, "  type = \"sin\"").unwrap();
        writeln!(toml, "  param1 = \"...\"").unwrap();
        writeln!(toml, "  param2 = \"...\"").unwrap();
        writeln!(toml).unwrap();
    }
    
    toml
}

/// Сохраняет TOML-конфигурацию в файл
pub fn save_config_to_file(path: &Path, content: &str) -> std::io::Result<()> {
    std::fs::write(path, content)
}

/// Секция [modelling]
#[derive(Debug, Clone, Deserialize)]
pub struct Modelling {
    pub sizex: f32,
    pub sizey: f32,
    #[allow(unused)]
    pub dx: f32,
    #[allow(unused)]
    pub dy: f32,
    #[allow(unused)]
    pub maxtime: f32,
}

/// Описание одного прямоугольника
#[derive(Debug, Deserialize)]
pub struct RectangleDef {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    #[allow(unused)]
    pub eps: f32,
    #[allow(unused)]
    pub mu: f32,
    #[allow(unused)]
    pub sigma: f32,
    #[allow(unused)]
    pub color: String,
}

/// Граничное условие
#[derive(Debug, Deserialize)]
pub struct BoundaryCondition {
    #[serde(default)]
    #[allow(unused)]
    pub type_: Option<String>,
    #[serde(default)]
    #[allow(unused)]
    pub param1: Option<String>,
    #[serde(default)]
    #[allow(unused)]
    pub param2: Option<String>,
}

/// Секция [boundary]
#[derive(Debug, Deserialize, Default)]
pub struct BoundarySection {
    #[serde(default)]
    #[allow(unused)]
    pub xmin: Option<BoundaryCondition>,
    #[serde(default)]
    #[allow(unused)]
    pub xmax: Option<BoundaryCondition>,
    #[serde(default)]
    #[allow(unused)]
    pub ymin: Option<BoundaryCondition>,
    #[serde(default)]
    #[allow(unused)]
    pub ymax: Option<BoundaryCondition>,
}

/// Секция [geometry]
#[derive(Debug, Deserialize)]
pub struct GeometrySection {
    #[serde(default)]
    pub rectangle: Vec<RectangleDef>,
}

/// Зонд
#[derive(Debug, Deserialize)]
pub struct ProbeDef {
    pub x: f32,
    pub y: f32,
    #[allow(unused)]
    pub color: String,
}

/// Секция [probes]
#[derive(Debug, Deserialize, Default)]
pub struct ProbesSection {
    #[serde(default)]
    pub probe: Vec<ProbeDef>,
}

/// Цилиндрический источник
#[derive(Debug, Deserialize)]
pub struct CylindricalSourceDef {
    pub x: f32,
    pub y: f32,
    #[serde(default)]
    #[allow(unused)]
    pub type_: Option<String>,
    #[serde(default)]
    #[allow(unused)]
    pub param1: Option<String>,
    #[serde(default)]
    #[allow(unused)]
    pub param2: Option<String>,
}

/// Плоская волна
#[derive(Debug, Deserialize)]
pub struct PlaneWaveSourceDef {
    #[allow(unused)]
    pub x1: f32,
    #[allow(unused)]
    pub y1: f32,
    #[allow(unused)]
    pub x2: f32,
    #[allow(unused)]
    pub y2: f32,
    #[serde(default)]
    #[allow(unused)]
    pub type_: Option<String>,
    #[serde(default)]
    #[allow(unused)]
    pub param1: Option<String>,
    #[serde(default)]
    #[allow(unused)]
    pub param2: Option<String>,
}

/// Секция [sources]
#[derive(Debug, Deserialize, Default)]
pub struct SourcesSection {
    #[serde(default)]
    #[allow(unused)]
    pub cylindrical: Vec<CylindricalSourceDef>,
    #[serde(default)]
    #[allow(unused)]
    pub planewave: Vec<PlaneWaveSourceDef>,
}

/// Вся конфигурация
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    #[allow(unused)]
    pub description: Option<String>,
    pub modelling: Modelling,
    #[serde(default)]
    #[allow(unused)]
    pub boundary: BoundarySection,
    pub geometry: GeometrySection,
    #[serde(default)]
    pub probes: ProbesSection,
    #[serde(default)]
    pub sources: SourcesSection,
}

/// Загрузить и распарсить TOML
pub fn load_config(path: impl AsRef<std::path::Path>) -> anyhow::Result<Config> {
    let text = std::fs::read_to_string(path)?;
    let cfg = toml::from_str::<Config>(&text)?;
    Ok(cfg)
}

// Функции конвертации координат больше не нужны - конвертация происходит в main.rs

/// Типы объектов для проекта
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObjectType {
    Rectangle,
    Source,
    Probe,
}

/// Тип элемента в боковой панели
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarItemType {
    Rectangle,
    Source,
    Probe,
}

/// Тип активного диалога (зарезервирован для будущего использования модальных окон)
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActiveDialog {
    None,
    ProjectSettings,
    Rectangle,
    Source,
    Probe,
}

/// Объект проекта
#[derive(Debug, Clone)]
pub struct ProjectObject {
    pub object_type: ObjectType,
    pub x1: f32,
    pub y1: f32,
    pub x2: Option<f32>,  // Для прямоугольника
    pub y2: Option<f32>,  // Для прямоугольника
    pub eps: Option<f32>, // Диэлектрическая проницаемость (только для прямоугольника)
    pub mu: Option<f32>,  // Магнитная проницаемость (только для прямоугольника)
}

/// Параметры проекта
#[derive(Debug, Clone)]
pub struct ProjectSettings {
    #[allow(dead_code)]
    pub description: String,
    pub sizex: f32,
    pub sizey: f32,
    #[allow(dead_code)]
    pub dx: f32,
    #[allow(dead_code)]
    pub dy: f32,
    #[allow(dead_code)]
    pub maxtime: f32,
    pub objects: Vec<ProjectObject>,
}

impl Default for ProjectSettings {
    fn default() -> Self {
        Self {
            description: DEFAULT_DESCRIPTION.to_string(),
            sizex: 1.0,
            sizey: 1.0,
            dx: 0.01,
            dy: 0.01,
            maxtime: 1.0,
            objects: Vec::new(),
        }
    }
}

impl ProjectSettings {
    /// Создаёт параметры проекта на основе загруженной конфигурации.
    #[allow(dead_code)]
    pub fn from_config(cfg: &Config) -> Self {
        let mut objects = Vec::new();

        for rect in &cfg.geometry.rectangle {
            objects.push(ProjectObject {
                object_type: ObjectType::Rectangle,
                x1: rect.x1,
                y1: rect.y1,
                x2: Some(rect.x2),
                y2: Some(rect.y2),
                eps: Some(rect.eps),
                mu: Some(rect.mu),
            });
        }

        for probe in &cfg.probes.probe {
            objects.push(ProjectObject {
                object_type: ObjectType::Probe,
                x1: probe.x,
                y1: probe.y,
                x2: None,
                y2: None,
                eps: None,
                mu: None,
            });
        }

        for source in &cfg.sources.cylindrical {
            objects.push(ProjectObject {
                object_type: ObjectType::Source,
                x1: source.x,
                y1: source.y,
                x2: None,
                y2: None,
                eps: None,
                mu: None,
            });
        }

        Self {
            description: cfg
                .description
                .clone()
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| DEFAULT_DESCRIPTION.to_string()),
            sizex: cfg.modelling.sizex,
            sizey: cfg.modelling.sizey,
            dx: cfg.modelling.dx,
            dy: cfg.modelling.dy,
            maxtime: cfg.modelling.maxtime,
            objects,
        }
    }

    /// Преобразует текущие настройки в TOML-строку.
    /// Может использоваться для экспорта конфигурации в файл.
    #[allow(dead_code)]
    pub fn to_toml_string(&self) -> String {
        let mut toml_content = String::new();
        let description = if self.description.is_empty() {
            DEFAULT_DESCRIPTION
        } else {
            &self.description
        };

        writeln!(toml_content, "description = \"{}\"", description).unwrap();
        toml_content.push('\n');
        toml_content.push_str("[modelling]\n");
        writeln!(toml_content, "sizex = {}", self.sizex).unwrap();
        writeln!(toml_content, "sizey = {}", self.sizey).unwrap();
        writeln!(toml_content, "dx = {}", self.dx).unwrap();
        writeln!(toml_content, "dy = {}", self.dy).unwrap();
        writeln!(toml_content, "maxtime = {}", self.maxtime).unwrap();

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

        toml_content.push_str("\n[geometry]\n");
        for obj in self
            .objects
            .iter()
            .filter(|o| o.object_type == ObjectType::Rectangle)
        {
            if let (Some(x2), Some(y2), Some(eps), Some(mu)) = (obj.x2, obj.y2, obj.eps, obj.mu) {
                toml_content.push_str("  [[geometry.rectangle]]\n");
                writeln!(toml_content, "  x1 = {}", obj.x1).unwrap();
                writeln!(toml_content, "  y1 = {}", obj.y1).unwrap();
                writeln!(toml_content, "  x2 = {}", x2).unwrap();
                writeln!(toml_content, "  y2 = {}", y2).unwrap();
                writeln!(toml_content, "  eps = {}", eps).unwrap();
                writeln!(toml_content, "  mu = {}", mu).unwrap();
                toml_content.push_str("  sigma = 0.01\n");
                toml_content.push_str("  color = \"0, 0, 255\"\n");
            }
        }

        toml_content.push_str("\n[probes]\n");
        for obj in self
            .objects
            .iter()
            .filter(|o| o.object_type == ObjectType::Probe)
        {
            toml_content.push_str("  [[probes.probe]]\n");
            writeln!(toml_content, "  x = {}", obj.x1).unwrap();
            writeln!(toml_content, "  y = {}", obj.y1).unwrap();
            toml_content.push_str("  color = \"0, 255, 255\"\n");
        }

        toml_content.push_str("\n[sources]\n");
        for obj in self
            .objects
            .iter()
            .filter(|o| o.object_type == ObjectType::Source)
        {
            toml_content.push_str("  [[sources.cylindrical]]\n");
            writeln!(toml_content, "  x = {}", obj.x1).unwrap();
            writeln!(toml_content, "  y = {}", obj.y1).unwrap();
            toml_content.push_str("  type = \"sin\"\n");
            toml_content.push_str("  param1 = \"...\"\n");
            toml_content.push_str("  param2 = \"...\"\n");
        }

        toml_content
    }

    /// Проверяет, что координаты объекта находятся в пределах области моделирования
    #[allow(unused)]
    pub fn is_coordinate_valid(&self, x: f32, y: f32) -> bool {
        x >= 0.0 && x <= self.sizex && y >= 0.0 && y <= self.sizey
    }

    /// Проверяет, что координаты прямоугольника находятся в пределах области моделирования
    #[allow(unused)]
    pub fn is_rectangle_valid(&self, x1: f32, y1: f32, x2: f32, y2: f32) -> bool {
        self.is_coordinate_valid(x1, y1) && self.is_coordinate_valid(x2, y2)
    }

    /// Добавляет объект в проект
    #[allow(unused)]
    pub fn add_object(&mut self, object: ProjectObject) {
        self.objects.push(object);
    }

    /// Конвертирует все объекты проекта в нормализованные координаты
    #[allow(unused)]
    pub fn to_normalized_objects(&self) -> Vec<ProjectObject> {
        self.objects
            .iter()
            .map(|obj| ProjectObject {
                object_type: obj.object_type,
                x1: obj.x1 / self.sizex,
                y1: obj.y1 / self.sizey,
                x2: obj.x2.map(|x| x / self.sizex),
                y2: obj.y2.map(|y| y / self.sizey),
                eps: obj.eps,
                mu: obj.mu,
            })
            .collect()
    }
}
