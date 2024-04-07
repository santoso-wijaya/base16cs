pub mod selenized;
pub mod solarized;

use crate::palette::Palette;
use std::collections::HashMap;

/// return all colorschemes keyed by their names
pub fn all<'a>() -> HashMap<&'a str, &'a Palette<'a>> {
    let all_colorschemes = vec![
        &solarized::SOLARIZED_DARK,
        &solarized::SOLARIZED_LIGHT,
        &selenized::SELENIZED_DARK,
        &selenized::SELENIZED_LIGHT,
    ];

    let mut m = HashMap::new();
    for palette in all_colorschemes {
        m.entry(palette.name).or_insert(palette);
    }

    m
}
