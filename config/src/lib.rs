pub struct CentraleConfig;

impl CentraleConfig {
    pub const SERVER_ADDRESS: &str = "127.0.0.1:8080";
    pub const DB_FOLDER: &str = "centrale";
    pub const DB_FILE: &str = "centrale.db";
    pub const DOMAIN: &str = "localhost.com";
    pub const COOKIE_TIMEOUT: i64 = 86400; // 60 * 60 * 24
    pub const COOKIE_SECURE: bool = false;
    pub const COOKIE_HTTP_ONLY: bool = false;
    // RATE LIMITER
    /// How many requests are allowed to pipeline in total per one IP
    pub const RATE_LIMITER_BURST_SIZE: u32 = 120000;
    /// How many places in pipeline are freed in a second for 1 IP
    pub const RATE_LIMITER_REQUESTS_PER_SECOND: u64 = 120000;
}
