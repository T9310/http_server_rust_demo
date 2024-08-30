
use std::fs;

pub trait ConnectionHandler {
    fn handle(&self, request: &str, buffer: &[u8]) -> Option<String>;

    fn generate_response_from_file(&self,filename: &String, status_line: &String) -> String {
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
                        "HTTP/1.1 404 ERROR",
                        contents
                    )
                }
        };
        response
    }
        
        
    fn add_content_length_to_response(&self, response: String) -> String {
        let body_start = response.find("\r\n\r\n").unwrap() + 4;
        let body_end = response.len();
    
        let content_length: usize = body_end - body_start;
        println!("Conten-length: {}", content_length);
        let response = response.replace("Content-Length: ", &format!("Content-Length: {}", content_length));
    
        response
    }

    fn find_target_html(&self, request: &str) -> String {
        let start_index = request.find("/").unwrap_or(0) + 1;
        let end_index = request[start_index..].find(" ").unwrap_or(0) + start_index;
        let target = &request[start_index..end_index];
        let target_html = format!("html_files/{}", target);
        println!("{}", target_html);
        target_html
    }
        
}

