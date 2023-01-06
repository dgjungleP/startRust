use hello_web::ThreadPool;
use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listen = TcpListener::bind("127.0.0.1:7788").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listen.incoming().take(6) {
        let stream = stream.unwrap();
        pool.excute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut tcp: TcpStream) {
    let mut buffer = [0; 1024];
    tcp.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status, filename) = if buffer.starts_with(get) {
        ("200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("200 OK", "hello.html")
    } else {
        ("404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );
    tcp.write(response.as_bytes()).unwrap();
    tcp.flush().unwrap();
}
