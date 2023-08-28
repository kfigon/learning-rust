use std::{
    io::{prelude::*, BufReader, Error},
    net::{TcpListener, TcpStream}, thread::{self},
};


fn start() {
    
    let listener = TcpListener::bind("0.0.0.0:8080").expect("failed to open tcp connection");
    loop {
        let con = listener.accept();
        thread::spawn(move || {
            match con {
                Err(err) => println!("error reading connection {}", err),
                Ok((stream, _)) => {
                    if let Err(err) = handle(stream) {
                        println!("error sending response {}", err)
                    }
                },
            }
        });
    }
}

fn handle(mut stream: TcpStream) -> std::io::Result<()> {
    let mut http_request = Vec::new();
    for line_res in BufReader::new(&stream).lines() {
        let line = line_res?;
        if line.is_empty() {
            break;
        } 
        http_request.push(line);
    }

    BufReader::new(&stream).lines()
        .take(f)

    println!("{http_request:?}");

    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n")?;
    stream.flush()?;

    Ok(())
}