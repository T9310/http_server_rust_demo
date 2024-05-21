use http_server_demo::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap(); 
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

    let (status_line, filename) = if buffer.starts_with(get_home) { 
        ("HTTP/1.1 200 OK".to_string(), "html_files/hello.html".to_string())
    } else if buffer.starts_with(b"PORENTA / HTTP/1.1\r\n"){
        handle_porenta()
    } else if buffer.starts_with(get) || buffer.starts_with(post) {
        handle_get_post(&request)
    } else {
        ("HTTP/1.1 405 METHOD NOT ALLOWED".to_string(), "html_files/unknown.html".to_string())
    };
    
    let response: String;

    if buffer.starts_with(post) {
        response = add_post_data_to_response(generate_response(&filename, &status_line), &request);
    } else {
        response = generate_response(&filename, &status_line);
    }

    let response = add_content_length_to_response(response);

    println!("Response: \n{}", &response);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn generate_response(filename: &String, status_line: &String) -> String {
    let response: String = match fs::read_to_string(&filename) {
        Ok(contents) => format!(
            "{}\r\nContent-Length: \r\nConnection: close\r\n\r\n{}",
            status_line,

            contents
        ),
        Err(_) => {
            let contents = fs::read_to_string("html_files/404.html").unwrap();
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
