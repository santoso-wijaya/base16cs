use crate::palette::{DerivedPalette, Palette};
use crate::template::PaletteRenderer;

use anyhow::{Context, Result};
use liquid::model::Value;

pub struct LiquidTemplate {
    path_str: String,
    /// A parsed Liquid template object.
    template: liquid::Template,
}

impl LiquidTemplate {
    /// Instantiate a LiquidTemplate by parsing the given file.
    /// The resulting template object will be ready for rendering given context.
    pub fn parse_file(path: &std::path::Path) -> Result<Self> {
        let parser = liquid::ParserBuilder::with_stdlib().build().unwrap();
        let path_str = String::from(path.to_str().unwrap());

        let template = parser.parse_file(path).with_context(|| {
            format!(
                "Could not parse as a Liquid template file: \"{}\"",
                path_str
            )
        })?;

        Ok(Self { path_str, template })
    }
}

impl PaletteRenderer for LiquidTemplate {
    fn render(&self, palette: &Palette, unroll_colors: bool) -> Result<String> {
        let derived_palette = DerivedPalette::from_palette(palette);

        let palette_obj = liquid::to_object(&derived_palette).with_context(|| {
            format!(
                "Could not serialize derived palette:\n{:?}",
                derived_palette
            )
        })?;
        let mut obj = liquid::Object::new();
        obj.insert("palette".into(), Value::Object(palette_obj));

        if unroll_colors {
            for derived_color in derived_palette.colors.iter() {
                let derived_color_obj = liquid::to_object(&derived_color).with_context(|| {
                    format!("Could not serialize derived color:\n{:?}", derived_color)
                })?;
                obj.insert(
                    derived_color.base.name.clone().into(),
                    Value::Object(derived_color_obj),
                );
            }
        }

        let rendered = self.template.render(&obj).with_context(|| {
            format!(
                "Could not render Liquid template \"{}\" with derived palette:\n{:?}",
                self.path_str, derived_palette
            )
        })?;

        Ok(rendered)
    }
}
