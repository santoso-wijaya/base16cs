use crate::palette::Palette;

use anyhow::Result;

/// A trait for an object that can render itself given an N-color palette.
pub trait PaletteRenderer<const N: usize> {
    /// Render this template (self), given a Palette object reference.
    ///
    /// * `palette` - The given Palette object reference.
    /// * `unroll_colors_hex` - Whether to unroll the colors in `palette` as sRGB hex strings.
    fn render(&self, palette: &Palette<N>, unroll_colors_hex: bool) -> Result<String>;
}

#[cfg(feature = "liquid")]
pub mod liquid;
