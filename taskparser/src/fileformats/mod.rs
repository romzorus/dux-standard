pub mod json;
pub mod yaml;

pub enum ContentFormat {
    Interactive,
    Json,
    Pythonscript,
    Shellscript,
    Yaml
}