pub trait ConnectionHandler {
    fn handle(&self, request: &str, buffer: &[u8]) -> Option<String>;
}