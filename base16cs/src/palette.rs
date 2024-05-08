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

type Base16Colors = [BaseColor; 16];

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Palette {
    pub name: String,
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
    pub colors: Base16Colors,
}

impl Palette {
    #[inline]
    pub fn new<S>(name: S, colors: Base16Colors) -> Palette
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

impl<'a> DerivedColor<'a> {
    pub fn from_base_color(base: &'a BaseColor) -> Self {
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

type Base16DerivedColors<'a> = [DerivedColor<'a>; 16];

#[derive(Serialize, Debug)]
pub struct DerivedPalette<'a> {
    pub name: &'a String,
    pub colors: Base16DerivedColors<'a>,
}

impl<'a> DerivedPalette<'a> {
    pub fn from_palette(base_palette: &'a Palette) -> Self {
        let colors: Base16DerivedColors = base_palette
            .colors
            .iter()
            .map(DerivedColor::from_base_color)
            .collect::<ArrayVec<_, 16>>()
            .into_inner()
            .unwrap();

        Self {
            name: &base_palette.name,
            colors,
        }
    }
}
