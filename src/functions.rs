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

/// Конвертирует список прямоугольников из метров → пиксели
pub fn rectangles_m_to_px(
    rects_m: &[RectangleDef],
    canvas_w: f32,
    canvas_h: f32,
    sizex: f32,
    sizey: f32,
) -> Vec<((f32, f32), (f32, f32))> {
    let sx = canvas_w / sizex;
    let sy = canvas_h / sizey;
    rects_m
        .iter()
        .map(|r| {
            let px1 = r.x1 * sx;
            let py1 = r.y1 * sy;
            let px2 = r.x2 * sx;
            let py2 = r.y2 * sy;
            ((px1, py1), (px2, py2))
        })
        .collect()
}