pub mod net;
extern crate term;

use net::ping::Ping;
use std::io::prelude::*;
use term::StdoutTerminal;

fn main() {
	let mut terminal = term::stdout().unwrap();

	let destination = String::from("127.0.0.1");
	print!("Pinging {}: ", destination);
    let ping = Ping::new(destination);

    match ping.run() {
    	Ok(_) => {
    		write_success(&mut *terminal, "OK");
    	},
    	Err(err) => {
    		terminal.fg(term::color::RED).unwrap();
    		(writeln!(terminal, "ERROR ({:?})", err)).unwrap();
    		terminal.fg(term::color::BLACK).unwrap();
    	}
    }

    println!("wat?");
}

fn write_success(terminal: &mut StdoutTerminal, message: &str) {
	terminal.fg(term::color::GREEN).unwrap();
	(writeln!(terminal, "{}", message)).unwrap();
	terminal.fg(term::color::BLACK).unwrap();
}