use crate::palette::{BaseColor, Palette};

pub const SELENIZED_DARK: Palette = Palette {
    name: "Selenized dark",
    palette: [
        // in Base16 framework:
        BaseColor::new("bg_0", 23, -12, -12), // base00 - default background
        BaseColor::new("bg_1", 28, -13, -13), // base01 - lighter bg
        BaseColor::new("bg_2", 36, -13, -13), // base02 - selection bg
        BaseColor::new("dim_0", 56, -8, -6),  // base03 - comments, invis
        BaseColor::new("fg_0", 75, -5, -2),   // base04 - dark foreground
        BaseColor::new("fg_1", 85, -5, -2),   // base05 - default foreground
        BaseColor::new("*", 91, 0, 13),       // base06 - light fg - unused
        BaseColor::new("*", 96, 0, 13),       // base07 - light bg - unused
        BaseColor::new("red", 60, 63, 40),    // base08 - vars, diff deleted
        BaseColor::new("orange", 67, 37, 50), // base09 - ints, bools, consts
        BaseColor::new("magenta", 66, 55, -15), // base0a - classes, search bg
        BaseColor::new("green", 69, -38, 55), // base0b - strings, diff inserted
        BaseColor::new("cyan", 73, -40, -4),  // base0c - regex, escape chars
        BaseColor::new("blue", 60, 0, -57),   // base0d - funcs, headings
        BaseColor::new("yellow", 75, 6, 68),  // base0e - keywords, diff changed
        BaseColor::new("violet", 64, 30, -45), // base0f - deprecated, embeds
    ],
};

pub const SELENIZED_LIGHT: Palette = Palette {
    name: "Selenized light",
    palette: [
        // in Base16 framework:
        BaseColor::new("bg_0", 96, 0, 13), // base00 - default background
        BaseColor::new("bg_1", 91, 0, 13), // base01 - darker bg
        BaseColor::new("bg_2", 82, 0, 13), // base02 - selection bg
        BaseColor::new("dim_0", 62, -4, 1), // base03 - comments, invis
        BaseColor::new("fg_0", 42, -6, -6), // base04 - light foreground
        BaseColor::new("fg_1", 31, -6, -6), // base05 - default foreground
        BaseColor::new("*", 28, -13, -13), // base06 - dark fg - unused
        BaseColor::new("*", 23, -12, -12), // base07 - dark bg - unused
        BaseColor::new("red", 46, 66, 42), // base08 - vars, diff deleted
        BaseColor::new("orange", 52, 39, 52), // base09 - ints, bools, consts
        BaseColor::new("magenta", 52, 58, -16), // base0a - classes, search bg
        BaseColor::new("green", 54, -40, 58), // base0b - strings, diff inserted
        BaseColor::new("cyan", 57, -42, -4), // base0c - regex, escape chars
        BaseColor::new("blue", 46, 0, -60), // base0d - funcs, headings
        BaseColor::new("yellow", 59, 6, 71), // base0e - keywords, diff changed
        BaseColor::new("violet", 49, 32, -57), // base0f - deprecated, embeds
    ],
};
