
pub struct HomePageHandler;

use crate::connection_handler_trait::ConnectionHandler;

impl ConnectionHandler for HomePageHandler {
    fn handle(&self, request: &str, buffer: &[u8]) -> Option<String> {
        let get_home = b"GET / HTTP/1.1\r\n";
        if buffer.starts_with(get_home) {
            let status_line = "HTTP/1.1 200 OK".to_string();
            let filename = "html_files/hello.html".to_string();
            Some(self.add_content_length_to_response(self.generate_response_from_html_file(&filename, &status_line)))
        } else {
            None
        }
    }
}


