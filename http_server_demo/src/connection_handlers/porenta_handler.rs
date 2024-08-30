pub struct PorentaHandler;

use crate::connection_handler_trait::ConnectionHandler;

impl ConnectionHandler for PorentaHandler {
    fn handle(&self, request: &str, buffer: &[u8]) -> Option<String> {
        if buffer.starts_with(b"PORENTA / HTTP/1.1\r\n") {
            let status_line = "HTTP/1.1 200 OK".to_string();
            let filename = "html_files/PORENTA.txt".to_string();
            Some(self.add_content_length_to_response(self.generate_response_from_file(&filename, &status_line)))
        } else {
            None
        }
    }
}
