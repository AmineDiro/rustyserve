use hello::ThreadPool;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::sleep;
use std::time::Duration;

fn handle_conn(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req = buf_reader.lines().next().unwrap().unwrap();
    println!(
        "Request-Header from {} : {:?}",
        stream.peer_addr().unwrap(),
        req
    );
    let (status_line, filename) = match &req[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            sleep(Duration::from_secs(5));

            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let page_content = fs::read_to_string(filename).unwrap();
    let length = page_content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{page_content}");

    stream.write_all(response.as_bytes()).unwrap();
}
fn main() {
    let listener =
        TcpListener::bind("127.0.0.1:5000").expect("Can't bind the listener to the specified port");

    let pool = ThreadPool::new(5);
    for stream in listener.incoming() {
        pool.execute(|| handle_conn(stream.unwrap()));
    }
}
