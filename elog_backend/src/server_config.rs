use serde::Deserialize;
use config::ConfigError;

const CONFIG_FILE_NAME: &str = "elog.yml";

// This lazy_static allows to have some sort of global singleton
// to access from other modules to the same configuration without creating new instances
lazy_static! {
    pub static ref ELOG_CONFIG: ElogConfig = ElogConfig::new();
}


#[derive(Deserialize, Debug, Clone)]
pub struct ElogDataBase {
    pub db_url: String,
    pub pool_size: u32,
}

impl Default for ElogDataBase {
    fn default() -> Self {
        ElogDataBase {
            db_url: String::from("mysql://user:pass@ip:port/db_name"),
            pool_size: 6
        }
    }

}

#[derive(Deserialize, Debug, Clone)]
pub struct ElogTokenConfig {
    pub jwt_secret: String,
    pub duration: u16
}

impl Default for ElogTokenConfig {
    fn default() -> Self {
        ElogTokenConfig {
            jwt_secret: String::from("elog-super-secret-key"),
            duration: 60
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ElogConfig {
    pub ip_address: String,
    pub server_port: u16,
    pub log_type: String,
    pub database: ElogDataBase,
    pub token: ElogTokenConfig
}

impl Default for ElogConfig {
    fn default() -> Self {
        ElogConfig {
            ip_address: String::from("0.0.0.0"),
            server_port: 8090,
            log_type: String::from("actix_web=DEBUG"),
            database: ElogDataBase::default(),
            token: ElogTokenConfig::default()
        }
    }
}

impl ElogConfig {

    // New instance of ElogConfig
    // gets current filepath with file name included "elog.yml"
    // then tries to get configuration
    // if something happens will get Default config for all properties
    pub fn new() -> Self {
        let file_path = std::env::current_dir().unwrap();
        let full_config_path = format!(
            "{}/{}",
            file_path.as_os_str().to_str().unwrap(),
            CONFIG_FILE_NAME
        );

        match Self::get_configuration(full_config_path.as_str()) {
            Ok(config) => config,
            Err(_error) => {
                println!("\n- Default config will be used due to:\n\t{}\n", _error);
                Self::default()
            }
        }
    }

    fn get_configuration(file_path: &str) -> Result<Self, ConfigError> {
        use config::{Config, File};

        let mut config = Config::new();
        match config.merge(File::with_name(file_path)) {
            Ok(_) => config.try_into(),
            Err(_error) => {
                println!("\n- Default config will be used due to:\n\t{}\n", _error);
                Ok(Self::default())
            }
        }
    }
}

#[cfg(test)]
mod settings_test {

    use super::*;

    #[test]
    fn get_default_db_config() {
        let expected = ElogDataBase {
            db_url: String::from("mysql://user:pass@ip:port/db_name"),
            pool_size: 6
        };

        let actual = ElogConfig::new();
        assert_eq!(actual.database.db_url, expected.db_url);
        assert_eq!(actual.database.pool_size, expected.pool_size);
    }

    #[test]
    fn get_default_token_config() {
        let expected = ElogTokenConfig {
            jwt_secret: String::from("elog-super-secret-key"),
            duration: 60
        };

        let actual = ElogConfig::new();
        assert_eq!(actual.token.jwt_secret, expected.jwt_secret);
        assert_eq!(actual.token.duration, expected.duration);
    }

    #[test]
    fn get_default_elog_config() {
        let expected = ElogConfig {
            ip_address: String::from("0.0.0.0"),
            server_port: 8090,
            log_type: String::from("actix_web=debug"),
            database: ElogDataBase::default(),
            token: ElogTokenConfig::default()
        };

        let actual = ElogConfig::new();
        assert_eq!(actual.ip_address, expected.ip_address);
        assert_eq!(actual.server_port, expected.server_port);
        assert_eq!(actual.log_type, expected.log_type);
    }
}
