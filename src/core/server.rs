use std::net::TcpListener;

pub struct Server {
    listen_host: String,
    listen_port: i32,
    bound_addr: String,
    tcp_listener: TcpListener,
}

impl Server {
    pub fn new(host: String, port: i32) -> Self {
        let bound_address = format!("{}:{}", host, port);

        Server {
            listen_host: host,
            listen_port: port,
            bound_addr: bound_address.clone(),
            tcp_listener: helpers::bind_listen_address(bound_address),
        }
    }

    pub fn listen_to_incoming_connections(&self) {
        loop {
            match self.tcp_listener.accept() {
                Ok(new_connection) => helpers::handle_new_connection(new_connection),
                Err(e) => eprintln!("Couldn't get client: {e:?}")
            }
        }
    }
}

mod helpers {
    use std::io::Write;
    use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
    use std::process;

    use crate::core::http;

    pub fn bind_listen_address(bound_addr: String) -> TcpListener {
        match TcpListener::bind(&bound_addr) {
            Ok(listener) => {
                println!("** Started listening on '{}'...", &bound_addr);
                listener
            }
            Err(cause) => {
                eprintln!("** Failed to begin listening on '{}' due to error: '{}'", &bound_addr, cause);
                process::exit(1)
            }
        }
    }

    pub fn handle_new_connection(new_connection: (TcpStream, SocketAddr)) {
        println!("* Handling new connection: {new_connection:?}");

        let mut tcp_stream = new_connection.0;
        let socket_addr = new_connection.1;

        tcp_stream.set_nonblocking(true).expect("TODO: panic message");

        let http_request = http::request::get_request_from_stream(&mut tcp_stream);
        let http_response = http::response::create_response();

        tcp_stream.write_all(http_response.to_string().as_bytes());
        tcp_stream.flush();

        // Cleanup.
        tcp_stream.shutdown(Shutdown::Both).expect("TODO: panic message");
    }
}
