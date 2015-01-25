extern crate telnet;

use telnet::Telnet;
use std::io::{TcpStream, BufferedWriter};

fn handle_event(input: &mut String, writer: &mut BufferedWriter<TcpStream>) {
    println!("Got input {}", input);
    writer.write_line(input.as_slice());
    writer.flush();
}

fn main() {
    let t = Telnet { address: "localhost".to_string(), port: 3000 };
    t.listen(handle_event);
}
