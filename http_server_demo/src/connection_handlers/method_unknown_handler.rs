pub struct MethodUnknownHandler;

use crate::connection_handler_trait::ConnectionHandler;

impl ConnectionHandler for MethodUnknownHandler {
    fn handle(&self, request: &str, buffer: &[u8]) -> Option<String> {

        let status_line = "HTTP/1.1 405 ERROR".to_string();
        let filename = "html_files/unknown.html".to_string();
        Some(self.add_content_length_to_response(self.generate_response_from_file(&filename, &status_line)))

    }
}
