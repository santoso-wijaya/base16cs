use clap::Parser;

use base16cs::colorschemes;

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

fn main() {
    let args = Cli::parse();
    let colorschemes = colorschemes::all();

    match colorschemes.get(args.palette.as_str()) {
        Some(&palette) => println!("{palette:#?}"),
        _ => {
            eprintln!("No colorscheme \"{0}\"", args.palette);
            std::process::exit(exitcode::CONFIG);
        }
    }
}
