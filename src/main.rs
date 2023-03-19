//use...

mod core;

fn main() -> std::io::Result<()> {
    let host = String::from("localhost");
    let port = 8080;

    let server = core::server::Server::new(host, port);

    server.listen_to_incoming_connections();

    Ok(())
}
