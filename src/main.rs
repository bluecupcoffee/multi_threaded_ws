use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
    process
};
use multi_threaded_ws::{ThreadPool, Worker};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Hello, world!");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let mut request = buf_reader.lines();
    let request_line: String;
    match request.next().unwrap() {
        Ok(r) => {request_line = r;},
        Err(e) => {
            eprintln!("Error!:\n{e}");
            process::exit(1);
        }
    }
    let (status, file) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "sleep.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let content = fs::read_to_string(file).unwrap();
    let length = content.len();
    let response = format!(
        "{status}\r\nContent-Length: {length}\r\n\r\n{content}"
    );
    stream.write_all(response.as_bytes()).unwrap();

}