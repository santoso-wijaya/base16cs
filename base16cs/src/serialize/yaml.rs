use crate::palette::Palette;
use crate::serialize::Serializable;

use anyhow::{Context, Result};

impl Serializable for Palette {
    fn serialize(&self) -> Result<String> {
        serde_yaml::to_string(self)
            .with_context(|| format!("Could not serialize palette to YAML:\n{:?}", self))
    }
}

impl Palette {
    pub fn from_yaml(yaml: &str) -> Result<Palette> {
        serde_yaml::from_str(yaml)
            .with_context(|| format!("Could not deserialize YAML to palette:\n{}", yaml))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::palette::BaseColor;

    fn create_palette() -> Palette {
        Palette::new(
            "Selenized light",
            [
                // in Base16 framework:
                BaseColor::new("bg_0", 96, 0, 13),        // base00 - default background
                BaseColor::new("bg_1", 91, 0, 13),        // base01 - darker bg
                BaseColor::new("bg_2", 82, 0, 13),        // base02 - selection bg
                BaseColor::new("dim_0", 62, -4, 1),       // base03 - comments, invis
                BaseColor::new("fg_0", 42, -6, -6),       // base04 - light foreground
                BaseColor::new("fg_1", 31, -6, -6),       // base05 - default foreground
                BaseColor::new("unused_0", 28, -13, -13), // base06 - dark fg - unused
                BaseColor::new("unused_1", 23, -12, -12), // base07 - dark bg - unused
                BaseColor::new("red", 46, 66, 42),        // base08 - vars, diff deleted
                BaseColor::new("orange", 52, 39, 52),     // base09 - ints, bools, consts
                BaseColor::new("magenta", 52, 58, -16),   // base0a - classes, search bg
                BaseColor::new("green", 54, -40, 58),     // base0b - strings, diff inserted
                BaseColor::new("cyan", 57, -42, -4),      // base0c - regex, escape chars
                BaseColor::new("blue", 46, 0, -60),       // base0d - funcs, headings
                BaseColor::new("yellow", 59, 6, 71),      // base0e - keywords, diff changed
                BaseColor::new("violet", 49, 32, -47),    // base0f - deprecated, embeds
            ],
        )
    }

    const PALETTE_YAML: &str = r#"name: Selenized light
colors:
- name: bg_0
  lab:
    l: 96.0
    a: 0.0
    b: 13.0
- name: bg_1
  lab:
    l: 91.0
    a: 0.0
    b: 13.0
- name: bg_2
  lab:
    l: 82.0
    a: 0.0
    b: 13.0
- name: dim_0
  lab:
    l: 62.0
    a: -4.0
    b: 1.0
- name: fg_0
  lab:
    l: 42.0
    a: -6.0
    b: -6.0
- name: fg_1
  lab:
    l: 31.0
    a: -6.0
    b: -6.0
- name: unused_0
  lab:
    l: 28.0
    a: -13.0
    b: -13.0
- name: unused_1
  lab:
    l: 23.0
    a: -12.0
    b: -12.0
- name: red
  lab:
    l: 46.0
    a: 66.0
    b: 42.0
- name: orange
  lab:
    l: 52.0
    a: 39.0
    b: 52.0
- name: magenta
  lab:
    l: 52.0
    a: 58.0
    b: -16.0
- name: green
  lab:
    l: 54.0
    a: -40.0
    b: 58.0
- name: cyan
  lab:
    l: 57.0
    a: -42.0
    b: -4.0
- name: blue
  lab:
    l: 46.0
    a: 0.0
    b: -60.0
- name: yellow
  lab:
    l: 59.0
    a: 6.0
    b: 71.0
- name: violet
  lab:
    l: 49.0
    a: 32.0
    b: -47.0
"#;

    #[test]
    fn test_yaml_serialize() -> Result<()> {
        let yaml = create_palette().serialize()?;
        assert_eq!(yaml, PALETTE_YAML);

        Ok(())
    }

    #[test]
    fn test_yaml_deserialize() -> Result<()> {
        let yaml = String::from(PALETTE_YAML);
        let palette = Palette::from_yaml(yaml.as_str())?;
        assert_eq!(palette, create_palette());

        Ok(())
    }
}
