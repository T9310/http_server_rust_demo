This is a practice project aimed at learning Rust by implementing an HTTP server. The server is not suitable for production use.

The server is based on this tutorial: https://doc.rust-lang.org/book/ch20-03-graceful-shutdown-and-cleanup.html
I extended it to allow adding new HTML files without further modifying the code. It can handle GET and POST requests and respond with error codes 404 and 405 if they occur.
