use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub jwt_expiration_hours: u64,
    pub cors_allow_origin: String,
    pub cors_allow_methods: String,
    pub cors_allow_headers: String,
    pub cors_max_age: u64,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Config {
            database_url: env::var("DATABASE_URL")?,
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .unwrap_or(8080),
            jwt_secret: env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string()),
            jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .unwrap_or(24),
            cors_allow_origin: env::var("CORS_ALLOW_ORIGIN")
                .unwrap_or_else(|_| "*".to_string()),
            cors_allow_methods: env::var("CORS_ALLOW_METHODS")
                .unwrap_or_else(|_| "GET,POST,PUT,DELETE,OPTIONS".to_string()),
            cors_allow_headers: env::var("CORS_ALLOW_HEADERS")
                .unwrap_or_else(|_| "*".to_string()),
            cors_max_age: env::var("CORS_MAX_AGE")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .unwrap_or(3600),
        })
    }

    pub fn server_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database_url: "postgresql://localhost/airbnb_db".to_string(),
            host: "127.0.0.1".to_string(),
            port: 8080,
            jwt_secret: "your-secret-key-change-in-production".to_string(),
            jwt_expiration_hours: 24,
            cors_allow_origin: "*".to_string(),
            cors_allow_methods: "GET,POST,PUT,DELETE,OPTIONS".to_string(),
            cors_allow_headers: "*".to_string(),
            cors_max_age: 3600,
        }
    }
}
