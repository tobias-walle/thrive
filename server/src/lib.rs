use std::fmt::{self};

use actix_web::{App, HttpServer};

async fn start_server(address: &str) -> anyhow::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            actix_files::Files::new("/", "./frontend/dist")
                .index_file("index.html")
                .show_files_listing(),
        )
    })
    .bind(address)?
    .run()
    .await?;
    Ok(())
}

#[derive(Debug)]
pub struct ServerAddress {
    pub protocol: String,
    pub host: String,
    pub port: u16,
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
    ServerAddress {
        protocol: "http".to_string(),
        host: "127.0.0.1".to_string(),
        port,
    }
}

pub async fn start(address: &ServerAddress) -> anyhow::Result<()> {
    let address = address.without_protocol();
    // Wrap the actix runtime into the tokio runtime, so we can use both
    tokio::task::spawn_blocking(move || {
        let mut system = actix_web::rt::System::new("server");
        system.block_on(async move { start_server(&address).await })
    })
    .await??;
    Ok(())
}
