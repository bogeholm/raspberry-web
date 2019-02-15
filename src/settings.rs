use config::{Config, ConfigError, File};
use clap::ArgMatches;

// https://github.com/mehcode/config-rs/tree/master/examples/hierarchical-env
#[derive(Debug, Serialize, Deserialize)]
pub struct Webserver {
    pub hostname: String,
    pub port: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Database {
    pub database_url: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GpioConfig {
    pub gpios_in_use: Option<Vec<i32>>,
    pub gpios_mode_output: Option<Vec<i32>>,
    pub gpios_mode_input: Option<Vec<i32>>,
    pub gpios_level_low: Option<Vec<i32>>,
    pub gpios_level_high: Option<Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub webserver: Webserver,
    pub database: Database,
    pub gpioconfig: GpioConfig
}

impl Settings {
    pub fn new(args: ArgMatches) -> Result<Self, ConfigError> {
        let mut settings = Config::new();

        // Configuration file for development
        #[cfg(debug_assertions)]
        let default_config_file = "config/configuration.toml";

        // Configuration file for installed binary
        #[cfg(not(debug_assertions))]
        let default_config_file = "/usr/local/rpiweb/configuration.toml";

        // Let user supply a config file, or use default
        let config_file = args.value_of("config-file")
            .unwrap_or(default_config_file);    
    
        settings.merge(File::with_name(config_file))?;
        settings.try_into()
    }
}