

use crate::error::{Error, Result};
use manager::IamCache;
use nebulafx_ecstore::store::ECStore;
use std::sync::{Arc, OnceLock};
use store::object::ObjectStore;
use sys::IamSys;
use tracing::{debug, instrument};

pub mod cache;
pub mod error;
pub mod manager;
pub mod store;
pub mod utils;

pub mod sys;

static IAM_SYS: OnceLock<Arc<IamSys<ObjectStore>>> = OnceLock::new();

#[instrument(skip(ecstore))]
pub async fn init_iam_sys(ecstore: Arc<ECStore>) -> Result<()> {
    debug!("init iam system");
    let s = IamCache::new(ObjectStore::new(ecstore)).await;

    IAM_SYS.get_or_init(move || IamSys::new(s).into());
    Ok(())
}

#[inline]
pub fn get() -> Result<Arc<IamSys<ObjectStore>>> {
    IAM_SYS.get().map(Arc::clone).ok_or(Error::IamSysNotInitialized)
}

pub fn get_global_iam_sys() -> Option<Arc<IamSys<ObjectStore>>> {
    IAM_SYS.get().cloned()
}
