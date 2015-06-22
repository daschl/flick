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

impl Ping {

	pub fn new(destination: String) -> Ping {
		Ping { destination: destination }
	}

	pub fn run(&self) -> PingResponse {
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
		// write docs to see how it is generated

		let stdout = String::from_utf8_lossy(&executed.stdout);
		let packets = PingPacket::from(&*stdout);
		
		PingResponse { 
			destination: self.destination.clone(), 
			packets: packets 
		}
	}

}