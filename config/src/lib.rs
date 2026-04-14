pub struct CentraleConfig;

impl CentraleConfig {
    // ENVIRONMENT VARIABLES:
    // MASTER KEY BETWEEN PROXY AND NODES
    pub const CENTRALE_MASTER_BEARER_TOKEN: &str = "CENTRALE_MASTER_BEARER_TOKEN";
    // MASTER PASSWORD FOR LOCAL DB ENCRYPTION
    pub const CENTRALE_MASTER_PASSWORD: &str = "CENTRALE_MASTER_PASSWORD";
    pub const CENTRALE_CERT_PRIVATE_KEY: &str = "CENTRALE_CERT_PRIVATE_KEY";
    pub const CENTRALE_CERT_PUB_KEY: &str = "CENTRALE_CERT_PUB_KEY";
    // NOT ENVIRONMENT VARIABLES:
    pub const SERVER_ADDRESS: &str = "0.0.0.0:443";
    pub const WRITER_SERVER_ADDRESS: &str = "127.0.0.1:8081";
    pub const DB_FOLDER: &str = "centrale";
    pub const DB_FILE: &str = "centrale.db";
    pub const COOKIE_TIMEOUT: i64 = 86400; // 60 * 60 * 24 * 30 // 86400 // 2592000
    pub const COOKIE_SECURE: bool = true; // FALSE FOR LOCAL
    pub const COOKIE_HTTP_ONLY: bool = true; // FALSE FOR LOCAL
    // RATE LIMITER
    /// How many requests are allowed to pipeline in total per one IP
    pub const RATE_LIMITER_BURST_SIZE: u32 = 10;
    /// How many places in pipeline are freed in a second for 1 IP
    pub const RATE_LIMITER_REQUESTS_PER_SECOND: u64 = 500;
    // RATE LIMITER FOR PUBLIC ENDPOINTS
    /// How many requests are allowed to pipeline in total per one IP
    pub const PUBLIC_RATE_LIMITER_BURST_SIZE: u32 = 1;
    /// How many places in pipeline are freed in a second for 1 IP
    pub const PUBLIC_RATE_LIMITER_REQUESTS_PER_SECOND: u64 = 1;
    // WRITER
    pub const WRITER_EVENTS_CAPACITY: usize = 10000;
    pub const WRITER_DB_FILE: &str = "writer.db";
    pub const WRITER_BUFFER_SIZE: usize = 1024;
    // SAMPLE SERVER
    pub const SAMPLE_SERVER_ADDRESS: &str = "127.0.0.1:11111"; // http://127.0.0.1:11111
    // WORKERS
    pub const PROXY_SERVER_WORKERS: usize = 1;
    pub const SAMPLE_SERVER_WORKERS: usize = 1;
    // AIR TOKEN
    pub const AIR_TOKEN_TIMEOUT: i64 = 60;

    pub fn master_bearer_token() -> String {
        std::env::var(Self::CENTRALE_MASTER_BEARER_TOKEN)
            .expect("CENTRALE_MASTER_BEARER_TOKEN must be set")
    }

    pub fn master_password() -> String {
        std::env::var(Self::CENTRALE_MASTER_PASSWORD).expect("CENTRALE_MASTER_PASSWORD must be set")
    }
    // CERT
    pub fn cert_private_key() -> String {
        std::env::var(Self::CENTRALE_CERT_PRIVATE_KEY)
            .expect("CENTRALE_CERT_PRIVATE_KEY must be set")
    }

    pub fn cert_pub_key() -> String {
        std::env::var(Self::CENTRALE_CERT_PUB_KEY).expect("CENTRALE_CERT_PUB_KEY must be set")
    }

    pub fn get(env_var: &str) -> String {
        let err = format!("{} must be set", env_var);
        std::env::var(env_var).expect(&err)
    }

    pub fn test() {
        Self::master_bearer_token();
        Self::master_password();
        Self::cert_private_key();
        Self::cert_pub_key();
        Self::get("DOMAIN");
    }
}
