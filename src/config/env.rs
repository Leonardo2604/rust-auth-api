use std::env;
use std::net::{IpAddr, SocketAddr};

#[derive(Clone, Debug)]
pub struct DatabaseEnv {
    url: String,
}

impl DatabaseEnv {
    pub fn new() -> Self {
        DatabaseEnv {
            url: env::var("DATABASE_URL").expect("DATABASE_URL must be set."),
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

#[derive(Clone, Debug)]
pub struct ServerEnv {
    port: u16,
    host: IpAddr,
}

impl ServerEnv {
    pub fn new() -> Self {
        ServerEnv {
            host: env::var("SERVER_HOST")
                .expect("SERVER_HOST must be set.")
                .parse()
                .expect("SERVER_HOST must be a valid IP address."),

            port: env::var("SERVER_PORT")
                .expect("SERVER_PORT must be set.")
                .parse()
                .expect("SERVER_PORT must be a number."),
        }
    }

    pub fn addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}

#[derive(Clone, Debug)]
pub struct Env {
    database: DatabaseEnv,
    server: ServerEnv,
}

impl Env {
    pub fn new() -> Env {
        Env {
            database: DatabaseEnv::new(),
            server: ServerEnv::new(),
        }
    }

    pub fn database(&self) -> &DatabaseEnv {
        &self.database
    }

    pub fn server(&self) -> &ServerEnv {
        &self.server
    }
}
