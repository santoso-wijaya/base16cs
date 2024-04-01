use color_space::Lab;

#[derive(Debug)]
pub struct BaseColor<'a> {
    pub name: &'a str,
    pub lab: Lab,
}

impl<'a> BaseColor<'a> {
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

#[derive(Debug)]
pub struct Palette<'a> {
    pub name: &'a str,
    // See: https://github.com/chriskempson/base16/blob/main/styling.md
    // In Base16 framework, [base00..base07] are monotone shades:
    // base00 - default background
    // base01 - lighter bg
    // base02 - selection bg
    // base03 - comments, invis
    // base04 - dark foreground
    // base05 - default foreground
    // base06 - light fg - often unused
    // base07 - light bg - often unused
    // [base08..base0f] are accent colors, with the following usage guidelines:
    // base08 - vars, diff deleted
    // base09 - ints, bools, consts
    // base0a - classes, search bg
    // base0b - strings, diff inserted
    // base0c - regex, escape chars
    // base0d - funcs, headings
    // base0e - keywords, diff changed
    // base0f - deprecated, embeds
    pub palette: [BaseColor<'a>; 16],
}
