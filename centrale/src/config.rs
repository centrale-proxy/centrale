pub struct CentraleConfig;

impl CentraleConfig {
    pub const SERVER_ADDRESS: &str = "127.0.0.1:8080";
    pub const DB_FOLDER: &str = "centrale";
    pub const DB_FILE: &str = "centrale.db";
    pub const DOMAIN: &str = "localhost";
    pub const COOKIE_TIMEOUT: i64 = 86400; // 60 * 60 * 24
    pub const COOKIE_SECURE: bool = false;
    pub const COOKIE_HTTP_ONLY: bool = false;
}
