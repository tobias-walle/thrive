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

pub async fn start(address: &str) -> anyhow::Result<()> {
    let address = address.to_string();
    // Wrap the actix runtime into the tokio runtime, so we can use both
    tokio::task::spawn_blocking(move || {
        let mut system = actix_web::rt::System::new("server");
        system.block_on(async move { start_server(&address).await })
    })
    .await??;
    Ok(())
}
