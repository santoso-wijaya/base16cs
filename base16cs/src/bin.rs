use base16cs::colorschemes;
use base16cs::palette;
use base16cs::serialize::Serializable;

use anyhow::Result;

fn main() -> Result<()> {
    for (name, palette) in colorschemes::all().into_iter() {
        print!("\"{name}\": ");
        print_palette_yaml(palette)?
    }
    Ok(())
}

fn print_palette_yaml(palette: &palette::Palette) -> Result<()> {
    println!("{}", palette.serialize()?);
    Ok(())
}
