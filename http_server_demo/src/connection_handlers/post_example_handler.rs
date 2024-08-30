pub struct PostHandler;

use crate::connection_handler_trait::ConnectionHandler;


impl PostHandler {
    fn handle_post(&self, request: &str) -> (String, String) {
        let target_html = <PostHandler as ConnectionHandler>::find_target_html(&self, request);
        ("HTTP/1.1 200 OK".to_string(), target_html)
    }

    fn add_post_data_to_response(&self, response: String, request: &str) -> String {
        let post_data = self.extract_post_data(request);
        println!("POST Body: {}", post_data);
        response.replace("{{POST_DATA}}", &post_data)
    }

    fn extract_post_data(&self, request: &str) -> String {
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
}

impl ConnectionHandler for PostHandler {
    fn handle(&self, request: &str, buffer: &[u8]) -> Option<String> {
        let post = b"POST";

        if buffer.starts_with(post) {
            let (status_line, filename) = self.handle_post(request);
            let mut response = self.generate_response_from_html_file(&filename, &status_line);

            response = self.add_post_data_to_response(response, request);

            Some(self.add_content_length_to_response(response))
        } else {
            None
        }
    }
}




