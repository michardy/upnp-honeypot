extern crate chrono;
extern crate redis;
extern crate serde;
extern crate rmp_serde as rmps;
extern crate elastic_index;

#[macro_use]
extern crate serde_derive;

mod ip;
mod ssdp;

use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
	{
		let redis_client = (redis::Client::open("redis://127.0.0.1/")).unwrap();
		let elastic_client = elastic_index::Client::new("http://localhost:9200".to_string());
		let con = (redis_client.get_connection()).unwrap();

		let socket = UdpSocket::bind("127.0.0.1:1900")?;

		loop {
			// Allocate 4096 byte packet buffer
			// Must be at least 2500 to allow collection of full shellcodes
			// See libupnp Exploitability section here: https://information.rapid7.com/rs/411-NAK-970/images/SecurityFlawsUPnP%20(1).pdf
			let mut buf = [0; 4096];

			let (amt, src) = socket.recv_from(&mut buf)?;
			let address = format!("{}", src.ip());
			let address_listing = ip::Ip::get(&address, &con);

			let buf = &mut buf[..amt];
			let req = ssdp::SsdpRequest::new(
				String::from_utf8_lossy(buf).into_owned(),
				address_listing.blacklisted
			);
			req.post(&elastic_client);
			if !address_listing.blacklisted {
				socket.send_to(include!("ssdp.response"), &src)?;
			}
			address_listing.write(&address, &con)
				.expect("Could not write IP object");
		}
	} // the socket is closed here
	Ok(())
}
