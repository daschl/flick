pub mod net;

use net::ping::Ping;

fn main() {
	// todo: allow to pass in hostname
	// print when it starts pinging and in green OK of in red FAILURE
	// if green keep going, if failure print the returned failure nicely.
    let ping = Ping::new(String::from("127.0.0.1"));

    println!("{:?}", ping.run());
}
