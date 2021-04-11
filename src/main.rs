use thrive::*;

#[tokio::main]
async fn main() {
    println!("{}", js::exec("console.log('Hello World')").await.unwrap());
}
