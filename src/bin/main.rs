use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use multithread_web_server::ThreadPool;

static DOMAIN: &str = "127.0.0.1";
static PORT: &str = "7878";

fn main() {
    let listener =
        TcpListener::bind(format!("{}:{}", DOMAIN, PORT)).expect("Something went wrong!");
    let pool = ThreadPool::new(2);

    println!("Listening on http://{}:{}\n", DOMAIN, PORT);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(Box::new(|| handle_connection(stream)));
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let html = fs::read_to_string("index.html").unwrap();
    let get = b"GET / HTTP/1.1";
    let sleep = b"GET /sleep HTTP/1.1";

    let (status_line, content) = match buffer {
        _ if buffer.starts_with(get) => ("HTTP/1.1 200 OK", "<h1>Working!</h1>"),
        _ if buffer.starts_with(sleep) => {
            thread::sleep(Duration::from_secs(3));
            ("HTTP/1.1 200 OK", "<h1>Sleeping!</h1>")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "<h1>Not found</h1>"),
    };

    let html = html.replace(
        "<div id=\"root\"></div>",
        &format!("<div id=\"root\">{}</div>", content),
    );

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        html.len(),
        html
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
