# Simple HTTP Server in Rust

This repository contains a simple HTTP server implemented in Rust. The server listens on port 8000 and responds to various routes with static HTML content or JSON data. It demonstrates basic networking concepts, file handling, and error management in Rust.

## Features

- Handles GET requests to `/`, `/second`, and `/api` endpoints.
- Returns a custom 404 page for unknown routes.
- Uses multithreading to handle multiple client connections concurrently.

## Getting Started

To run this server locally, follow these steps:

1. Ensure you have Rust and Cargo installed. If not, visit [the official Rust website](https://www.rust-lang.org/tools/install) to install them.

2. Clone this repository to your local machine.

3. Open a terminal and navigate to the cloned repository's directory.

4. Run `cargo run` to build and execute the server.

5. Open a web browser and navigate to `http://127.0.0.1:8000` to view the server's homepage.

6. Try accessing other routes like `http://127.0.0.1:8000/second` or `http://127.0.0.1:8000/api` to see the server's response.

## Directory Structure

- `src/`: Contains the Rust source code.
  - `main.rs`: The entry point of the application.
- `./public/`: Stores static HTML files served by the server.
  - `index.html`: The homepage.
  - `second.html`: An additional page accessible via `/second`.
  - `404.html`: Custom 404 error page.

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for discussion.
