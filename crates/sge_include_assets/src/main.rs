sge_include_assets::sge_include_assets!("test", TEST_FOLDER);

fn main() {
    let msg = TEST_FOLDER.a.toml.top_level;
    println!("{}", msg);
}
