use sge::prelude::*;

// can parse JSON, TOML, and Ron
sge_include_assets!("assets/data", ASSETS);

fn main() {
    println!("{}", ASSETS.test.messages.hello);
}
