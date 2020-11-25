use std::fs;
use std::path::Path;

use bundler::bundle;

fn main() {
    let code = bundle("./");
    let dest_path = Path::new("./dist/bundle.rs");

    fs::write(&dest_path,code).unwrap();
}
