use crate::version;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;
use tracing::{debug, error, info};

/// Update check related errors
#[derive(Error, Debug)]
pub enum UpdateCheckError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Version parsing failed: {0}")]
    VersionParseError(String),

    #[error("Invalid version response: {0}")]
    InvalidResponse(String),
}

/// Version information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionInfo {
    /// Version number
    pub version: String,
    /// Release date
    pub release_date: Option<String>,
    /// Release notes
    pub release_notes: Option<String>,
    /// Download URL
    pub download_url: Option<String>,
}

/// Update check result
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCheckResult {
    /// Whether update is available
    pub update_available: bool,
    /// Current version
    pub current_version: String,
    /// Latest version information
    pub latest_version: Option<VersionInfo>,
    /// Check time
    pub check_time: chrono::DateTime<chrono::Utc>,
}

/// Version checker
pub struct VersionChecker {
    /// HTTP client
    client: reqwest::Client,
    /// Version server URL
    version_url: String,
    /// Request timeout
    timeout: Duration,
}

impl Default for VersionChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl VersionChecker {
    /// Create a new version checker
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent(format!("RustFS/{}", get_current_version()))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            client,
            version_url: "https://version.rustfs.com/latest.json".to_string(),
            timeout: Duration::from_secs(10),
        }
    }

    /// Create version checker with custom configuration
    #[allow(dead_code)]
    pub fn with_config(url: String, timeout: Duration) -> Self {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .user_agent(format!("RustFS/{}", get_current_version()))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        Self {
            client,
            version_url: url,
            timeout,
        }
    }

    /// Check for updates
    pub async fn check_for_updates(&self) -> Result<UpdateCheckResult, UpdateCheckError> {
        let current_version = get_current_version();
        debug!("Checking for updates, current version: {}", current_version);

        // Send HTTP GET request to get latest version information
        let response = self.client.get(&self.version_url).timeout(self.timeout).send().await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("Version check request failed, status code: {}, response: {}", status, error_text);
            return Err(UpdateCheckError::InvalidResponse(format!(
                "HTTP status code: {status}, response: {error_text}"
            )));
        }

        // Parse response
        let response_bytes = response.bytes().await?;
        let version_info: VersionInfo = match serde_json::from_slice(&response_bytes) {
            Ok(v) => v,
            Err(e) => {
                let error_text = String::from_utf8_lossy(&response_bytes);
                error!("Version check request failed, response: {}", e);
                return Err(UpdateCheckError::InvalidResponse(format!(
                    "JSON parsing failed: {e}, response: {error_text}"
                )));
            }
        };

        debug!("Retrieved latest version information: {:?}", version_info);

        // Compare versions using version.rs functions
        let update_available = version::is_newer_version(&current_version, &version_info.version)
            .map_err(|e| UpdateCheckError::VersionParseError(e.to_string()))?;

        let result = UpdateCheckResult {
            update_available,
            current_version,
            latest_version: Some(version_info),
            check_time: chrono::Utc::now(),
        };

        if result.update_available {
            info!(
                "New version available: {} -> {}",
                result.current_version,
                result.latest_version.as_ref().unwrap().version
            );
        } else {
            info!("Current version is up to date: {}", result.current_version);
        }

        Ok(result)
    }
}

/// Get current version number
pub fn get_current_version() -> String {
    version::get_version()
}

/// Convenience function for async update checking
pub async fn check_updates() -> Result<UpdateCheckResult, UpdateCheckError> {
    let checker = VersionChecker::new();
    checker.check_for_updates().await
}

/// Update check with custom URL
#[allow(dead_code)]
pub async fn check_updates_with_url(url: String) -> Result<UpdateCheckResult, UpdateCheckError> {
    let checker = VersionChecker::with_config(url, Duration::from_secs(10));
    checker.check_for_updates().await
}
