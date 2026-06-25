use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::PgConnectOptions;

/// Application settings that are needed to start application
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

/// Database settings for postgresql database
#[derive(Deserialize, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

/// Application info prot and host (the host ex: localhoset and the port Ex: 8022)
#[derive(Debug, Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
}

impl DatabaseSettings {
    /// Returns the connection string for postgresql connection
    pub fn connect_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .database(&self.database_name)
            .port(self.port)
            .password(&self.password.expose_secret())
            .username(&self.username)
    }
}

/// Reads the configuration from the configuration directory or environment variables and returns it
pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to get current directory path");
    let config_directory = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("failed to parse APP_ENVIRONMENT");
    let settings = config::Config::builder()
        .add_source(
            config::File::with_name(
                config_directory
                    .join("base.toml")
                    .to_string_lossy()
                    .as_ref(),
            )
            .required(true),
        )
        .add_source(
            config::File::with_name(
                config_directory
                    .join(format!("{}.toml", environment.as_str()))
                    .to_string_lossy()
                    .as_ref(),
            )
            .required(true),
        );

    settings.build()?.try_deserialize::<Settings>()
}

/// Enum to decide at which environment application is running local or production
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}
