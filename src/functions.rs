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

/// Секция [geometry]
#[derive(Debug, Deserialize)]
pub struct GeometrySection {
    #[serde(default)]
    pub rectangle: Vec<RectangleDef>,
}

/// Вся конфигурация
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    #[allow(dead_code)]
    pub description: Option<String>,
    pub modelling: Modelling,
    pub geometry: GeometrySection,
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