mod net;

use net::ping::Ping;

fn main() {
    let ping = Ping::new(String::from("127.0.0.1"));

    println!("{:?}", ping.run());
}
