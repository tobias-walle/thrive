mod web_view;

pub async fn start() -> anyhow::Result<()> {
    let address = thrive_server::find_available_address();
    let full_address = address.full().clone();
    let server_handle = tokio::spawn(async move { thrive_server::start(&address).await });
    web_view::start(&full_address)?;
    server_handle.await??;
    Ok(())
}
