use std::env;
use std::net::{IpAddr, SocketAddr};

#[derive(Clone, Debug)]
pub struct Server {
    port: u16,
    host: IpAddr,
}

impl Server {
    pub fn new() -> Self {
        Self {
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
