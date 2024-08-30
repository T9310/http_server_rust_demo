use http_server_demo::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::Arc;

mod connection_handler_trait;
use crate::connection_handler_trait::ConnectionHandler;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    let pool = ThreadPool::new(2);

    let handlers: Vec<Arc<dyn ConnectionHandler + Send + Sync>> = vec![
        Arc::new(HomePageHandler)
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



struct HomePageHandler;

impl ConnectionHandler for HomePageHandler {
    fn handle(&self, request: &str, buffer: &[u8]) -> Option<String> {
        let get_home = b"GET / HTTP/1.1\r\n";
        if buffer.starts_with(get_home) {
            let status_line = "HTTP/1.1 200 OK".to_string();
            let filename = "html_files/hello.html".to_string();
            Some(add_content_length_to_response(generate_response(&filename, &status_line)))
        } else {
            None
        }
    }
}


fn generate_response(filename: &String, status_line: &String) -> String {
let response = match fs::read_to_string(&filename) {
        Ok(contents) => format!(
            "{}\r\nContent-Length: \r\nConnection: close\r\n\r\n{}",
            status_line,
            contents
        ),
        Err(_) => {
            let contents = match fs::read_to_string("html_files/404.html") {
                Ok(contents) => contents,
                Err(_) => "ERROR 404".to_string(),
            };
            format!(
                "{}\r\nContent-Length: \r\nConnection: close\r\n\r\n{}",
                status_line,
                contents
            )
        }
    };
    response
}

fn handle_get_post(request: &str) -> (String, String) {
    let target_html = find_target_html(request);
    ("HTTP/1.1 200 OK".to_string(), target_html)
}

fn handle_porenta() -> (String, String) {
    ("HTTP/1.1 200 OK".to_string(), "html_files/PORENTA.txt".to_string())
}


fn find_target_html(request: &str) -> String {
    let start_index = request.find("/").unwrap_or(0) + 1;
    let end_index = request[start_index..].find(" ").unwrap_or(0) + start_index;
    let target = &request[start_index..end_index];
    let target_html = format!("html_files/{}", target);
    println!("{}", target_html);
    target_html
}

fn add_post_data_to_response(response: String, request: &str) -> String {
    let post_data = extract_post_data(request);
    println!("POST Body: {}", post_data);
    let response = response.replace("{{POST_DATA}}", &post_data);
    response
}

fn add_content_length_to_response(response: String) -> String {
    let body_start = response.find("\r\n\r\n").unwrap() + 4;
    let body_end = response.len();

    let content_length: usize = body_end - body_start;
    println!("Conten-length: {}", content_length);
    let response = response.replace("Content-Length: ", &format!("Content-Length: {}", content_length));

    response
}

fn extract_post_data(request: &str) -> String {
    let content_length_header = "Content-Length: ";
    if let Some(start) = request.find(content_length_header) { 
        let content_length_start = start + content_length_header.len(); 
        if let Some(end) = request[start..].find("\r\n") { 
            let content_length_str = &request[content_length_start..start + end]; 
            if let Ok(content_length) = content_length_str.trim().parse::<usize>() { 
                let body_start = request.find("\r\n\r\n").unwrap() + 4; 
                let body_end = body_start + content_length; 
                return request[body_start..body_end].to_string(); 
            }
        }
    }
    String::new()
}
