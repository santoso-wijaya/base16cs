use anyhow::{Context, Result};
use glob::glob;
use liquid::model::{to_value, KString};
use liquid::partials::{EagerCompiler, InMemorySource, PartialSource};
use liquid::{Object, Parser, ParserBuilder, Template};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use crate::palette::{DerivedPalette, Palette};
use crate::template::PaletteRenderer;

/// Represents a parsed Liquid template.
pub struct LiquidTemplate {
    /// Path to the Liquid template file.
    path: PathBuf,

    /// A parsed Liquid template object.
    template: Template,
}

type Partials = EagerCompiler<InMemorySource>;

impl LiquidTemplate {
    /// Instantiates a LiquidTemplate by parsing the given file.
    ///
    /// The resulting template object will be ready for rendering given context.
    ///
    /// * `path` - The path to the file to parse as a Liquid template.
    /// * `partials_dirs` - Paths to directories, if any, where template partials
    ///   can be searched for `{% render %}` or `{% include %}` directive tags.
    pub fn parse_file(path: &Path, partials_dirs: Vec<PathBuf>) -> Result<Self> {
        let parser = LiquidTemplate::build_parser(partials_dirs)?;

        let template = parser
            .parse_file(path)
            .with_context(|| format!("Could not parse Liquid template file: \"{:?}\"", path))?;

        Ok(Self {
            path: path.to_path_buf(),
            template,
        })
    }

    /// Builds a Liquid Parser and, optionally, preload it with template partials.
    fn build_parser(partials_dirs: Vec<PathBuf>) -> Result<Parser> {
        let partials = {
            let mut _partials = Partials::empty();
            for dirpath in partials_dirs {
                LiquidTemplate::parse_partials(dirpath.as_path(), &mut _partials)?;
            }
            _partials
        };

        let has_partials = !partials.names().is_empty();
        let parser = {
            let mut builder = ParserBuilder::with_stdlib();
            builder = if has_partials {
                builder.partials(partials)
            } else {
                builder
            };

            builder.build()?
        };

        Ok(parser)
    }

    /// Parses all `.liquid` files in the given directory, and insert them into the given partials.
    fn parse_partials(dirpath: &Path, partials: &mut Partials) -> Result<()> {
        let pattern = format!("{}/*.liquid", dirpath.to_str().unwrap());
        let matching_paths = glob(&pattern[..])?;

        for path in matching_paths.filter_map(core::result::Result::ok) {
            let basename = String::from(path.file_name().unwrap().to_str().unwrap());
            let filepath = String::from(path.to_str().unwrap());
            let contents = read_to_string(path)
                .with_context(|| format!("Could not read partial file: {}", filepath))?;

            // TODO: Handle the case when basename conflicts (with same-named file from a different
            // directory).
            partials.add(basename, contents);
        }

        Ok(())
    }
}

impl<const N: usize> PaletteRenderer<N> for LiquidTemplate {
    /// Renders this Liquid template with an injection of the given palette.
    ///
    /// The given `palette` will be converted into a `liquid::Object` value
    /// and injected as a variable in the rendered template with the key `"palette"`.
    ///
    /// * `palette` - a Palete to be injected when rendering this Liquid template.
    /// * `unroll_colors_hex` - if `true`, each color in the palette will be unrolled
    ///   as its sRGB hex string and keyed to said color's name.
    fn render(&self, palette: &Palette<N>, unroll_colors_hex: bool) -> Result<String> {
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
    use crate::palette::{Base16Palette, BaseColor};

    use rstest::*;
    use std::fs::write;
    use tempdir::TempDir;

    #[fixture]
    fn palette() -> Base16Palette {
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
            LiquidTemplate::parse_file(tempfile_path.as_path(), Vec::new())
        }

        /// Creates/overwrites a Liquid template file ("test.liquid") with the given contents.
        ///
        /// Returns a LiquidTemplate object that is parsed from said contents.
        ///
        /// Note that the Liquid parser used here is also configured to parse partials from the
        /// underlying temp directory.
        fn create_liquid_template_with_partials(
            &self,
            template_contents: &str,
        ) -> Result<LiquidTemplate> {
            self.create_liquid_template_with_multidir_partials(template_contents, Vec::new())
        }

        /// As above, but with multiple partials directories.
        fn create_liquid_template_with_multidir_partials(
            &self,
            template_contents: &str,
            additional_dirpaths: Vec<PathBuf>,
        ) -> Result<LiquidTemplate> {
            let tempfile_path = self.write_to_file(LIQUID_TEMPLATE_FILENAME, template_contents)?;
            let dirpaths = {
                let mut paths = Vec::new();
                paths.push(self.tmpdir.path().to_path_buf());
                paths.extend(additional_dirpaths);
                paths
            };

            LiquidTemplate::parse_file(tempfile_path.as_path(), dirpaths)
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

    #[fixture]
    fn tmpdir_2() -> TempDirFixture {
        TempDirFixture {
            tmpdir: TempDir::new("tests_2").unwrap(),
        }
    }

    #[rstest]
    fn test_render_palette(tmpdir: TempDirFixture, palette: Base16Palette) -> Result<()> {
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
    fn test_render_unroll_colors_hex(tmpdir: TempDirFixture, palette: Base16Palette) -> Result<()> {
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
    fn test_render_no_unroll_colors_hex(
        tmpdir: TempDirFixture,
        palette: Base16Palette,
    ) -> Result<()> {
        let liquid_template_content = r#"
            bg_0: #{{ bg_0 }}
            bg_1: #{{ bg_1 }}
        "#;

        let liquid_template = tmpdir.create_liquid_template_no_partials(liquid_template_content)?;

        let result = liquid_template.render(&palette, false);
        result.expect_err("Should not have been able to render template with unrolled color names");

        Ok(())
    }

    #[rstest]
    fn test_render_with_partials(tmpdir: TempDirFixture, palette: Base16Palette) -> Result<()> {
        let partial_content = "{{ title }}:\n";
        let liquid_template_content = r#"
          {%- render "common", title: "Palette" %}
            bg_0: #{{ bg_0 }}
            bg_1: #{{ bg_1 }}
        "#;
        let liquid_template_rendered = r#"Palette:

            bg_0: #fef3da
            bg_1: #f0e4cc
        "#;

        tmpdir.write_to_file("common.liquid", partial_content)?;

        let liquid_template =
            tmpdir.create_liquid_template_with_partials(liquid_template_content)?;

        let rendered = liquid_template.render(&palette, true)?;
        assert_eq!(liquid_template_rendered, rendered);

        Ok(())
    }

    #[rstest]
    fn test_render_with_partials_multiple_dirs(
        tmpdir: TempDirFixture,
        tmpdir_2: TempDirFixture,
        palette: Base16Palette,
    ) -> Result<()> {
        let partial_content_prepend = "{{ palette.name }}:\n";
        let liquid_template_content = r#"
          {%- include "prepend.liquid" %}
            bg_0: #{{ bg_0 }}
            bg_1: #{{ bg_1 }}
            {%- include "append.liquid" -%}
        "#;
        let partial_content_append = r#"
            fg_0: #{{ fg_0 }}
            fg_1: #{{ fg_1 }}
        "#;
        let liquid_template_rendered = r#"Selenized light:

            bg_0: #fef3da
            bg_1: #f0e4cc
            fg_0: #52666d
            fg_1: #384c52
        "#;

        tmpdir.write_to_file("prepend.liquid", partial_content_prepend)?;
        tmpdir_2.write_to_file("append.liquid", partial_content_append)?;

        let liquid_template = tmpdir.create_liquid_template_with_multidir_partials(
            liquid_template_content,
            vec![tmpdir_2.tmpdir.path().to_path_buf()],
        )?;

        let rendered = liquid_template.render(&palette, true)?;
        assert_eq!(liquid_template_rendered, rendered);

        Ok(())
    }
}
