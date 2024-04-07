use crate::colorspace::{LabDef, RgbDef};
use arrayvec::ArrayVec;
use color_space::{Lab, Rgb, ToRgb};
use serde::{Deserialize, Serialize};

/// A base color in its canonical form.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct BaseColor<'a> {
    /// This base color's name.
    pub name: &'a str,

    /// This base color's canonical L*a*b* values.
    #[serde(with = "LabDef")]
    pub lab: Lab,
}

impl<'a> BaseColor<'a> {
    /// Creates a BaseColor as a compile-time constant.
    #[inline]
    pub const fn new(name: &'a str, l: i32, a: i32, b: i32) -> BaseColor {
        BaseColor {
            name,
            lab: Lab {
                l: l as f64,
                a: a as f64,
                b: b as f64,
            },
        }
    }
}

/// A color with derived forms.
#[derive(Serialize, Debug)]
pub struct DerivedColor<'a> {
    /// This color's canonical form, as well as its name.
    pub base: &'a BaseColor<'a>,

    /// This color's derived sRGB values form.
    #[serde(with = "RgbDef")]
    pub rgb: Rgb,
}

impl<'a> DerivedColor<'a> {
    pub fn from_base_color(base: &'a BaseColor) -> Self {
        Self {
            base,
            rgb: base.lab.to_rgb(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Palette<'a> {
    pub name: &'a str,
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
    pub colors: [BaseColor<'a>; 16],
}

#[derive(Serialize, Debug)]
pub struct DerivedPalette<'a> {
    pub name: String,
    pub colors: [DerivedColor<'a>; 16],
}

impl<'a> DerivedPalette<'a> {
    pub fn from_palette(base_palette: &'a Palette) -> Self {
        let colors: [DerivedColor; 16] = base_palette
            .colors
            .iter()
            .map(DerivedColor::from_base_color)
            .collect::<ArrayVec<_, 16>>()
            .into_inner()
            .unwrap();

        Self {
            name: String::from(base_palette.name),
            colors,
        }
    }
}
