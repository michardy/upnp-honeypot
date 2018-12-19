#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]

#[macro_use] extern crate rocket;

use rocket::fairing::AdHoc;

#[get("/rootDesc.xml")]
fn index() -> &'static str {
	include_str!("description.xml")
}

fn main() {
	rocket::ignite()
		.mount("/", routes![index])
		.attach(AdHoc::on_response("Reset server", |_, res| {
			res.set_raw_header("Server", "Linux/2.6, UPnP/1.0, miniupnpd/1.0");
		}))
		.launch();
}
