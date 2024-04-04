use crate::colorspace::{LabDef, RgbDef};
use color_space::{Lab, Rgb, ToRgb};
use serde::{Deserialize, Serialize};

/// A base color in its canonical form.
#[derive(Serialize, Deserialize, Debug)]
pub struct BaseColor {
    /// This base color's name.
    pub name: &'static str,

    /// This base color's canonical L*a*b* values.
    #[serde(with = "LabDef")]
    pub lab: Lab,
}

impl BaseColor {
    /// Creates a BaseColor as a compile-time constant.
    #[inline]
    pub const fn new(name: &'static str, l: i32, a: i32, b: i32) -> BaseColor {
        BaseColor {
            name,
            lab: Lab {
                l: l as f64,
                a: a as f64,
                b: b as f64,
            },
        }
    }

    pub fn to_derived_color(&self) -> DerivedColor {
        DerivedColor {
            base: self,
            rgb: self.lab.to_rgb(),
        }
    }
}

/// A color with derived forms.
#[derive(Serialize, Debug)]
pub struct DerivedColor<'a> {
    /// This color's canonical form, as well as its name.
    pub base: &'a BaseColor,

    /// This color's derived sRGB values form.
    #[serde(with = "RgbDef")]
    pub rgb: Rgb,
}

#[derive(Serialize, Debug)]
pub struct Palette {
    pub name: &'static str,
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
    pub palette: [BaseColor; 16],
}

impl Palette {
    //// Creates a palette with derived colors.
    // pub fn to_derived_palette(&self) -> DerivedPalette {
    //     // TODO: Implement
    // }
}

#[derive(Serialize, Debug)]
pub struct DerivedPalette<'a> {
    pub name: String,
    pub palette: [DerivedColor<'a>; 16],
}
