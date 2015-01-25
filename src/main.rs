extern crate telnet;

use telnet::Telnet;
fn main() {
    let t = Telnet { address: "localhost".to_string(), port: 3000 };
    t.listen();
}
