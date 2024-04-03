use crate::palette::Palette;
use crate::template::PaletteRenderer;

use anyhow::{Context, Result};
use liquid::{ParserBuilder, Template};

pub struct LiquidTemplate {
    /// A parsed Liquid template object.
    template: Template,
}

impl LiquidTemplate {
    /// Instantiate a LiquidTemplate by parsing the given file.
    /// The resulting template object will be ready for rendering given context.
    pub fn parse_file(path: &std::path::Path) -> Result<Self> {
        let parser = ParserBuilder::with_stdlib().build().unwrap();
        let template = parser.parse_file(path).with_context(|| {
            format!(
                "Could not parse as a Liquid template file: \"{}\"",
                path.to_str().unwrap()
            )
        })?;

        Ok(Self { template })
    }
}

impl PaletteRenderer for LiquidTemplate {
    fn render(&self, palette: &Palette) -> String {
        // TODO: Implement
        String::from("")
    }
}
