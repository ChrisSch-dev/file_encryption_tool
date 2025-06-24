use config::{Config, File, FileFormat};

pub fn load_settings(path: &str) -> Result<Config, config::ConfigError> {
    Config::builder()
        .add_source(File::new(path, FileFormat::Toml))
        .build()
}