//! # Rendering a template with injected palette
//!
//! This module contains the `PaletteRenderer` interface: it represents a live
//! template that can render itself when injected with a palette. This is useful
//! for generating a colorscheme file, for instance.
//!
//! This crate is compiled with a Liquid template renderer by default.
//!
//! ## Example: Liquid template injected with unrolled color names from a palette
//!
//! Let's say we have the following Liquid template file `selenized.md.liquid`:
//!
//! ```liquid
//! # Selenized Light
//!
//! | name | sRGB (#hex) |
//! |------|-------------|
//! | bg_0 | #{{ bg_0 }} |
//! | bg_1 | #{{ bg_1 }} |
//! | fg_0 | #{{ fg_0 }} |
//! | fg_1 | #{{ fg_1 }} |
//! ```
//!
//! Given the following palette (in serialized yaml file):
//!
//! ```yaml
//! name: Selenized light
//! colors:
//! - name: bg_0
//!   lab:
//!     l: 96.0
//!     a: 0.0
//!     b: 13.0
//! - name: bg_1
//!   lab:
//!     l: 91.0
//!     a: 0.0
//!     b: 13.0
//! - name: bg_2
//!   lab:
//!     l: 82.0
//!     a: 0.0
//!     b: 13.0
//! - name: dim_0
//!   lab:
//!     l: 62.0
//!     a: -4.0
//!     b: 1.0
//! - name: fg_0
//!   lab:
//!     l: 42.0
//!     a: -6.0
//!     b: -6.0
//! - name: fg_1
//!   lab:
//!     l: 31.0
//!     a: -6.0
//!     b: -6.0
//! ... <snip>
//! ```
//!
//! We can deserialize the palette and inject it into a template for rendering:
//!
//! ```rust,no_run
//! use base16cs::{Base16Palette, RenderOptions, PaletteRenderer};
//! use base16cs::liquid::LiquidTemplate;
//! # use std::path::Path;
//!
//! # let yaml = "";
//! let palette = Base16Palette::from_yaml(&yaml).unwrap();
//! let template = LiquidTemplate::parse_file(Path::new("selenized.md.liquid"), Vec::new()).unwrap();
//!
//! let rendered = template.render(&palette, RenderOptions { unroll_colors_hex: true }).unwrap();
//!
//! assert_eq!(rendered, r#"# Selenized Light
//!
//! | name | sRGB (#hex) |
//! |------|-------------|
//! | bg_0 | #fef3da |
//! | bg_1 | #f0e4cc |
//! | fg_0 | #52666d |
//! | fg_1 | #384c52 |
//! "#);
//! ```
//!
//! Note that palette derivation to sRGB colorspace is handled internally by the
//! palette renderer.

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
