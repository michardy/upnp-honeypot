use chrono::prelude::*;
use redis::Commands;

use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};

#[derive(Serialize, Deserialize, Debug)]
/// Structure used to store IP traffic records
pub struct Ip {
	/// Whether the IP is blacklisted
	pub blacklisted: bool,
	/// When the IP address was last seen
	last_seen: Option<DateTime<Utc>>,
	/// Value from 3600 to zero dictating whether the IP will be blacklisted
	/// The value starts at 3600 and is decrimented by the number of seconds remaining in the hour since the last request
	/// In the event the request came in more than an hour after the previous request it is set back to 3600
	/// This function is used to prevent this service from being effectivly harnessed in an amplified DOS attack.  
	consumed: i32
}

impl Ip {
	fn new() -> Ip {
		Ip {
			blacklisted: false,
			last_seen: None,
			consumed: 3600
		}
	}
	pub fn write(self, address: &String, connection: &redis::Connection) -> redis::RedisResult<()> {
		let mut buf: Vec<u8> = Vec::new();
		self.serialize(&mut Serializer::new(&mut buf)).unwrap();
		connection.set(address, buf)?;
		Ok(())
	}
	pub fn get(address: &String, connection: &redis::Connection) -> Ip {
		let buf: Vec<u8> = connection.get(address).unwrap();
		let mut de = Deserializer::new(&buf[..]);
		let mut addr: Ip = match Deserialize::deserialize(&mut de) {
			Ok(out) => out,
			Err(_) => Ip::new()
		};
		addr.update(Utc::now());
		addr
	}
	/// Updates the consumed and last_seen fields. Additionally blacklists consumed IPs
	pub fn update(&mut self, recieved: DateTime<Utc>) {
		if !self.blacklisted {
			match self.last_seen {
				Some(time) => {
					let delta = recieved.signed_duration_since(time);
					if delta.num_seconds() >= 3600 {
						self.consumed = 3600;
					} else {
						self.consumed = self.consumed - (3600-delta.num_seconds() as i32);
						if self.consumed <= 0 {
							self.blacklisted = true;
						}
					}
				},
				None => {}
			}
			self.last_seen = Some(recieved);
		}
	}
}
