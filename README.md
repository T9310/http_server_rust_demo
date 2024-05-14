# README Http Server Demo in Rust

This is a practice project aimed at learning Rust by implementing an HTTP server. The server is not suitable for production use.

The server is based on this tutorial: https://doc.rust-lang.org/book/ch20-03-graceful-shutdown-and-cleanup.html

The server has been extended to also handle POST requests. In addition to handling GET requests as described in the original tutorial, this server can now respond to POST requests. New HTML files can be added without further modifying the code. If errors occur, the server can respond with appropriate error codes such as 404 (Not Found) and 405 (Method Not Allowed).

The server can be compiled using Rust's cargo tool. To do this, navigate to the main directory of the server at the level of the src folder and execute the following commands:

```bash
cargo build
```


and

```bash
cargo run
```


Alternatively, you can use Docker to start the server. To do this, at the level of the Dockerfile, execute the following commands:

```bash
docker build -t http_server_demo .
```


and

```bash
docker run -p 8000:8000 http_server_demo
```

#### The server will now be waiting for requests on `127.0.0.1:8000`.
