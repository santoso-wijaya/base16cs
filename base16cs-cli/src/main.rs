use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use base16cs::palette::DerivedPalette;
use base16cs::palette::Palette;
use base16cs::serialize::Serializable;
use base16cs::template::liquid::LiquidTemplate;
use base16cs::template::PaletteRenderer;

/// Load a Liquid template file and render it with the values of a colorscheme
/// palette.
#[derive(Parser)]
struct Cli {
    /// The path to the yaml file of the palette to load.
    #[arg(short = 'p', long = "palette")]
    palette: PathBuf,
    /// The path(s) to directories for loading Liquid partials.
    #[clap(short = 'd', long = "partial_dir", num_args = 0..)]
    partials_dirs: Vec<PathBuf>,
    /// Whether to unroll `color` objects into hex strings with their names as Liquid keys.
    #[arg(short = 'u', long = "unroll_colors_hex")]
    unroll_colors_hex: bool,
    /// The path to the template file to read.
    /// Without a template file, print the derived palette yaml and exit.
    #[arg(short = 't', long = "template")]
    template: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let palette_yaml = std::fs::read_to_string(args.palette.as_path())?;
    let palette = Palette::from_yaml(palette_yaml.as_str())?;

    let output = match args.template {
        None => print_derived_palette(&palette),
        Some(template_path) => render_template(
            template_path,
            args.partials_dirs,
            &palette,
            args.unroll_colors_hex,
        ),
    }?;

    println!("{}", output);

    Ok(())
}

fn print_derived_palette(palette: &Palette) -> Result<String> {
    let derived_palette = DerivedPalette::from_palette(palette);
    derived_palette.serialize()
}

fn render_template(
    path: PathBuf,
    partials_dirs: Vec<PathBuf>,
    palette: &Palette,
    unroll_colors_hex: bool,
) -> Result<String> {
    let template = LiquidTemplate::parse_file(path.as_path(), partials_dirs)?;
    template.render(palette, unroll_colors_hex)
}
