use chrono::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
/// A structure used to store all incoming SSDP packets in Elasticsearch.
/// This structure is designed to store malformed and potentally malicious packets.
/// It supports:
/// - Repeated headers
/// - Non interger vaues in integer fields
/// - Mixing headers from different methods
/// It is capable of storing any M-SEARCH, NOTIFY, or search response packet.
/// Parsing done here additionally helps inform what kind of response to send if any.
/// Addional fields are generated to increase searchability
pub struct SsdpRequest {
	/// Approximate time request was recieved
	timestamp: Option<DateTime<Utc>>,
	/// Request method. Either `NOTIFY` or `M-SEARCH`
	method: Option<str>,
	/// Resource URI. Must be *
	uri: Option<str>,
	/// Protocol and version. Generally HTTP/1.1
	protocol: Option<str>,
	/// Status code
	status: Option<str>,
	/// Reason (Ex: for status code of 200 reason is OK)
	reason: Option<str>,
	// Search
	/// Target device address or SSDP multicast address
	host: Vec<str>,
	/// The "MAN" field. If set must be `ssdp:discover`
	man: Vec<str>,
	/// The "MX" field. Max number of seconds for random response delay.
	/// Ranges from 1 to 5
	delay: Vec<str>,
	/// The "ST" field. If set contains selector describing devices that should respond
	target: Vec<str>,
	/// User-Agent
	user_agent: Vec<str>,
	// Notification and response
	/// The "CACHE-CONTROL" field
	cache_control: Vec<str>,
	/// The url of the root device description
	location: Vec<str>,
	/// The "NT" field. Notification type
	notification: Vec<str>,
	/// The "NTS" field. Sub notification type
	sub_notification: Vec<str>,
	/// The software and version information
	server: Vec<str>,
	/// The "USN" field. The unique service name of a UPNP service
	service: Vec<str>,
	/// The "BOOTID.UPNP.ORG" field. Boot ID. 31 bit unsigned int (Don't question it)
	boot: Vec<str>,
	/// The "NEXTBOOTID.UPNP.ORG" field. Sent if multihomed device changes connectivity on another network
	next_boot: Vec<str>,
	/// The "CONFIGID.UPNP.ORG" field. Sent when config is updated.
	config: Vec<str>,
	/// The "SEARCHPORT.UPNP.ORG" field. Used for setting nonstandard search ports
	port: Vec<str>,
	// Response only
	/// Response generation date
	date: Vec<str>,
	/// The "EXT" field. Required for backwards compatability
	extension: Vec<str>,
	// Generated fields
	/// Did the request get a response?
	responded: bool,
	/// Is the request valid?
	valid: bool,
	/// Is the HTTPU header valid?
	valid_httpu: bool,
	/// Were any headers invalid?
	unknown: bool,
	/// Were any mandatory headers skipped?
	skipped: bool,
	/// Were any headers written multiple times?
	overwrite: bool,
	/// Did the headers end with a blank line?
	terminated: bool,
	/// Was the combination of headers valid?
	combination: bool,
	/// Request length
	length: u64,
	/// Raw request content
	/// Created because unrecognized fields get dropped
	raw: str
}

impl SsdpRequest {
	/// Parses status header on SSDP's HTTP over UDP packets
	fn parse_httpu(&mut self, lines:Vec<&str>) -> u8 {
		let parts: Vec<&str> = lines[0].split(' ').collect();
		if parts[0] == "HTTP/1.1" {
		} else {
			self.protocol = some(parts[0]);
			self.status = some(parts[1]);
			self.reason = some(parts[2]);
		}
		self.method = Some(parts[0]);
		self.uri = Some(parts[1]);
		self.protocol = Some(parts[2]);
	}
	/// Searches for and parses M-SEARCH parameters
	fn parse_search(&mut self, lines:Vec<&str>) {
		for line in lines {
			let parts: Vec<&str> = lines[0].split(": ").collect();
			if parts[0] == "HOST" {
				self.host.push(parts[1]);
			} else if parts[0] == "MAN" {
				self.man.push(parts[1]);
			} else if parts[0] == "MX" {
				self.delay.push(parts[1]);
			} else if parts[0] == "ST" {
				self.target.push(parts[1]);
			} else if parts[0] == "USER-AGENT" {
				self.user_agent.push(parts[1]);
			}
		}
	}
	/// Searches for and parses NOTIFY and response headers
	fn parse_notify(&mut self, lines:Vec<&str>) {
		for line in lines {
			let parts: Vec<&str> = lines[0].split(": ").collect();
			if parts[0] == "CACHE-CONTROL" {
				self.cache_control.push(parts[1]);
			} else if parts[0] == "LOCATION" {
				self.location.push(parts[1]);
			} else if parts[0] == "NT" {
				self.notification.push(parts[1]);
			} else if parts[0] == "NTS" {
				self.sub_notification.push(parts[1]);
			} else if parts[0] == "SERVER" {
				self.server.push(parts[1]);
			} else if parts[0] == "USN" {
				self.service.push(parts[1]);
			} else if parts[0] == "BOOTID.UPNP.ORG" {
				self.boot.push(parts[1]);
			} else if parts[0] == "NEXTBOOTID.UPNP.ORG" {
				self.next_boot.push(parts[1]);
			} else if parts[0] == "CONFIGID.UPNP.ORG" {
				self.config.push(parts[1]);
			} else if parts[0] == "SEARCHPORT.UPNP.ORG" {
				self.port.push(parts[1]);
			}
		}
	}
	/// Parses response headers
	fn parse_response(&mut self, lines:Vec<&str>) {
		for line in lines {
			let parts: Vec<&str> = lines[0].split(": ").collect();
			if parts[0] == "DATE" {
				self.date.push(parts[1]);
			} else if parts[0] == "EXT" {
				self.extension.push(parts[1]);
			}
		}
	}
	/// Parse a recieved SSDP request
	fn parse(&mut self) {
		let lines:Vec<&str> = self.raw.split("\r\n").collect();
		parse_httpu(&mut self, lines);
		parse_search(&mut self, lines);
		parse_notify(&mut self, lines);
		parse_response(&mut self, lines);
	}
	/// Populate generated fields
	fn generate(&mut self, lines:Vec<&str>) {
		self.valid_httpu = (
			(
				match self.method {
					Some(_) => true,
					None => false
				} &&
				match self.uri {
					Some(_) => true,
					None => false
				} &&
				match self.protocol {
					Some(_) => true,
					None => false
				}
			) || (
				match self.protocol {
					Some(_) => true,
					None => false
				} &&
				match self.status {
					Some(_) => true,
					None => false
				} &&
				match self.reason {
					Some(_) => true,
					None => false
				}
			)
		);
		if line.len() > 0 {
			self.terminated = line[line.len()-1] == "\r\n"
		} else {
			self.terminated = false;
		}
		self.length = self.raw.len();
	}
	/// Instantiate SsdpRequest object
	pub fn new(payload: str, blocked: bool) -> SsdpRequest {
		SsdpRequest {
			method: "",
			uri: "",
			protocol: "",
			host: "",
			target: "",
			man: "",
			timeout: 1,
			responded: true,
			raw: Vec::new(),
			responded: !blocked
		}
	}
	fn post(self) {
		/// Upload object to elasticsearch
	}
}
