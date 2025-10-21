use serde::Deserialize;
use std::path::PathBuf;

/// Диалог выбора TOML‑файла
pub fn select_toml_file() -> Option<PathBuf> {
    rfd::FileDialog::new()
        .add_filter("TOML", &["toml"])
        .set_title("Выберите файл конфигурации")
        .pick_file()
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

/// Конвертирует список прямоугольников из метров -> нормализованные координаты (0..1)
pub fn rectangles_m_to_normalized(
    rects_m: &[RectangleDef],
    sizex: f32,
    sizey: f32,
) -> Vec<((f32, f32), (f32, f32))> {
    rects_m
        .iter()
        .map(|r| {
            let nx1 = r.x1 / sizex;
            let ny1 = r.y1 / sizey;
            let nx2 = r.x2 / sizex;
            let ny2 = r.y2 / sizey;
            ((nx1, ny1), (nx2, ny2))
        })
        .collect()
}

/// Конвертирует список зондов из метров -> нормализованные координаты (0..1)
pub fn probes_m_to_normalized(
    probes_m: &[ProbeDef],
    sizex: f32,
    sizey: f32,
) -> Vec<(f32, f32)> {
    probes_m
        .iter()
        .map(|p| {
            let nx = p.x / sizex;
            let ny = p.y / sizey;
            (nx, ny)
        })
        .collect()
}

/// Конвертирует список цилиндрических источников из метров -> нормализованные координаты (0..1)
pub fn cylindrical_sources_m_to_normalized(
    sources_m: &[CylindricalSourceDef],
    sizex: f32,
    sizey: f32,
) -> Vec<(f32, f32)> {
    sources_m
        .iter()
        .map(|s| {
            let nx = s.x / sizex;
            let ny = s.y / sizey;
            (nx, ny)
        })
        .collect()
}

/// Конвертирует список плоских волн из метров -> нормализованные координаты (0..1)
#[allow(unused)]
pub fn planewave_sources_m_to_normalized(
    sources_m: &[PlaneWaveSourceDef],
    sizex: f32,
    sizey: f32,
) -> Vec<((f32, f32), (f32, f32))> {
    sources_m
        .iter()
        .map(|s| {
            let nx1 = s.x1 / sizex;
            let ny1 = s.y1 / sizey;
            let nx2 = s.x2 / sizex;
            let ny2 = s.y2 / sizey;
            ((nx1, ny1), (nx2, ny2))
        })
        .collect()
}

/// Типы объектов для проекта
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObjectType {
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
    pub x2: Option<f32>, // Для прямоугольника
    pub y2: Option<f32>, // Для прямоугольника
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
            description: String::new(),
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
        self.objects.iter().map(|obj| {
            ProjectObject {
                object_type: obj.object_type,
                x1: obj.x1 / self.sizex,
                y1: obj.y1 / self.sizey,
                x2: obj.x2.map(|x| x / self.sizex),
                y2: obj.y2.map(|y| y / self.sizey),
                eps: obj.eps,
                mu: obj.mu,
            }
        }).collect()
    }
}