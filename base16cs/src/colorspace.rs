use color_space::{Lab, Rgb};
use serde::{Deserialize, Serialize};

// See: https://serde.rs/remote-derive.html
#[derive(Serialize, Deserialize)]
#[serde(remote = "Lab")]
pub struct LabDef {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Rgb")]
pub struct RgbDef {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}
