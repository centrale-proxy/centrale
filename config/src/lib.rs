pub struct CentraleConfig;

impl CentraleConfig {
    pub const WRITER_SERVER_ADDRESS: &str = "127.0.0.1:8081";
    pub const DB_FOLDER: &str = "centrale";
    pub const DB_FILE: &str = "centrale.db";
    // RATE LIMITER
    /// How many requests are allowed to pipeline in total per one IP
    pub const RATE_LIMITER_BURST_SIZE: u32 = 10;
    /// How many places in pipeline are freed in a second for 1 IP
    pub const RATE_LIMITER_REQUESTS_PER_SECOND: u64 = 2000;
    // WRITER
    pub const WRITER_EVENTS_CAPACITY: usize = 10000;
    pub const WRITER_DB_FILE: &str = "writer.db";
    pub const WRITER_BUFFER_SIZE: usize = 1024;
    // WORKERS
    pub const PROXY_SERVER_WORKERS: usize = 1;
    pub const DESTINATION_SERVER_WORKERS: usize = 1;
    // SUBDOMAIN
    pub const MAX_SUBDOMAIN_LENGTH: usize = 30;
    // SUBDOMAIN
    pub const MAX_SUBDOMAIN_NAME_LENGTH: usize = 30;

    // ENVIRONMENT VARIABLES:
    // MASTER KEY BETWEEN PROXY AND NODES
    pub const CENTRALE_MASTER_BEARER_TOKEN: &str = "CENTRALE_MASTER_BEARER_TOKEN";
    // MASTER PASSWORD FOR LOCAL DB ENCRYPTION
    pub const CENTRALE_MASTER_PASSWORD: &str = "CENTRALE_MASTER_PASSWORD";
    pub const CENTRALE_CERT_PRIVATE_KEY: &str = "CENTRALE_CERT_PRIVATE_KEY";
    pub const CENTRALE_CERT_PUB_KEY: &str = "CENTRALE_CERT_PUB_KEY";
    /*
       pub fn master_bearer_token() -> String {
           std::env::var(Self::CENTRALE_MASTER_BEARER_TOKEN)
               .expect("CENTRALE_MASTER_BEARER_TOKEN must be set")
       }
    */
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
        //Self::master_bearer_token();
        Self::master_password();

        let domain = Self::get("DOMAIN");
        println!("DOMAIN: {}", domain);

        let server_address = Self::get("SERVER_ADDRESS");
        println!("SERVER_ADDRESS: {}", server_address);

        let target_server = Self::get("DESTINATION_SERVER_ADDRESS");
        println!("DESTINATION_SERVER_ADDRESS: {}", target_server);

        Self::get("DESTINATION_SERVER_PASSWORD")
            .parse::<String>()
            .unwrap();

        let serve_front = Self::get("SERVE_FRONT").parse::<bool>().unwrap();
        println!("SERVE_FRONT: {}", serve_front);

        if serve_front == true {
            let front_end_folder = Self::get("FRONT_END_FOLDER").parse::<String>().unwrap();
            println!("FRONT_END_FOLDER: {}", front_end_folder);
        }

        Self::get("PUBLIC_RATE_LIMITER_BURST_SIZE")
            .parse::<u32>()
            .unwrap();

        Self::get("PUBLIC_RATE_LIMITER_REQUESTS_PER_SECOND")
            .parse::<u64>()
            .unwrap();

        let cookie_timeout = Self::get("COOKIE_TIMEOUT").parse::<i64>().unwrap();
        println!("COOKIE_TIMEOUT: {}", cookie_timeout);

        let cookie_secure = Self::get("COOKIE_SECURE").parse::<bool>().unwrap();
        println!("COOKIE_SECURE: {}", cookie_secure);

        let cookie_https = Self::get("COOKIE_HTTP_ONLY").parse::<bool>().unwrap();
        println!("COOKIE_HTTP_ONLY: {}", cookie_https);

        Self::cert_private_key();
        Self::cert_pub_key();
    }
}
