// Store trait - kept for backward compatibility during migration
// Types (UserType, MappedPolicy, GroupInfo) have been moved to types.rs
// TODO: Remove Store trait after IamCache and IamSys are refactored to use database repositories

use crate::error::Result;
use nebulafx_policy::{auth::UserIdentity, policy::PolicyDoc};
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashMap;

// Re-export types from types.rs for backward compatibility
// This allows external code to use `nebulafx_iam::store::UserType` etc.
pub use crate::types::{GroupInfo, MappedPolicy, UserType};

#[async_trait::async_trait]
pub trait Store: Clone + Send + Sync + 'static {
    fn has_watcher(&self) -> bool;
    async fn save_iam_config<Item: Serialize + Send>(&self, item: Item, path: impl AsRef<str> + Send) -> Result<()>;
    async fn load_iam_config<Item: DeserializeOwned>(&self, path: impl AsRef<str> + Send) -> Result<Item>;
    async fn delete_iam_config(&self, path: impl AsRef<str> + Send) -> Result<()>;

    async fn save_user_identity(&self, name: &str, user_type: UserType, item: UserIdentity, ttl: Option<usize>) -> Result<()>;
    async fn delete_user_identity(&self, name: &str, user_type: UserType) -> Result<()>;
    async fn load_user_identity(&self, name: &str, user_type: UserType) -> Result<UserIdentity>;

    async fn load_user(&self, name: &str, user_type: UserType, m: &mut HashMap<String, UserIdentity>) -> Result<()>;
    async fn load_users(&self, user_type: UserType, m: &mut HashMap<String, UserIdentity>) -> Result<()>;
    async fn load_secret_key(&self, name: &str, user_type: UserType) -> Result<String>;

    async fn save_group_info(&self, name: &str, item: GroupInfo) -> Result<()>;
    async fn delete_group_info(&self, name: &str) -> Result<()>;
    async fn load_group(&self, name: &str, m: &mut HashMap<String, GroupInfo>) -> Result<()>;
    async fn load_groups(&self, m: &mut HashMap<String, GroupInfo>) -> Result<()>;

    async fn save_policy_doc(&self, name: &str, item: PolicyDoc) -> Result<()>;
    async fn delete_policy_doc(&self, name: &str) -> Result<()>;
    async fn load_policy(&self, name: &str) -> Result<PolicyDoc>;
    async fn load_policy_doc(&self, name: &str, m: &mut HashMap<String, PolicyDoc>) -> Result<()>;
    async fn load_policy_docs(&self, m: &mut HashMap<String, PolicyDoc>) -> Result<()>;

    async fn save_mapped_policy(
        &self,
        name: &str,
        user_type: UserType,
        is_group: bool,
        item: MappedPolicy,
        ttl: Option<usize>,
    ) -> Result<()>;
    async fn delete_mapped_policy(&self, name: &str, user_type: UserType, is_group: bool) -> Result<()>;
    async fn load_mapped_policy(
        &self,
        name: &str,
        user_type: UserType,
        is_group: bool,
        m: &mut HashMap<String, MappedPolicy>,
    ) -> Result<()>;
    async fn load_mapped_policies(
        &self,
        user_type: UserType,
        is_group: bool,
        m: &mut HashMap<String, MappedPolicy>,
    ) -> Result<()>;

    // load_all is no longer needed as we query database directly
    // async fn load_all(&self, cache: &Cache) -> Result<()>;
}
