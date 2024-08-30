use http_server_demo::ThreadPool;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;

mod connection_handler_trait;
use crate::connection_handler_trait::ConnectionHandler;

mod connection_handlers;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    let pool = ThreadPool::new(2);

    let handlers: Vec<Arc<dyn ConnectionHandler + Send + Sync>> = vec![
        Arc::new(connection_handlers::home_page_handler::HomePageHandler),
        Arc::new(connection_handlers::post_example_handler::PostHandler),
        Arc::new(connection_handlers::porenta_handler::PorentaHandler),
        Arc::new(connection_handlers::html_file_handler::HtmlFileHandler)
    ];

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let handlers = handlers.clone();

        pool.execute(move || {
            handle_connection(stream, &handlers);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream, handlers: &[Arc<dyn ConnectionHandler + Send + Sync>]) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request:\n{}", request);

    for handler in handlers {
        if let Some(response) = handler.handle(&request, &buffer) {
            stream.write_all(response.as_bytes()).unwrap();
            stream.flush().unwrap();
            return;
        }
    }
}


