use std::io::{TcpListener, TcpStream, BufferedReader, BufferedWriter};
use std::io::{Acceptor, Listener};
use std::thread::Thread;
use std::str;

pub struct Telnet {
    pub address: String,
    pub port: i32,
    //reader: BufferedReader<TcpStream>,
    //writer: BufferedWriter<TcpStream>,
}

fn handle_connect(stream: TcpStream) {
    let mut reader = BufferedReader::new(stream.clone());
    let mut writer = BufferedWriter::new(stream.clone());
    loop {
        let mut raw_input = reader.read_line().unwrap();
        raw_input.pop();
        println!("got {}", raw_input);
        writer.write_line(raw_input.as_slice());
        writer.flush();
    }
}


impl Telnet {
    pub fn listen(&self) {
        let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
        
        // bind the listener to the specified address
        let mut acceptor = listener.listen().unwrap();

        // accept connections and process them, spawning a new tasks for each one
        for stream in acceptor.incoming() {
            match stream {
                Err(e) => {  }
                Ok(stream) => {
                    Thread::spawn(move|| {
                        // connection succeeded
                        handle_connect(stream)
                    });
                }
            }
        }

        // close the socket server
        drop(acceptor);
    }
}
