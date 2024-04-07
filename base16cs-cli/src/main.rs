use anyhow::Result;
use base16cs::template::PaletteRenderer;
use clap::Parser;

use base16cs::palette::Palette;
use base16cs::template::liquid::LiquidTemplate;

/// Load a Liquid template file and render it with the values of a colorscheme
/// palette.
#[derive(Parser)]
struct Cli {
    /// The path to the yaml file of the palette to load
    #[arg(short = 'p', long = "palette")]
    palette: std::path::PathBuf,
    /// The path to the template file to read
    #[arg(short = 't', long = "template")]
    template: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let palette_yaml = std::fs::read_to_string(args.palette.as_path())?;
    let palette = Palette::from_yaml(palette_yaml.as_str())?;

    let path = args.template.as_path();
    let template = LiquidTemplate::parse_file(path)?;
    let rendered = template.render(&palette)?;

    println!("{}", rendered);

    Ok(())
}
