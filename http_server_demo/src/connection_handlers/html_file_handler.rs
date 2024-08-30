
pub struct HtmlFileHandler;

use crate::connection_handler_trait::ConnectionHandler;

impl ConnectionHandler for HtmlFileHandler {
    fn handle(&self, request: &str, buffer: &[u8]) -> Option<String> {
        let get_home = b"GET / HTTP/1.1\r\n";
        let get = b"GET /"; 
        if !buffer.starts_with(get_home) && buffer.starts_with(get) {
            let status_line = "HTTP/1.1 200 OK".to_string();
            let filename = self.find_target_html(request);
            Some(self.add_content_length_to_response(self.generate_response_from_file(&filename, &status_line)))
        } else {
            None
        }
    }
}


