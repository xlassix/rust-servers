use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(content) => format!(
            "HTTP/1.1 200 OK\r\n Content-Type: text/html\r\n\r\n{}",
            content
        ),
        Err(_) => {
            "HTTP/1.1 500 Internal Server Error \r\n\r\n500 Internal Server Error ".to_string()
        }
    }
}

fn get_response(request: &str) -> Result<String, &'static str> {
    if request.contains("GET /second") {
        Ok(read_file("./public/second.html"))
    } else if request.contains("GET /api") {
        Ok("HTTP/1.1 200 OK\r\n Content-Type: application/json\r\n\r\n{'code':'0','msg':'success','success':true}".to_string())
    } else if request.contains("GET / ") {
        Ok(read_file("./public/index.html"))
    } else {
        Ok(read_file("./public/404.html"))
    }
}

fn handle_client(mut stream: TcpStream) {
    // Buffer for incoming Data
    let mut buffer = [0; 1024];

    if let Err(e) = stream.read(&mut buffer) {
        eprint!("Failed to read data from Connection {}", e);
        return;
    }

    let request = String::from_utf8_lossy(&buffer[..]);

    let response = match get_response(&request) {
        Ok(content) => content,
        Err(_) => {
            "HTTP/1.1 500 Internal Server Error \r\n\r\n500 Internal Server Error ".to_string()
        }
    };

    // write the response back to the client
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprint!("Failed to read data from Connection {}", e);
        return;
    }

    if let Err(e) = stream.flush() {
        eprint!("Failed to flush Connection {}", e);
        return;
    }
}

fn main() {
    // Define Server Port and Listener
    const PORT: i32 = 8000;
    let listener = TcpListener::bind(format!("127.0.0.1:{}", PORT))
        .expect(format!("Failed to Bind to Port ':{}'", PORT).as_str());

    println!("Server is running at http://127.0.0.1:{}", PORT);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Not Ideal: spawn a thread for every client
                thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprint!("Error accepting connecion {}", e)
            }
        }
    }
}
