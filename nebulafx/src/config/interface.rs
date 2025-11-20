use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: Option<ServerConfig>,
    pub database: Option<DatabaseConfig>,
    pub storage: Option<StorageConfig>,
    pub tls: Option<TlsConfig>,
    pub observability: Option<ObservabilityConfig>,
    pub license: Option<LicenseConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub name: Option<String>,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub server_domains: Option<Vec<String>>,
    pub region: Option<String>,
    pub volumes: Option<String>,
    pub cors_allowed_origins: Option<String>,
    pub console_cors_allowed_origins: Option<String>,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub root_user: Option<String>,
    pub root_password: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
    pub charset: Option<String>,
    pub parse_time: Option<bool>,
    pub loc: Option<String>,
    pub logger_level: Option<String>,
    pub auto_migrate: Option<bool>,
    pub connection: Option<DatabaseConnectionConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConnectionConfig {
    pub timeout: Option<String>,
    pub max_retries: Option<u32>,
    pub retry_interval: Option<String>,
    pub max_idle_connections: Option<u32>,
    pub max_open_connections: Option<u32>,
    pub conn_max_idle_time: Option<String>,
    pub conn_max_lifetime: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StorageConfig {
    pub base_path: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TlsConfig {
    pub path: Option<String>,
    pub key_file: Option<String>,
    pub cert_file: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ObservabilityConfig {
    pub endpoint: Option<String>,
    pub trace_endpoint: Option<String>,
    pub metric_endpoint: Option<String>,
    pub log_endpoint: Option<String>,
    pub use_stdout: Option<bool>,
    pub sample_ratio: Option<f64>,
    pub meter_interval: Option<u64>,
    pub service_name: Option<String>,
    pub service_version: Option<String>,
    pub environment: Option<String>,
    pub logger_level: Option<String>,
    pub log_stdout_enabled: Option<bool>,
    pub log_directory: Option<String>,
    pub log_filename: Option<String>,
    pub log_rotation_size_mb: Option<u64>,
    pub log_rotation_time: Option<String>,
    pub log_keep_files: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LicenseConfig {
    pub content: Option<String>,
}

