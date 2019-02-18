use quick_xml::Reader;
use quick_xml::events::Event;

use rocket::data::DataStream;

pub struct Soap {
	method: String,
	parameters: Vec<String>
}

impl Soap {
	pub fn new(method: String, parameters: Vec<String>) -> Soap {
		Soap {
			method: String,
			parameters: parameters
		}
	}
	pub fn parse(xml: DataStream) -> Soap {
		let mut buf = Vec::new();
		let mut ns_buf = Vec::new();
		let mut reader = Reader::from_reader(xml);
		reader.trim_text(true);
		loop {
			match reader.read_namespaced_event(&mut buf, &mut ns_buf) {
				Ok((ref ns, Event::Start(ref e))) => {
					match e.name() {
						b"Envelope" => {
							println!("attributes values: {:?}",
								e.attributes().map(|a| a.unwrap().value)
									.collect::<Vec<_>>()
							);
							println!("namespace: {:?}", ns);
							},
						b"tag2" => count += 1,
						_ => (),
					}
				},
				Ok(Event::Text(e)) => {},
				Ok(Event::Eof) => break,
				Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
				_ => (),
			}
			buf.clear();
		}
		Soap::new(String::from("test"), Vec![])
	}
	pub fn generate(self) -> String {
		String::from("")
	}
}
