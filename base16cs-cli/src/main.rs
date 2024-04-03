use anyhow::Result;
use base16cs::template::PaletteRenderer;
use clap::Parser;

use base16cs::colorschemes;
use base16cs::template::liquid::LiquidTemplate;

/// Load a Liquid template file and render it with the values of a colorschemes
/// palette.
#[derive(Parser)]
struct Cli {
    /// The name of the palette to load
    #[arg(short = 'p', long = "palette")]
    palette: String,
    /// The path to the template file to read
    #[arg(short = 't', long = "template")]
    template: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let colorschemes = colorschemes::all();

    let palette = match colorschemes.get(args.palette.as_str()) {
        Some(&palette) => palette,
        _ => {
            eprintln!("No colorscheme \"{0}\"", args.palette);
            std::process::exit(exitcode::CONFIG);
        }
    };

    let path = args.template.as_path();
    let template = LiquidTemplate::parse_file(path)?;
    let rendered = template.render(palette)?;

    println!("{}", rendered);

    Ok(())
}
