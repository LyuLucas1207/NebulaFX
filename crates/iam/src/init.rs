use crate::migrations::ALL_MIGRATIONS;
use crate::repository::user::UserRepository;
use sqlx::PgPool;
use tracing::info;

/// Initialize database tables
/// 
/// # Arguments
/// * `pool` - PostgreSQL connection pool
/// 
/// # Returns
/// Returns `Ok(())` on success, or an error if table creation fails
pub async fn init_database(pool: &PgPool) -> Result<(), sqlx::Error> {
    info!("Creating IAM database tables...");
    
    for migration in ALL_MIGRATIONS {
        sqlx::query(migration)
            .execute(pool)
            .await?;
    }
    
    info!("All IAM database tables created successfully");
    Ok(())
}

/// Initialize root user (ID = 1) if it doesn't exist
/// 
/// # Arguments
/// * `pool` - PostgreSQL connection pool
/// * `access_key` - Root user access key
/// * `secret_key` - Root user secret key
/// 
/// # Returns
/// Returns `Ok(())` on success, or an error if initialization fails
pub async fn init_root_user(
    pool: &PgPool,
    access_key: &str,
    secret_key: &str,
) -> Result<(), sqlx::Error> {
    // Check if root user (ID = 1) already exists
    let exists = UserRepository::exists_by_id(pool, 1).await?;
    
    if !exists {
        info!("Creating root user with ID=1...");
        UserRepository::create_root_user(pool, access_key, secret_key).await?;
        info!("Root user created successfully");
    } else {
        info!("Root user already exists, updating credentials...");
        UserRepository::create_root_user(pool, access_key, secret_key).await?;
        info!("Root user credentials updated successfully");
    }
    
    Ok(())
}

