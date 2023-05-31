use std::io;
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::utils::thread_pool::ThreadPool;

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

    pub fn listen_to_incoming_connections(&self, thread_pool_capacity: usize) {
        let pool = ThreadPool::new(thread_pool_capacity);

        let requested_shutdown = Arc::new(AtomicBool::new(false));
        let requested_shutdown_closure = requested_shutdown.clone();

        ctrlc::set_handler(move || requested_shutdown_closure.store(true, Ordering::SeqCst))
            .expect("Error setting Ctrl-C handler");

        println!("** Started listening for new connections on '{}' using a thread pool with capacity set to {}...", &self.bound_addr, thread_pool_capacity);

        // Make the "tcp_listener.accept()" non-blocking so breaking out of the loop is immediate (not waiting for a connection first).
        self.tcp_listener.set_nonblocking(true)
            .expect(format!("* Cannot set TCP listener for '{}' as non-blocking!", self.bound_addr).as_str());

        while !requested_shutdown.load(Ordering::SeqCst) {
            match self.tcp_listener.accept() {
                Ok(new_connection) => {
                    pool.execute(|| {
                        helpers::handle_new_connection(new_connection);
                    });
                }

                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // Wait until network socket is ready, typically implemented via platform-specific APIs such as epoll or IOCP.

                    // Or... just ignore the error and move on.
                    continue;
                }

                Err(e) => eprintln!("* Couldn't get client: {e:?}")
            }
        }

        println!("** Stopped listening for new connections on '{}'!", &self.bound_addr);

        // NOTE: The thread pool goes out of scope and the customised drop() method (from the Drop trait) is automatically called for it.
    }
}

mod helpers {
    use std::{process, thread};
    use std::io::Write;
    use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
    use std::time::Duration;

    use crate::core::http;

    pub fn bind_listen_address(bound_addr: String) -> TcpListener {
        match TcpListener::bind(&bound_addr) {
            Ok(listener) => {
                println!("** Bound a new TCP listener to '{}'!", &bound_addr);
                listener
            }
            Err(cause) => {
                eprintln!("** Failed to bind a new TCP listener to '{}' due to error: '{}'", &bound_addr, cause);
                process::exit(1)
            }
        }
    }

    pub fn handle_new_connection(new_connection: (TcpStream, SocketAddr)) {
        println!("* Handling new connection: {new_connection:?}");

        let mut tcp_stream = new_connection.0;
        let socket_addr = new_connection.1;

        let http_request = http::request::get_request_from_stream(&mut tcp_stream);





        // TODO: this is temp code for testing that multi-threading works in the server!!!

        if http_request.request_line.uri == "/sleep" {
            thread::sleep(Duration::from_secs(5));
        }

        // ~ end of multi-threading test code!!!





        // TODO: process the request and collect necessary data for producing a response.
        let http_response = http::response::create_response(http_request.request_line.version);

        tcp_stream.write_all(http_response.to_string().as_bytes()).unwrap();
        tcp_stream.flush().unwrap();

        // Cleanup.
        tcp_stream.shutdown(Shutdown::Both).expect("TCP stream shutdown call failed!");
    }
}
