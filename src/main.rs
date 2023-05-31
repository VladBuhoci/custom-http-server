//use...

mod core;
mod utils;

fn main() -> std::io::Result<()> {
    let host = String::from("localhost");
    let port = 8080;
    let thread_count = 4;

    let server = core::server::Server::new(host, port);

    // This is a blocking function.
    server.listen_to_incoming_connections(thread_count);

    Ok(())
}
