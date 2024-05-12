//! # `base16cs`
//!
//! A library for defining a **palette** of **base colors** in canonical CIE
//! L\*a\*b\* colorspace values, and then deriving other colorspace values from
//! them.
//!
//! This library also provides serializers and deserializers for palettes, and
//! a template renderer (Liquid, by default) for injecting palette and color
//! variables into a template for renders.
//!
//! ## Examples
//!
//! ```rust
//! use base16cs::{Palette, BaseColor, DerivedPalette};
//!
//! // Define a canonical palette
//! let palette = Palette::new(
//!     "My Palette",
//!     [
//!         BaseColor::new("bg", 96, 0, 13),
//!         BaseColor::new("fg", 31, -6, -6),
//!     ]);
//!
//! // Derive with computed sRGB values
//! let derived_palette = DerivedPalette::from(&palette);
//! assert_eq!(derived_palette.name, "My Palette");
//!
//! let derived_colors = &derived_palette.colors;
//! assert_eq!(derived_colors[0].base, &palette.colors[0]);
//! assert_eq!(derived_colors[1].base, &palette.colors[1]);
//!
//! assert_eq!(derived_colors[0].srgb_hex, "fef3da");
//! assert_eq!(derived_colors[1].srgb_hex, "384c52");
//!
//! // Serialize (to YAML)
//! use base16cs::Serializable;
//!
//! let serialized = derived_palette.serialize().unwrap();
//! assert_eq!(serialized, r#"name: My Palette
//! colors:
//! - base:
//!     name: bg
//!     lab:
//!       l: 96.0
//!       a: 0.0
//!       b: 13.0
//!   srgb:
//!     red: 254
//!     green: 243
//!     blue: 218
//!   srgb_hex: fef3da
//! - base:
//!     name: fg
//!     lab:
//!       l: 31.0
//!       a: -6.0
//!       b: -6.0
//!   srgb:
//!     red: 56
//!     green: 76
//!     blue: 82
//!   srgb_hex: 384c52
//! "#);
//! ```

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
