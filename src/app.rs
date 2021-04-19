use crate::{server, web_view};
use futures::try_join;

pub async fn start() -> anyhow::Result<()> {
    let port = portpicker::pick_unused_port().expect("Couldn't find a free port.");
    let address = &format!("127.0.0.1:{}", port);
    let full_address = format!("http://{}", address);
    println!("Start Server on {}...", &full_address);
    let a = server::start(address);
    let b = web_view::start(&full_address);
    try_join!(a, b)?;
    Ok(())
}
