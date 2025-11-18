use shadow_rs::shadow;

shadow!(build);

/// Get the current version string
/// 
/// Returns the version from build-time information:
/// - If a git tag exists, returns "refs/tags/{tag}"
/// - Otherwise, returns the package version from Cargo.toml
#[allow(clippy::const_is_empty)]
pub fn get_version() -> String {
    if !build::TAG.is_empty() {
        format!("refs/tags/{}", build::TAG)
    } else if !build::SHORT_COMMIT.is_empty() {
        format!("@{}", build::SHORT_COMMIT)
    } else {
        format!("refs/tags/{}", build::PKG_VERSION)
    }
}