use actix_web::{get, web, App, HttpResponse, HttpServer, Result};

#[get("/")]
async fn index() -> Result<HttpResponse> {
    let content = include_str!("../frontend/index.html");
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(content))
}

async fn start_server(address: &str) -> anyhow::Result<()> {
    HttpServer::new(|| App::new().service(index))
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
