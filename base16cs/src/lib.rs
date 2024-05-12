//! A library for defining a **palette** of **base colors** in canonical CIE
//! L\*a\*b\* colorspace values, and then deriving other colorspace values from
//! them.
//!
//! This library also provides serializers and deserializers for palettes, and
//! a template renderer (Liquid, by default) for injecting palette and color
//! variables into a template for renders.
//!
//! # Examples
//!
//! ## Serde
//!
//! let palette = Palette::new(
//!     "My Palette",
//!     [
//!         BaseColor::new("bg", 96, 0, 13),
//!         BaseColor::new("fg", 31, -6, -6),
//!     ]);
//!

mod palette;
mod serialize;
mod template;

pub use palette::Base16Colors;
pub use palette::Base16DerivedColors;
pub use palette::Base16DerivedPalette;
pub use palette::Base16Palette;
pub use palette::BaseColor;
pub use palette::DerivedColor;
pub use palette::DerivedPalette;
pub use palette::Palette;

pub use serialize::yaml;
pub use serialize::Serializable;

pub use template::liquid;
pub use template::PaletteRenderer;
