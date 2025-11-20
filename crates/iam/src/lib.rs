

use std::sync::{Arc, OnceLock};

use crate::sys::IamSys;
use crate::error::{Error, Result};

pub mod error;
pub mod manager;
pub mod store;
pub mod utils;

pub mod sys;

// Database-related modules
pub mod entity;
pub mod repository;
pub mod migrations;
pub mod init;
pub mod types;

// Re-export repository types for convenience
pub use repository::{
    UserRepository, PolicyRepository, GroupRepository, 
    MappedPolicyRepository, UserIdentityRepository
};

// Re-export types for convenience (also available via store:: for backward compatibility)
pub use types::{GroupInfo, MappedPolicy, UserType};

static IAM_SYS: OnceLock<Arc<IamSys>> = OnceLock::new();

/// Initialize IAM system with database connection pool
pub async fn init_iam_sys(pool: sqlx::PgPool) -> Result<()> {
    let iam_sys = Arc::new(IamSys::new(pool));
    
    IAM_SYS.set(iam_sys)
        .map_err(|_| Error::other("IAM system already initialized"))?;
    
    Ok(())
}

/// Get the global IAM system instance
#[inline]
pub fn get() -> Result<Arc<IamSys>> {
    IAM_SYS.get().map(Arc::clone).ok_or(Error::IamSysNotInitialized)
}

/// Get the global IAM system instance (returns None if not initialized)
pub fn get_global_iam_sys() -> Option<Arc<IamSys>> {
    IAM_SYS.get().cloned()
}
