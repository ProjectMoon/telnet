use std::io::{TcpListener, TcpStream, BufferedReader, BufferedWriter};
use std::io::{Acceptor, Listener};
use std::thread::Thread;
use std::sync::Arc;

pub struct Telnet {
    pub address: String,
    pub port: i32,
    //reader: BufferedReader<TcpStream>,
    //writer: BufferedWriter<TcpStream>,
}

impl Telnet {
    pub fn listen<H: Handler>(&self, handler: H)  {
        let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
        
        let mut acceptor = listener.listen().unwrap();
        let handler_arc = Arc::new(handler);
        
        for stream in acceptor.incoming() {
            match stream {
                Err(e) => {  }
                Ok(stream) => {
                    let local_handler = handler_arc.clone();
                    Thread::spawn(move|| {
                        let mut reader = BufferedReader::new(stream.clone());
                        let mut writer = BufferedWriter::new(stream.clone());
                        
                        loop {
                            let mut raw_input = reader.read_line().unwrap();
                            raw_input.pop();
                            local_handler.handle(&mut raw_input, &mut writer);
                        }
                    });
                }
            }
        }
        
        drop(acceptor);
    }
}

pub trait Handler: Sync + Send {
    fn handle(&self, &mut String, &mut BufferedWriter<TcpStream>);
}

impl<F> Handler for F where F: Fn(&mut String, &mut BufferedWriter<TcpStream>), F: Sync + Send {
    fn handle(&self, input: &mut String, writer: &mut BufferedWriter<TcpStream>) {
        (*self)(input, writer);
    }
}
