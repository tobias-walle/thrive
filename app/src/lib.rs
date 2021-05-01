use futures::try_join;

mod web_view;

pub async fn start() -> anyhow::Result<()> {
    let address = thrive_server::find_available_address();
    let full_address = &address.full();
    try_join!(
        thrive_server::start(&address),
        web_view::start(full_address)
    )?;
    Ok(())
}
