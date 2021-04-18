use web_view::*;

pub fn start() {
    let html = include_str!("../frontend/index.html");
    web_view::builder()
        .title("My Project")
        .content(Content::Html(html))
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}
