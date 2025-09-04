use std::env;

#[derive(Clone, Debug)]
pub struct Database {
    host: String,
    port: u16,
    username: String,
    password: String,
    name: String,
    max_connections: u32,
}

impl Database {
    pub fn new() -> Self {
        Self {
            host: env::var("DATABASE_HOST").expect("DATABASE_HOST must be set."),

            username: env::var("DATABASE_USER").expect("DATABASE_USER must be set."),

            password: env::var("DATABASE_PASS").expect("DATABASE_PASS must be set."),

            name: env::var("DATABASE_NAME").expect("DATABASE_NAME must be set."),

            port: env::var("DATABASE_PORT")
                .expect("DATABASE_PORT must be set.")
                .parse()
                .expect("DATABASE_PORT must be a number."),

            max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                .expect("DATABASE_MAX_CONNECTIONS must be set.")
                .parse()
                .expect("DATABASE_MAX_CONNECTIONS must be a number."),
        }
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn max_connections(&self) -> u32 {
        self.max_connections
    }
}
