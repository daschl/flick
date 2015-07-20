//! The ping module allows to perform pings against a target system.
//!
//! In the current implementation, it utilizes the `ping` command provided by
//! the underlying operating system, because ICMP request and responses very
//! often require root permissions (or setuid) to run properly.
//!
//! You want to start with the `Ping` struct and its `run` method:
//!
//! # Examples
//!
//! ```
//! use net::ping::Ping;
//!
//! let ping = Ping::new(String::from("127.0.0.1"));
//! println!("{:?}", ping.run());
//! // Prints: Ok(PingResponse { destination: "127.0.0.1", packets: [PingPacket { icmp_seq: 0, time: 0.051000000000000004, ttl: 64 }] })
//! ```
use std::process::Command;

#[derive(Debug)]
pub struct PingResponse {
	destination: String,
	packets: Vec<PingPacket>
}

#[derive(Debug)]
pub struct PingPacket {
	icmp_seq: i32,
	time: f64,
	ttl: i32
}

impl PingPacket {

	fn from(source: &str) -> Vec<PingPacket> {
		source
			.lines()
			.filter(|line| String::from(*line).contains("time="))
			.map(|line| line.split(' ').collect::<Vec<&str>>())
			.map(|words| {
				let icmp_seq = words[4].split('=').collect::<Vec<&str>>();
				let ttl = words[5].split('=').collect::<Vec<&str>>();
				let time = words[6].split('=').collect::<Vec<&str>>();
				PingPacket { 
					icmp_seq: icmp_seq[1].parse().unwrap(), 
					time: time[1].parse().unwrap(), 
					ttl: ttl[1].parse().unwrap() 
				}
			})
			.collect::<Vec<PingPacket>>()
	}
}

#[derive(Debug)]
pub struct Ping {
	destination: String
}

#[derive(Debug)]
pub enum PingError { Generic }

impl Ping {

	pub fn new(destination: String) -> Ping {
		Ping { destination: destination }
	}

	/// Performs the `ping` operation against the destination.
	pub fn run(&self) -> Result<PingResponse, PingError> {
		let executed = Command::new("ping")
			.arg("-c 1")
			.arg(self.destination.clone())
			.output()
			.unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });

		// Todo: return result type with errors
		// Check status and if != 0 translate into error status
		// do not panic if error in this module, just raise the error to the caller (remove unwraps)
		// add support for custom -c invocations
		// error if ping does not exist
		// error if hostname not found or other error
		// test it????

		let stdout = String::from_utf8_lossy(&executed.stdout);
		let packets = PingPacket::from(&*stdout);

		if packets.len() > 0 {
			Ok(PingResponse { 
				destination: self.destination.clone(), 
				packets: packets 
			})
		} else {
			Err(PingError::Generic)
		}
	}

}