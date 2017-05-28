extern crate drop_shadows;
#[macro_use]
extern crate clap;

use drop_shadows::*;
use std::path::Path;

fn main() {

    match run() {
        Err(err) => {
            println!("Error: {}", err);
            std::process::exit(1);
        }
        Ok(_) => std::process::exit(0),
    }
}

macro_rules! set_config {
    ($config:expr, $matches:expr, $name:ident, $arg_type:ty) => (
        $config.$name = $matches
            .value_of(stringify!($name))
            .and_then(|x| x.parse::<$arg_type>().ok())
            .unwrap_or($config.$name);
)
}

fn run() -> ShadowResult<()> {

    let matches = clap_app!(drop_shadows =>
            (version: env!("CARGO_PKG_VERSION"))
            (author: env!("CARGO_PKG_AUTHORS"))
            (about: env!("CARGO_PKG_DESCRIPTION"))
            (@arg IN_FILE: +required "The input image file")
            (@arg OUT_FILE: +required "The output image file")
            (@arg margin: -m --margin +takes_value "The size of the added border margin")
            (@arg blur_margin: -b --blur_margin +takes_value "The size of the blur")
            (@arg blur_amount: -a --blur_amount +takes_value "The intensity of the blur")
            )
            .get_matches();

    let mut config = Config::default();

    set_config!(config, matches, margin, u32);
    set_config!(config, matches, blur_margin, u32);
    set_config!(config, matches, blur_amount, f32);

    let in_file = matches.value_of_os("IN_FILE").unwrap();
    let out_file = matches.value_of_os("OUT_FILE").unwrap();

    DropShadowBuilder::from_file(Path::new(in_file))
        .config(&config)
        .apply()?
        .to_file(Path::new(out_file))?;

    Ok(())

}
