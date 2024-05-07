pub mod liquid;

use crate::palette::Palette;

use anyhow::Result;

/// A trait for an object that can render itself given a base16 palette.
pub trait PaletteRenderer {
    /// Render this template (self), given a Palette object reference.
    ///
    /// The given Palette object will be converted into a `liquid::Object` value
    /// with the key `"palette"`.
    ///
    /// * `palette` - The given Palette object reference.
    /// * `unroll_names` - Whether to unroll the colors in `palette` as objects
    ///   with their own keys.
    fn render(&self, palette: &Palette, unroll_names: bool) -> Result<String>;
}
