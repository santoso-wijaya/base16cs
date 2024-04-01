use crate::palette::{BaseColor, Palette};

// monotone shades
pub const BASE03: BaseColor = BaseColor::new("base03", 15, -12, -12);
pub const BASE02: BaseColor = BaseColor::new("base02", 20, -12, -12);
pub const BASE01: BaseColor = BaseColor::new("base01", 45, -7, -7);
pub const BASE00: BaseColor = BaseColor::new("base00", 50, -7, -7);
pub const BASE0: BaseColor = BaseColor::new("base0", 60, -6, -3);
pub const BASE1: BaseColor = BaseColor::new("base1", 65, -5, -2);
pub const BASE2: BaseColor = BaseColor::new("base2", 92, -00, 10);
pub const BASE3: BaseColor = BaseColor::new("base3", 97, 00, 10);
// accent colors
pub const RED: BaseColor = BaseColor::new("red", 50, 65, 45);
pub const ORANGE: BaseColor = BaseColor::new("orange", 50, 50, 55);
pub const MAGENTA: BaseColor = BaseColor::new("magenta", 60, 65, -5);
pub const GREEN: BaseColor = BaseColor::new("green", 60, -20, 65);
pub const CYAN: BaseColor = BaseColor::new("cyan", 60, -35, -5);
pub const BLUE: BaseColor = BaseColor::new("blue", 55, -10, -45);
pub const YELLOW: BaseColor = BaseColor::new("yellow", 60, 10, 65);
pub const VIOLET: BaseColor = BaseColor::new("violet", 50, 15, -45);

pub const SOLARIZED_DARK: Palette = Palette {
    palette: [
        // in Base16 framework:
        BASE03,  // base00 - default background
        BASE02,  // base01 - lighter bg
        BASE01,  // base02 - selection bg
        BASE00,  // base03 - comments, invis
        BASE0,   // base04 - dark foreground
        BASE1,   // base05 - default foreground
        BASE2,   // base06 - light fg
        BASE3,   // base07 - light bg
        RED,     // base08 - vars, diff deleted
        ORANGE,  // base09 - ints, bools, consts
        MAGENTA, // base0a - classes, search bg
        GREEN,   // base0b - strings, diff inserted
        CYAN,    // base0c - regex, escape chars
        BLUE,    // base0d - funcs, headings
        YELLOW,  // base0e - keywords, diff changed
        VIOLET,  // base0f - deprecated, embeds
    ],
};

pub const SOLARIZED_LIGHT: Palette = Palette {
    palette: [
        // in Base16 framework:
        BASE3,   // base00 - default background
        BASE2,   // base01 - darker bg
        BASE1,   // base02 - selection bg
        BASE0,   // base03 - comments, invis
        BASE00,  // base04 - light foreground
        BASE01,  // base05 - default foreground
        BASE02,  // base06 - dark fg
        BASE03,  // base07 - dark bg
        RED,     // base08 - vars, diff deleted
        ORANGE,  // base09 - ints, bools, consts
        MAGENTA, // base0a - classes, search bg
        GREEN,   // base0b - strings, diff inserted
        CYAN,    // base0c - regex, escape chars
        BLUE,    // base0d - funcs, headings
        YELLOW,  // base0e - keywords, diff changed
        VIOLET,  // base0f - deprecated, embeds
    ],
};
