pub mod liquid;

use crate::palette::Palette;

use anyhow::Result;

/// A trait for an object that can render itself given a base16 palette.
pub trait PaletteRenderer {
    fn render(&self, palette: &Palette) -> Result<String>;
}
