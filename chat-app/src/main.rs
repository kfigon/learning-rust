use std::{net::{TcpListener, TcpStream}, thread, io::{Write, Read}, sync::{Mutex, Arc, mpsc::{Receiver, Sender}}};

const PORT: i32 = 8080;

// open con in multiple windows
// nc localhost 8080

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(format!("127.0.0.1:{PORT}"))?;
    println!("starting a server on {PORT}");

    let (tx,rx): (Sender<String>, Receiver<String>)= std::sync::mpsc::channel();
    let clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(vec![]));

    {
        // server
        let clients = clients.clone();
        thread::spawn(move|| dispatch_msg(clients, rx));
    }

    for stream in listener.incoming() {
        match stream {
            Ok(v) => {
                clients.lock().unwrap().push(v.try_clone().unwrap());
                println!("client connected");

                let sender = tx.clone();
                thread::spawn(move ||handle_client(v, sender))
            },
            Err(err) => { 
                println!("error connection {err:?}");
                continue;
            }
        };
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream, msgs: Sender<String>) {
    loop {
        let mut buf = [0;512];
        if let Err(e) = stream.read(&mut buf) {
            println!("erro reading {e}");
            continue;
        }
        
        msgs.send(String::from_utf8(buf.to_vec()).unwrap()).unwrap();
    }
}

fn dispatch_msg(clients: Arc<Mutex<Vec<TcpStream>>>, rx: Receiver<String>) {
    loop {
        let data = match rx.recv() {
            Err(e) => {
                println!("error receiving data {e}");
                continue;
            },
            Ok(v) => v,
        };
        
        {
            let mut cs = clients.lock().unwrap();

            for c in cs.iter_mut() {
                c.write(format!("==== {data}").as_bytes()).unwrap();
            }
        }
    }
}

