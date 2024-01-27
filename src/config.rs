use ::config::{Config, ConfigError};

const DEFAULT_CONFIG_FILE: &str = "./config.toml";

pub fn get_config() -> Result<Config, ConfigError> {
    let arg = std::env::args().nth(1);

    let config_file = arg.as_deref().unwrap_or(DEFAULT_CONFIG_FILE);

    dbg!(&config_file);

    Config::builder()
        .add_source(config::File::with_name(config_file))
        .build()
}
