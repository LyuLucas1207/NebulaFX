/// SQL migration for creating users table
pub const CREATE_USERS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    access_key VARCHAR(255) NOT NULL UNIQUE,
    secret_key VARCHAR(255) NOT NULL,
    user_type VARCHAR(50) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_users_access_key ON users(access_key);
"#;

/// SQL migration for creating policies table
pub const CREATE_POLICIES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS policies (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    policy_doc JSONB NOT NULL,
    version BIGINT NOT NULL DEFAULT 1,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_policies_name ON policies(name);
"#;

/// SQL migration for creating groups table
pub const CREATE_GROUPS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS groups (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL UNIQUE,
    status VARCHAR(50) NOT NULL DEFAULT 'enabled',
    members JSONB NOT NULL DEFAULT '[]'::jsonb,
    version BIGINT NOT NULL DEFAULT 1,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_groups_name ON groups(name);
"#;

/// SQL migration for creating mapped_policies table
pub const CREATE_MAPPED_POLICIES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS mapped_policies (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    user_type VARCHAR(50) NOT NULL,
    is_group BOOLEAN NOT NULL DEFAULT false,
    policies TEXT NOT NULL,
    version BIGINT NOT NULL DEFAULT 1,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(name, user_type, is_group)
);

CREATE INDEX IF NOT EXISTS idx_mapped_policies_name ON mapped_policies(name);
CREATE INDEX IF NOT EXISTS idx_mapped_policies_user_type ON mapped_policies(user_type);
CREATE INDEX IF NOT EXISTS idx_mapped_policies_is_group ON mapped_policies(is_group);
"#;

/// SQL migration for creating user_identities table (for storing UserIdentity data)
pub const CREATE_USER_IDENTITIES_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS user_identities (
    id BIGSERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    user_type VARCHAR(50) NOT NULL,
    identity_data JSONB NOT NULL,
    ttl INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(name, user_type)
);

CREATE INDEX IF NOT EXISTS idx_user_identities_name ON user_identities(name);
CREATE INDEX IF NOT EXISTS idx_user_identities_user_type ON user_identities(user_type);
"#;

/// All migration SQL statements
pub const ALL_MIGRATIONS: &[&str] = &[
    CREATE_USERS_TABLE,
    CREATE_POLICIES_TABLE,
    CREATE_GROUPS_TABLE,
    CREATE_MAPPED_POLICIES_TABLE,
    CREATE_USER_IDENTITIES_TABLE,
];

