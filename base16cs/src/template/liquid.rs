use anyhow::{Context, Result};
use liquid::model::{to_value, KString};
use liquid::{Object, ParserBuilder, Template};
use std::path::{Path, PathBuf};

use crate::palette::{DerivedPalette, Palette};
use crate::template::PaletteRenderer;

pub struct LiquidTemplate {
    path: PathBuf,
    /// A parsed Liquid template object.
    template: Template,
}

impl LiquidTemplate {
    /// Instantiate a LiquidTemplate by parsing the given file.
    /// The resulting template object will be ready for rendering given context.
    pub fn parse_file(path: &Path) -> Result<Self> {
        let parser = ParserBuilder::with_stdlib().build().unwrap();

        let template = parser.parse_file(path).with_context(|| {
            format!("Could not parse as a Liquid template file: \"{:?}\"", path)
        })?;

        Ok(Self {
            path: path.to_path_buf(),
            template,
        })
    }
}

impl PaletteRenderer for LiquidTemplate {
    fn render(&self, palette: &Palette, unroll_colors_hex: bool) -> Result<String> {
        let derived_palette = DerivedPalette::from_palette(palette);

        let palette_obj_value = to_value(&derived_palette).with_context(|| {
            format!(
                "Could not serialize derived palette:\n{:?}",
                derived_palette
            )
        })?;
        let mut obj = Object::new();
        obj.insert("palette".into(), palette_obj_value);

        if unroll_colors_hex {
            for derived_color in derived_palette.colors.iter() {
                let srgb_hex_value = to_value(&derived_color.srgb_hex).with_context(|| {
                    format!(
                        "Could not serialize derived color's sRGB hex to value:\n{:?}",
                        derived_color
                    )
                })?;
                obj.insert(
                    KString::from_string(derived_color.base.name.clone()),
                    srgb_hex_value,
                );
            }
        }

        let rendered = self.template.render(&obj).with_context(|| {
            format!(
                "Could not render Liquid template \"{:?}\" with derived palette:\n{:?}",
                self.path, derived_palette
            )
        })?;

        Ok(rendered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::palette::BaseColor;

    use std::fs::write;
    use tempdir::TempDir;

    fn create_palette() -> Palette {
        Palette::new(
            "Selenized light",
            [
                // in Base16 framework:
                BaseColor::new("bg_0", 96, 0, 13), // base00 - default background
                BaseColor::new("bg_1", 91, 0, 13), // base01 - darker bg
                BaseColor::new("bg_2", 82, 0, 13), // base02 - selection bg
                BaseColor::new("dim_0", 62, -4, 1), // base03 - comments, invis
                BaseColor::new("fg_0", 42, -6, -6), // base04 - light foreground
                BaseColor::new("fg_1", 31, -6, -6), // base05 - default foreground
                BaseColor::new("unused_0", 28, -13, -13), // base06 - dark fg - unused
                BaseColor::new("unused_1", 23, -12, -12), // base07 - dark bg - unused
                BaseColor::new("red", 46, 66, 42), // base08 - vars, diff deleted
                BaseColor::new("orange", 52, 39, 52), // base09 - ints, bools, consts
                BaseColor::new("magenta", 52, 58, -16), // base0a - classes, search bg
                BaseColor::new("green", 54, -40, 58), // base0b - strings, diff inserted
                BaseColor::new("cyan", 57, -42, -4), // base0c - regex, escape chars
                BaseColor::new("blue", 46, 0, -60), // base0d - funcs, headings
                BaseColor::new("yellow", 59, 6, 71), // base0e - keywords, diff changed
                BaseColor::new("violet", 49, 32, -47), // base0f - deprecated, embeds
            ],
        )
    }

    fn create_liquid_template(tmpdir: &TempDir, template_content: &str) -> LiquidTemplate {
        let tempfile_path = write_to_file(tmpdir, template_content).unwrap();
        LiquidTemplate::parse_file(tempfile_path.as_path()).unwrap()
    }

    fn write_to_file(tmpdir: &TempDir, content: &str) -> Result<PathBuf> {
        let filepath = tmpdir.path().join("test-template.liquid");
        write(filepath.clone(), content)?;

        Ok(filepath)
    }

    #[test]
    fn test_render_palette() -> Result<()> {
        let tmpdir = TempDir::new("tests")?;

        let liquid_template_content = r#"
            {%- for color in palette.colors limit: 3 %}

                {{ color.base.name }}:
                - {{ color.base.lab.l }} {{ color.base.lab.a }} {{ color.base.lab.b }}
                - rgb({{ color.srgb.red }} {{ color.srgb.green }} {{ color.srgb.blue }})

            {%- endfor %}
        "#;
        let liquid_template_rendered = r#"

                bg_0:
                - 96 0 13
                - rgb(254 243 218)

                bg_1:
                - 91 0 13
                - rgb(240 228 204)

                bg_2:
                - 82 0 13
                - rgb(214 203 180)
        "#;

        let palette = create_palette();
        let liquid_template = create_liquid_template(&tmpdir, liquid_template_content);

        let rendered = liquid_template.render(&palette, true)?;

        assert_eq!(liquid_template_rendered, rendered);

        tmpdir.close()?;
        Ok(())
    }

    #[test]
    fn test_render_unroll_colors_hex() -> Result<()> {
        let tmpdir = TempDir::new("tests")?;

        let liquid_template_content = r#"
            bg_0: #{{ bg_0 }}
            bg_1: #{{ bg_1 }}
        "#;
        let liquid_template_rendered = r#"
            bg_0: #fef3da
            bg_1: #f0e4cc
        "#;

        let palette = create_palette();
        let liquid_template = create_liquid_template(&tmpdir, liquid_template_content);

        let rendered = liquid_template.render(&palette, true)?;

        assert_eq!(liquid_template_rendered, rendered);

        tmpdir.close()?;
        Ok(())
    }

    #[test]
    fn test_render_no_unroll_colors_hex() -> Result<()> {
        let tmpdir = TempDir::new("tests")?;

        let liquid_template_content = r#"
            bg_0: #{{ bg_0 }}
            bg_1: #{{ bg_1 }}
        "#;

        let palette = create_palette();
        let liquid_template = create_liquid_template(&tmpdir, liquid_template_content);

        let result = liquid_template.render(&palette, false);
        result.expect_err("Should not have been able to render template with unrolled color names");

        tmpdir.close()?;
        Ok(())
    }
}
