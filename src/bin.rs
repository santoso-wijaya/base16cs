use base16cs::colorschemes::{selenized, solarized};
use base16cs::palette;

fn main() {
    print_palette(&solarized::SOLARIZED_DARK);
    print_palette(&solarized::SOLARIZED_LIGHT);
    print_palette(&selenized::SELENIZED_DARK);
    print_palette(&selenized::SELENIZED_LIGHT);
}

fn print_palette(palette: &palette::Palette) {
    println!("{palette:#?}");
}
