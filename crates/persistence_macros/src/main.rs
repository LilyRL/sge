#[persistence::persistent]
struct Config {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
