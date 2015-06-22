use std::process::Command;

#[derive(Debug)]
pub struct Ping {
	destination: String
}

impl Ping {

	pub fn new(destination: String) -> Ping {
		Ping { destination: destination }
	}

	pub fn run(&self) -> PingResponse {
		let output = Command::new("ping")
			.arg("-c 1")
			.arg(self.destination.clone())
			.output()
			.unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });


		let output = String::from_utf8_lossy(&output.stdout);

		let packets: Vec<PingPacket> = output
			.lines()
			.filter(|line| String::from(*line).starts_with("64"))
			.map(|line| {
				let split = line.split(' ').collect::<Vec<&str>>();
				let icmp_seq = split[4].split('=').collect::<Vec<&str>>();
				let ttl = split[5].split('=').collect::<Vec<&str>>();
				let time = split[6].split('=').collect::<Vec<&str>>();
				PingPacket { icmp_seq: icmp_seq[1].parse().unwrap(), time: time[1].parse().unwrap(), ttl: ttl[1].parse().unwrap() }
			})
			.collect();
		
		PingResponse { destination: self.destination.clone(), packets: packets }
	}

}

#[derive(Debug)]
struct PingResponse {
	destination: String,
	packets: Vec<PingPacket>
}

#[derive(Debug)]
struct PingPacket {
	icmp_seq: i32,
	time: f64,
	ttl: i32
}