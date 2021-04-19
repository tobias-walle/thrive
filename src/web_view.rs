use std::fmt::format;

use tokio::task;
use tokio::time;
use tokio::time::Duration;
use web_view::*;

pub async fn start(backend_address: &str) -> Result<(), anyhow::Error> {
    let backend_address = backend_address.to_string();
    task::spawn_blocking(move || {
        web_view::builder()
            .title("My Project")
            .content(Content::Url(backend_address))
            .resizable(true)
            .debug(true)
            .user_data(())
            .invoke_handler(|_webview, _arg| Ok(()))
            .run()
    })
    .await??;
    Ok(())
}
