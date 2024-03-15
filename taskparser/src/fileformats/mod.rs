pub mod json;
pub mod toml;
pub mod yaml;

pub enum ContentFormat {
    Interactive,
    Json,
    Pythonscript,
    Shellscript,
    Toml,
    Yaml
}