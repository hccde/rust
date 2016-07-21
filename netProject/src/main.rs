use std::net::{TcpListener, TcpStream,Shutdown};
use std::thread;
use std::io::prelude::*;
use std::fmt;

fn main(){
let listener = TcpListener::bind("127.0.0.1:5234").unwrap();

fn handle_client(mut stream: TcpStream) {
	println!("someone connect");
	let mut str = String::new();
	let mut  buf = vec![];
	buf.write_all(b"HTTP/1.1 200 OK\r\n").unwrap();
	stream.write(&buf);
	stream.write_fmt(format_args!("hello admos"));//发送回去
    println!("{:?}",stream.read_to_string(&mut str).unwrap());
    println!("{:?}", &str);

    // stream.shutdown(Shutdown::Write);


}

// accept connections and process them, spawning a new thread for each one
for stream in listener.incoming() {
    match stream {
        Ok(stream) => {
            thread::spawn(move|| {
                // connection succeeded
                let mut streams = stream;
                handle_client(streams);
            });
        }
        Err(e) => { /* connection failed */ 
        	println!("we failed");
        }
    }
}

// close the socket server
drop(listener);
}