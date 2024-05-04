use http_server_demo::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(2);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }


    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request:\n{}", request);

    let get_home = b"GET / HTTP/1.1\r\n";
    let get = b"GET";
    let post = b"POST";

    let (status_line, filename) = if buffer.starts_with(get_home) { // Standard Pfad
        ("HTTP/1.1 200 OK".to_string(), "hello.html".to_string())
    } else if buffer.starts_with(get) {
        handle_get(&request)
    } else if buffer.starts_with(post) {
        handle_post(&request)
    } else {
        ("HTTP/1.1 405 METHOD NOT ALLOWED".to_string(), "unknown.html".to_string())
    };

    let response = match fs::read_to_string(&filename) {
        Ok(contents) => format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        ),
        Err(_) => {
            let contents = fs::read_to_string("404.html").unwrap();
            format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                status_line,
                contents.len(),
                contents
            )
        }
    };

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_get(request: &str) -> (String, String) {
    let target_html = find_target_html(request);
    ("HTTP/1.1 200 OK".to_string(), target_html)
}


fn find_target_html(request: &str) -> String {
    let start_index = request.find("/").unwrap_or(0) + 1;
    let end_index = request[start_index..].find(" ").unwrap_or(0) + start_index;
    let target = &request[start_index..end_index];
    let target_html = format!("{}.html", target);
    println!("{}", target_html);
    target_html
}


fn handle_post(request: &str) -> (String, String) {
    let target_html = find_target_html(request);
    let body = extract_post_body(request);
    println!("POST Body: {}", body);

    let response = fs::read_to_string(&target_html)
        .unwrap()
        .replace("{{POST_DATA}}", &body);
    let response_len = response.len();

    (
        format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            response_len, response
        ),
        response,
    )
}

fn extract_post_body(request: &str) -> String {
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
