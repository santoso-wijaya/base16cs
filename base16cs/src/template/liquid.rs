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
