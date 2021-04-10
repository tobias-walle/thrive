use alcro::{Content, UIBuilder};
use serde_json::to_value;

fn main() {
    let ui = UIBuilder::new()
        .content(Content::Html("<html><body>Close Me!</body></html>"))
        .run()
        .expect("Unable to launch");
    assert_eq!(ui.eval("document.body.innerText").unwrap(), "Close Me!");

    //Expose rust function to js
    ui.bind("product", |args| {
        let mut product = 1;
        for arg in args {
            match arg.as_i64() {
                Some(i) => product *= i,
                None => return Err(to_value("Not number").unwrap()),
            }
        }
        dbg!(product);
        Ok(to_value(product).unwrap())
    })
    .expect("Unable to bind function");

    ui.eval("console.log('Hello World');").unwrap();

    assert_eq!(ui.eval("(async () => await product(1,2,3))();").unwrap(), 6);
    assert!(ui.eval("(async () => await product(1,2,'hi'))();").is_err());
    ui.wait_finish();
}
