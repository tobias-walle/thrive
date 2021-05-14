use std::fmt;

#[derive(Debug)]
pub struct ServerAddress {
    pub protocol: String,
    pub host: String,
    pub port: u16,
}

impl Default for ServerAddress {
    fn default() -> Self {
        ServerAddress {
            protocol: "http".to_string(),
            host: "localhost".to_string(),
            port: 10000,
        }
    }
}

impl ServerAddress {
    pub fn from_port(port: u16) -> ServerAddress {
        ServerAddress {
            port,
            ..Default::default()
        }
    }
}

impl fmt::Display for ServerAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}://{}:{}", self.protocol, self.host, self.port)
    }
}

impl ServerAddress {
    pub fn full(&self) -> String {
        self.to_string()
    }

    pub fn without_protocol(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

pub fn find_available_address() -> ServerAddress {
    let port = portpicker::pick_unused_port().expect("Couldn't find a free port.");
    ServerAddress::from_port(port)
}
