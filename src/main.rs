use std::net::TcpListener;

use zero2prod::configuration::get_configuration;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = format!("127.0.0.1:{}", configuration.application_port);

    let listener = TcpListener::bind(address)?;
    let address = listener.local_addr()?;
    println!("Server Listening on {}", address);

    run(listener)?.await
}
