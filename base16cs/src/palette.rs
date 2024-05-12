use arrayvec::ArrayVec;
use num_traits::cast::AsPrimitive;
use palette::rgb::Srgb;
use palette::{lab::Lab, IntoColor};
use serde::{Deserialize, Serialize};

/// A base color in its canonical form.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BaseColor {
    /// This base color's canonical name.
    pub name: String,

    /// This base color's canonical CIE L*a*b* values.
    pub lab: Lab,
}

impl BaseColor {
    #[inline]
    pub fn new<S, F>(name: S, l: F, a: F, b: F) -> BaseColor
    where
        S: Into<String>,
        F: AsPrimitive<f32>,
    {
        BaseColor {
            name: name.into(),
            lab: Lab::new(l.as_(), a.as_(), b.as_()),
        }
    }
}

/// A palette is a collection of base colors (in their canonical forms only).
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Palette<const N: usize> {
    /// This palette's name.
    pub name: String,

    /// The base colors in this palette.
    #[serde(with = "serde_arrays")]
    pub colors: [BaseColor; N],
}

/// A palette with 16 base colors.
///
/// See: https://github.com/chriskempson/base16/blob/main/styling.md
/// In Base16 framework, [base00..base07] are monotone shades:
/// base00 - default background
/// base01 - lighter bg
/// base02 - selection bg
/// base03 - comments, invis
/// base04 - dark foreground
/// base05 - default foreground
/// base06 - light fg - often unused
/// base07 - light bg - often unused
/// [base08..base0f] are accent colors, with the following usage guidelines:
/// base08 - vars, diff deleted
/// base09 - ints, bools, consts
/// base0a - classes, search bg
/// base0b - strings, diff inserted
/// base0c - regex, escape chars
/// base0d - funcs, headings
/// base0e - keywords, diff changed
/// base0f - deprecated, embeds
pub type Base16Palette = Palette<16>;
pub type Base16Colors = [BaseColor; 16];

impl<const N: usize> Palette<N> {
    #[inline]
    pub fn new<S>(name: S, colors: [BaseColor; N]) -> Palette<N>
    where
        S: Into<String>,
    {
        Palette {
            name: name.into(),
            colors,
        }
    }
}

/// A color with derived forms (sRGB values derived from its canonical CIE Lab).
#[derive(Serialize, Debug)]
pub struct DerivedColor<'a> {
    /// This color's canonical form, as well as its name.
    pub base: &'a BaseColor,

    /// This color's derived sRGB values form.
    pub srgb: Srgb<u8>,

    /// This color's derived sRGB values, in stringified hex form ("{:x}").
    pub srgb_hex: String,
}

impl<'a> From<&'a BaseColor> for DerivedColor<'a> {
    fn from(base: &'a BaseColor) -> Self {
        let srgb: Srgb = base.lab.into_color();
        let srgb_u8: Srgb<u8> = srgb.into_format();
        let srgb_hex = format!("{:x}", srgb_u8);

        Self {
            base,
            srgb: srgb_u8,
            srgb_hex,
        }
    }
}

/// Like Palette, a DerivedPalette contains an array of DerivedColors.
#[derive(Serialize, Debug)]
pub struct DerivedPalette<'a, const N: usize> {
    /// A reference to the base palette's name.
    pub name: &'a str,

    /// The derived colors in this palette.
    #[serde(with = "serde_arrays")]
    pub colors: [DerivedColor<'a>; N],
}

pub type Base16DerivedPalette<'a> = DerivedPalette<'a, 16>;
pub type Base16DerivedColors<'a> = [DerivedColor<'a>; 16];

impl<'a, const N: usize> From<&'a Palette<N>> for DerivedPalette<'a, N> {
    fn from(base_palette: &'a Palette<N>) -> Self {
        let colors: [DerivedColor<'a>; N] = base_palette
            .colors
            .iter()
            .map(DerivedColor::from)
            .collect::<ArrayVec<_, N>>()
            .into_inner()
            .unwrap();

        Self {
            name: &base_palette.name,
            colors,
        }
    }
}
