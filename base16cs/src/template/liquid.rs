use anyhow::{Context, Result};
use glob::glob;
use liquid::model::{to_value, KString};
use liquid::partials::{EagerCompiler, InMemorySource};
use liquid::{Object, Parser, ParserBuilder, Template};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use crate::palette::{DerivedPalette, Palette};
use crate::template::PaletteRenderer;

pub struct LiquidTemplate {
    path: PathBuf,
    /// A parsed Liquid template object.
    template: Template,
}

type Partials = EagerCompiler<InMemorySource>;

impl LiquidTemplate {
    /// Instantiate a LiquidTemplate by parsing the given file.
    ///
    /// The resulting template object will be ready for rendering given context.
    ///
    /// * `path` - The path to the file to parse as a Liquid template.
    /// * `partials_dir` - Optional path to a directory where template partials
    ///   can be searched for `{% render %}`.
    pub fn parse_file(path: &Path, partials_dir: Option<&Path>) -> Result<Self> {
        let parser = LiquidTemplate::build_parser(partials_dir)?;

        let template = parser
            .parse_file(path)
            .with_context(|| format!("Could not parse Liquid template file: \"{:?}\"", path))?;

        Ok(Self {
            path: path.to_path_buf(),
            template,
        })
    }

    /// Build a Liquid Parser and, optionally, preload it with template partials.
    fn build_parser(partials_dir: Option<&Path>) -> Result<Parser> {
        let partials_result: Option<Result<Partials>> =
            partials_dir.map(LiquidTemplate::parse_partials);

        let mut parser_builder = ParserBuilder::with_stdlib();
        parser_builder = match partials_result {
            None => parser_builder,
            Some(result) => parser_builder.partials(result?),
        };

        Ok(parser_builder.build()?)
    }

    fn parse_partials(dirpath: &Path) -> Result<Partials> {
        let mut partials = Partials::empty();

        let pattern = format!("{}/*.liquid", dirpath.to_str().unwrap());
        let matching_paths = glob(&pattern[..])?;

        for path in matching_paths.filter_map(core::result::Result::ok) {
            let basename = String::from(path.file_name().unwrap().to_str().unwrap());
            let filepath = String::from(path.to_str().unwrap());
            let contents = read_to_string(path)
                .with_context(|| format!("Could not read partial file: {}", filepath))?;

            partials.add(basename, contents);
        }

        Ok(partials)
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

        // Insert each color's sRGB hex string as values keyed to the color's names.
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

    use rstest::*;
    use std::fs::write;
    use tempdir::TempDir;

    #[fixture]
    fn palette() -> Palette {
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

    struct TempDirFixture {
        tmpdir: TempDir,
    }

    const LIQUID_TEMPLATE_FILENAME: &str = "test.liquid";

    impl TempDirFixture {
        /// Creates/overwrites a Liquid template file ("test.liquid") with the given contents.
        ///
        /// Returns a LiquidTemplate object that is parsed from said contents.
        ///
        /// Note that the Liquid parser used here does not compile any partials.
        fn create_liquid_template_no_partials(
            &self,
            template_contents: &str,
        ) -> Result<LiquidTemplate> {
            let tempfile_path = self.write_to_file(LIQUID_TEMPLATE_FILENAME, template_contents)?;
            LiquidTemplate::parse_file(tempfile_path.as_path(), None)
        }

        /// Writes the given UTF-8 contents string into a file in this TempDir fixture.
        ///
        /// Returns a full filepath to the newly created file.
        fn write_to_file(&self, filename: &str, contents: &str) -> Result<PathBuf> {
            let filepath = self.tmpdir.path().join(filename);
            write(filepath.clone(), contents)?;

            Ok(filepath)
        }
    }

    #[fixture]
    fn tmpdir() -> TempDirFixture {
        TempDirFixture {
            tmpdir: TempDir::new("tests").unwrap(),
        }
    }

    #[rstest]
    fn test_render_palette(tmpdir: TempDirFixture, palette: Palette) -> Result<()> {
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

        let liquid_template = tmpdir.create_liquid_template_no_partials(liquid_template_content)?;

        let rendered = liquid_template.render(&palette, true)?;

        assert_eq!(liquid_template_rendered, rendered);

        Ok(())
    }

    #[rstest]
    fn test_render_unroll_colors_hex(tmpdir: TempDirFixture, palette: Palette) -> Result<()> {
        let liquid_template_content = r#"
            bg_0: #{{ bg_0 }}
            bg_1: #{{ bg_1 }}
        "#;
        let liquid_template_rendered = r#"
            bg_0: #fef3da
            bg_1: #f0e4cc
        "#;

        let liquid_template = tmpdir.create_liquid_template_no_partials(liquid_template_content)?;

        let rendered = liquid_template.render(&palette, true)?;

        assert_eq!(liquid_template_rendered, rendered);

        Ok(())
    }

    #[rstest]
    fn test_render_no_unroll_colors_hex(tmpdir: TempDirFixture, palette: Palette) -> Result<()> {
        let liquid_template_content = r#"
            bg_0: #{{ bg_0 }}
            bg_1: #{{ bg_1 }}
        "#;

        let liquid_template = tmpdir.create_liquid_template_no_partials(liquid_template_content)?;

        let result = liquid_template.render(&palette, false);
        result.expect_err("Should not have been able to render template with unrolled color names");

        Ok(())
    }
}
