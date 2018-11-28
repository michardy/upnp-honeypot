use chrono::prelude::*;
use std::collections::HashSet;

#[derive(Serialize)]
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
	method: Option<String>,
	/// Resource URI. Must be *
	uri: Option<String>,
	/// Protocol and version. Generally HTTP/1.1
	protocol: Option<String>,
	/// Status code
	status: Option<String>,
	/// Reason (Ex: for status code of 200 reason is OK)
	reason: Option<String>,
	// Search
	/// Target device address or SSDP multicast address
	host: Vec<String>,
	/// The "MAN" field. If set must be `ssdp:discover`
	man: Vec<String>,
	/// The "MX" field. Max number of seconds for random response delay.
	/// Ranges from 1 to 5
	delay: Vec<String>,
	/// The "ST" field. If set contains selector describing devices that should respond
	target: Vec<String>,
	/// User-Agent
	user_agent: Vec<String>,
	// Notification and response
	/// The "CACHE-CONTROL" field
	cache_control: Vec<String>,
	/// The url of the root device description
	location: Vec<String>,
	/// The "NT" field. Notification type
	notification: Vec<String>,
	/// The "NTS" field. Sub notification type
	sub_notification: Vec<String>,
	/// The software and version information
	server: Vec<String>,
	/// The "USN" field. The unique service name of a UPNP service
	service: Vec<String>,
	/// The "BOOTID.UPNP.ORG" field. Boot ID. 31 bit unsigned int (Don't question it)
	boot: Vec<String>,
	/// The "NEXTBOOTID.UPNP.ORG" field. Sent if multihomed device changes connectivity on another network
	next_boot: Vec<String>,
	/// The "CONFIGID.UPNP.ORG" field. Sent when config is updated.
	config: Vec<String>,
	/// The "SEARCHPORT.UPNP.ORG" field. Used for setting nonstandard search ports
	port: Vec<String>,
	// Response only
	/// Response generation date
	date: Vec<String>,
	/// The "EXT" field. Required for backwards compatability
	extension: Vec<String>,
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
	length: usize,
	/// Raw request content
	/// Created because unrecognized fields get dropped
	raw: String
}

impl SsdpRequest {
	fn check_headers(&mut self, lines: &Vec<String) {
		for line in lines {
			let parts: Vec<String> = line.split(": ").map(|x| x.to_string()).collect();
			let headers: HashSet<&'static str> = [
				"HOST",
				"MAN",
				"MX",
				"ST",
				"USER-AGENT",
				"CACHE-CONTROL",
				"LOCATION",
				"NT",
				"NTS",
				"SERVER",
				"USN",
				"BOOTID.UPNP.ORG",
				"NEXTBOOTID.UPNP.ORG",
				"CONFIGID.UPNP.ORG",
				"SEARCHPORT.UPNP.ORG",
				"DATE",
				"EXT"
			].iter().cloned().collect();
			if !parts[0].contains(header)) {
				self.unknown = true;
				return
			}
		}
		self.unknown = false;
	}
	/// Parses status header on SSDP's HTTP over UDP packets
	fn parse_httpu(&mut self, lines: &Vec<String>) {
		let parts: Vec<String> = lines[0].split(' ').map(|x| x.to_string()).collect();
		if parts[0] == "HTTP/1.1" {
		} else {
			self.protocol = Some(parts[0].clone());
			self.status = Some(parts[1].clone());
			self.reason = Some(parts[2].clone());
		}
		self.method = Some(parts[0].clone());
		self.uri = Some(parts[1].clone());
		self.protocol = Some(parts[2].clone());
	}
	/// Searches for and parses M-SEARCH parameters
	fn parse_search(&mut self, lines: &Vec<String>) {
		for line in lines {
			let parts: Vec<String> = line.split(": ").map(|x| x.to_string()).collect();
			if parts[0] == "HOST" {
				self.host.push(parts[1].clone());
			} else if parts[0] == "MAN" {
				self.man.push(parts[1].clone());
			} else if parts[0] == "MX" {
				self.delay.push(parts[1].clone());
			} else if parts[0] == "ST" {
				self.target.push(parts[1].clone());
			} else if parts[0] == "USER-AGENT" {
				self.user_agent.push(parts[1].clone());
			}
		}
	}
	/// Searches for and parses NOTIFY and response headers
	fn parse_notify(&mut self, lines: &Vec<String>) {
		for line in lines {
			let parts: Vec<String> = line.split(": ").map(|x| x.to_string()).collect();
			if parts[0] == "CACHE-CONTROL" {
				self.cache_control.push(parts[1].clone());
			} else if parts[0] == "LOCATION" {
				self.location.push(parts[1].clone());
			} else if parts[0] == "NT" {
				self.notification.push(parts[1].clone());
			} else if parts[0] == "NTS" {
				self.sub_notification.push(parts[1].clone());
			} else if parts[0] == "SERVER" {
				self.server.push(parts[1].clone());
			} else if parts[0] == "USN" {
				self.service.push(parts[1].clone());
			} else if parts[0] == "BOOTID.UPNP.ORG" {
				self.boot.push(parts[1].clone());
			} else if parts[0] == "NEXTBOOTID.UPNP.ORG" {
				self.next_boot.push(parts[1].clone());
			} else if parts[0] == "CONFIGID.UPNP.ORG" {
				self.config.push(parts[1].clone());
			} else if parts[0] == "SEARCHPORT.UPNP.ORG" {
				self.port.push(parts[1].clone());
			}
		}
	}
	/// Parses response headers
	fn parse_response(&mut self, lines: &Vec<String>) {
		for line in lines {
			let parts: Vec<String> = line.split(": ").map(|x| x.to_string()).collect();
			if parts[0] == "DATE" {
				self.date.push(parts[1].clone());
			} else if parts[0] == "EXT" {
				self.extension.push(parts[1].clone());
			}
		}
	}
	/// Parse a recieved SSDP request
	fn parse(&mut self) {
		let lines:Vec<String> = self.raw.split("\r\n").map(|x| x.to_string()).collect();
		self.parse_httpu(&lines);
		self.parse_search(&lines);
		self.parse_notify(&lines);
		self.parse_response(&lines);
		self.check_headers(&lines);
		self.generate(&lines);
	}
	/// Check the combination headers to see if they are valid
	fn check_combo(&mut self) {
		if host.len > 0 && host.len < 2 {
		}
	}
	/// Populate generated fields
	fn generate(&mut self, lines: &Vec<String>) {
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
		if lines.len() > 0 {
			self.terminated = lines[lines.len()-1] == "\r\n"
		} else {
			self.terminated = false;
		}
		self.length = self.raw.len();
	}
	/// Instantiate SsdpRequest object
	pub fn new(payload: String, blocked: bool) -> SsdpRequest {
		let mut req = SsdpRequest {
			timestamp: Some(Utc::now()),
			method: None,
			uri: None,
			protocol: None,
			status: None,
			reason: None,
			host: Vec::new(),
			man: Vec::new(),
			delay: Vec::new(),
			target: Vec::new(),
			user_agent: Vec::new(),
			cache_control: Vec::new(),
			location: Vec::new(),
			notification: Vec::new(),
			sub_notification: Vec::new(),
			server: Vec::new(),
			service: Vec::new(),
			boot: Vec::new(),
			next_boot: Vec::new(),
			config: Vec::new(),
			port: Vec::new(),
			date: Vec::new(),
			extension: Vec::new(),
			responded: !blocked,
			valid: false,
			valid_httpu: false,
			unknown: false,
			skipped: false,
			overwrite: false,
			terminated: false,
			combination: false,
			length: 0usize,
			raw: payload
		};
		req.parse();
		req
	}
	/// Upload object to elasticsearch
	pub fn post(self, client: &elastic_index::Client) {
		elastic_index::Index::new("ssdp".to_string()).index(&client, self);
	}
}
