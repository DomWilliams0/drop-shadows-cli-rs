extern crate drop_shadows;

use drop_shadows::*;
use std::path::Path;

fn main() {
    let _ = DropShadowBuilder::from_file(Path::new("/tmp/image.png"));
}
