use base16cs::colorschemes;
use base16cs::palette;

fn main() {
    for (name, palette) in colorschemes::all().into_iter() {
        print!("\"{name}\": ");
        print_palette(palette);
    }
}

fn print_palette(palette: &palette::Palette) {
    println!("{palette:#?}");
}
