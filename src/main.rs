use web_server::ThreadPool;
use std::{
    error::Error,
    fmt::Debug, fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread, time::Duration
};

fn main() -> Result<(), Box<dyn Error>> {
    // listen for connection attempts at port 7878 (probably available).
    // to do this, we create a listener and bind it to port 7878
    // todo: handle bind errors
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::build(4)?;

    // a stream represents an open connection between the client and the server.
    // we want to be able to read from and write to each stream
    // todo: handle errors when connecting on port <1024, and when having a simultaneous connection
    // create a thread pool
    for stream in listener.incoming() {
        // todo: handle stream unwrap errs
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    // hacky way to retrieve http header
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // create and write response
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "pages/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "pages/hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "pages/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap(); // read html to String
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    // write_all takes a mutable buffer of u8 elements. write this to the stream
    stream.write_all(response.as_bytes()).unwrap();

    /*
    // collects the lines of the request our browser sends to our server in a vector
    // todo: handle invalid utf-8 or other problems reading from stream
    let http_request: Vec<_> = buf_reader
        .lines() // split by line
        .map(|result| result.unwrap()) // either get each result, or terminate the program
        .take_while(|line| !line.is_empty()) // use a closure to only take nonempty lines
                                     // end of http request is two empty newlines
        .collect(); // turns the iterator into a collection
    */

    // println!("request: {http_request:#?}") // pretty print
}