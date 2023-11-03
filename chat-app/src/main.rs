use std::{net::{TcpListener, TcpStream}, thread, io::{Write, Read}, sync::{Mutex, Arc}};

const PORT: i32 = 8080;

// send data with 
// echo -n "foo" | nc localhost 8080

// or continously with 
// nc localhost 8080

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{PORT}"))?;
    println!("starting a server on {PORT}");

    let history: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));

    for stream in listener.incoming() {
        match stream {
            Ok(v) => {
                let msgs = history.clone();
                thread::spawn(move ||handle_client(v, msgs))
            },
            Err(err) => { 
                println!("error connection {err:?}");
                continue;
            }
        };
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream, msgs: Arc<Mutex<Vec<String>>>) {
    let mut buf = String::new();
    if let Err(e) =  stream.read_to_string(&mut buf) {
        println!("erro reading {e}");
    }
    
    let mut d = msgs.lock().unwrap();
    d.push(buf);
    let d = d.join(", ");
    if let Err(e) = stream.write(format!("[{d}]").as_bytes()) {
        println!("error writing to stream: {e}");
    }
}
