use crate::palette::Palette;

use anyhow::Result;

/// Rendering options.
pub struct RenderOptions {
    /// Whether to unroll the colors in `palette` as sRGB hex strings.
    pub unroll_colors_hex: bool,
}

/// A trait for an object that can render itself given an N-color palette.
pub trait PaletteRenderer<const N: usize> {
    /// Render this template (self), given a Palette object reference.
    ///
    /// * `palette` - The given Palette object reference.
    /// * `options` - Rendering options for this template and its injected palette.
    fn render(&self, palette: &Palette<N>, options: RenderOptions) -> Result<String>;
}

#[cfg(feature = "liquid")]
pub mod liquid;
