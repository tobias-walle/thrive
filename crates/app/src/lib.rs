use futures::try_join;

mod web_view;

pub async fn start() -> anyhow::Result<()> {
    let port = portpicker::pick_unused_port().expect("Couldn't find a free port.");
    let address = &format!("127.0.0.1:{}", port);
    let full_address = format!("http://{}", address);
    println!("Start Server on {}...", &full_address);
    let a = thrive_server::start(address);
    let b = web_view::start(&full_address);
    try_join!(a, b)?;
    Ok(())
}
