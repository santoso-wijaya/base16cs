pub mod liquid;

use crate::palette::Palette;

/// A trait for an object that can render itself given a base16 palette.
pub trait PaletteRenderer {
    fn render(&self, palette: &Palette) -> String;
}
