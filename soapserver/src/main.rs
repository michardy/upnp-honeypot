#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]

#[macro_use] extern crate rocket;

extern crate quick_xml;

use rocket::fairing::AdHoc;
use rocket::Data;

// Device Descriptions

mod soap;

/// Root device description
#[get("/rootDesc.xml")]
fn index() -> &'static str {
	include_str!("description.xml")
}

/// Description of LAN service
#[get("/lan.xml")]
fn lan() -> &'static str {
	include_str!("lan.xml")
}

/// Description of WANIP service
#[get("/Public_UPNP_WANIPConn.xml")]
fn ip() -> &'static str {
	include_str!("wanip.xml")
}

/// WAN Point to Point Protocol description
#[get("/Public_UPNP_WANPPPConn.xml")]
fn ppp() -> &'static str {
	include_str!("wanppp.xml")
}

/// WANIP SOAP handler
#[post("/Public_UPNP_C3", data="<xml>")]
fn wan_ip_soap(xml: Data) -> &'static str {
	let xml_stream = data.open();
	soap::Soap::parse(xml_stream);
	"test"
}


fn main() {
	rocket::ignite()
		.mount("/", routes![index, idg, wap])
		.attach(AdHoc::on_response("Reset server", |_, res| {
			res.set_raw_header("Server", "Linux/2.6, UPnP/1.0, miniupnpd/1.0");
		}))
		.launch();
}
