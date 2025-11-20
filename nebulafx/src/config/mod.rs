use clap::Parser;
use const_str::concat;
use std::string::ToString;
shadow_rs::shadow!(build);

#[allow(clippy::const_is_empty)]
const SHORT_VERSION: &str = {
    if !build::TAG.is_empty() {
        build::TAG
    } else if !build::SHORT_COMMIT.is_empty() {
        const_str::concat!("@", build::SHORT_COMMIT)
    } else {
        build::PKG_VERSION
    }
};

const LONG_VERSION: &str = {
    // Inline version string to work around const_str::concat! limitations with const variables
    const VERSION_STR: &str = {
        if !build::TAG.is_empty() {
            build::TAG
        } else if !build::SHORT_COMMIT.is_empty() {
            const_str::concat!("@", build::SHORT_COMMIT)
        } else {
            build::PKG_VERSION
        }
    };
    const_str::concat!(
        VERSION_STR, "\n",
        "build time   : ", build::BUILD_TIME, "\n",
        "build profile: ", build::BUILD_RUST_CHANNEL, "\n",
        "build os     : ", build::BUILD_OS, "\n",
        "rust version : ", build::RUST_VERSION, "\n",
        "rust channel : ", build::RUST_CHANNEL, "\n",
        "git branch   : ", build::BRANCH, "\n",
        "git commit   : ", build::COMMIT_HASH, "\n",
        "git tag      : ", build::TAG, "\n",
        "git status   :\n", build::GIT_STATUS_FILE,
    )
};

#[derive(Debug, Parser, Clone)]
#[command(version = SHORT_VERSION, long_version = LONG_VERSION)]
pub struct Opt {
    /// DIR points to a directory on a filesystem.
    #[arg(required = true, env = "NEUBULAFX_VOLUMES")]
    pub volumes: Vec<String>,

    /// bind to a specific ADDRESS:PORT, ADDRESS can be an IP or hostname
    #[arg(long, default_value_t = nebulafx_config::DEFAULT_ADDRESS.to_string(), env = "NEUBULAFX_ADDRESS")]
    pub address: String,

    /// Domain name used for virtual-hosted-style requests.
    #[arg(long, env = "NEUBULAFX_SERVER_DOMAINS")]
    pub server_domains: Vec<String>,

    /// Access key used for authentication.
    #[arg(long, default_value_t = nebulafx_config::DEFAULT_ACCESS_KEY.to_string(), env = "NEUBULAFX_ACCESS_KEY")]
    pub access_key: String,

    /// Secret key used for authentication.
    #[arg(long, default_value_t = nebulafx_config::DEFAULT_SECRET_KEY.to_string(), env = "NEUBULAFX_SECRET_KEY")]
    pub secret_key: String,

    // 已移除：前端独立运行，不再需要 console_enable 和 console_address 参数
    // Console API 端点（/nebulafx/console/*）始终通过主服务器提供

    /// Observability endpoint for trace, metrics and logs,only support grpc mode.
    #[arg(long, default_value_t = nebulafx_config::DEFAULT_OBS_ENDPOINT.to_string(), env = "NEUBULAFX_OBS_ENDPOINT")]
    pub obs_endpoint: String,

    /// tls path for nebulafx API and console.
    #[arg(long, env = "NEUBULAFX_TLS_PATH")]
    pub tls_path: Option<String>,

    #[arg(long, env = "NEUBULAFX_LICENSE")]
    pub license: Option<String>,

    #[arg(long, env = "NEUBULAFX_REGION")]
    pub region: Option<String>,

    /// Enable KMS encryption for server-side encryption
    #[arg(long, default_value_t = false, env = "NEUBULAFX_KMS_ENABLE")]
    pub kms_enable: bool,

    /// KMS backend type (local or vault)
    #[arg(long, default_value_t = String::from("local"), env = "NEUBULAFX_KMS_BACKEND")]
    pub kms_backend: String,

    /// KMS key directory for local backend
    #[arg(long, env = "NEUBULAFX_KMS_KEY_DIR")]
    pub kms_key_dir: Option<String>,

    /// Vault address for vault backend
    #[arg(long, env = "NEUBULAFX_KMS_VAULT_ADDRESS")]
    pub kms_vault_address: Option<String>,

    /// Vault token for vault backend
    #[arg(long, env = "NEUBULAFX_KMS_VAULT_TOKEN")]
    pub kms_vault_token: Option<String>,

    /// Default KMS key ID for encryption
    #[arg(long, env = "NEUBULAFX_KMS_DEFAULT_KEY_ID")]
    pub kms_default_key_id: Option<String>,
}

// lazy_static::lazy_static! {
//     pub(crate)  static ref OPT: OnceLock<Opt> = OnceLock::new();
// }

// pub fn init_config(opt: Opt) {
//     OPT.set(opt).expect("Failed to set global config");
// }

// pub fn get_config() -> &'static Opt {
//     OPT.get().expect("Global config not initialized")
// }
