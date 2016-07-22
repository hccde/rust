use std::net::{TcpListener, TcpStream,Shutdown};
use std::thread;
use std::io::prelude::*;
use std::fmt;
use std::io::BufWriter;
use std::u8;
use std::str::*;
fn main(){

let listener = TcpListener::bind("127.0.0.1:5234").unwrap();

fn handle_client(mut stream: TcpStream) {
	println!("someone connect");
	let mut str = String::new();
    let bufstr = "HTTP/1.1 201 OKs\r\nServer: bfe/1.0.8.14\r\nContent-Type: text/html;charset=utf-8\r\nContent-Length=100\r\n \r\n";
    let mut vvv:Vec<u8> = vec![];
    for b in bufstr.as_bytes() {
            &vvv.push(*b);
    }

    println!("{:?}",from_utf8(&vvv).unwrap());
	stream.write(&vvv);
	stream.write_fmt(format_args!("\r\n<html><body>hello admos</body></html>"));//发送回去
    stream.flush();
    stream.shutdown(Shutdown::Write);
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