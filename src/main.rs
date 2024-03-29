use std::net::TcpListener;

use newsletter_with_rust::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind random port");
    run(listener)?.await
}
