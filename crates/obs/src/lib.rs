

//! # NebulaFX Observability
//!
//! provides tools for system and service monitoring
//!
//! ## feature mark
//! - `default`: default monitoring function
//! - `gpu`: gpu monitoring function
//! - `full`: includes all functions
//!
//! to enable gpu monitoring add in cargo toml
//!
//! ```toml
//! # using gpu monitoring
//! nebulafx-obs = { version = "0.1.0", features = ["gpu"] }
//!
//! # use all functions
//! nebulafx-obs = { version = "0.1.0", features = ["full"] }
//! ```
///
/// ## Usage
///
/// ```no_run
/// use nebulafx_obs::init_obs;
///
/// # #[tokio::main]
/// # async fn main() {
/// #   let _guard = match init_obs(None).await {
/// #         Ok(g) => g,
/// #         Err(e) => {
/// #             panic!("Failed to initialize observability: {:?}", e);
/// #         }
/// #     };
/// #   // Application logic here
/// #   {
/// #       // Simulate some work
/// #       tokio::time::sleep(std::time::Duration::from_secs(2)).await;
/// #       println!("Application is running...");
/// #   }
/// #   // Guard will be dropped here, flushing telemetry data
/// # }
/// ```
mod config;
mod error;
mod global;
mod metrics;
mod recorder;
mod system;
mod telemetry;

pub use config::*;
pub use error::*;
pub use global::*;
pub use metrics::*;
pub use recorder::*;
pub use system::SystemObserver;
pub use telemetry::OtelGuard;
