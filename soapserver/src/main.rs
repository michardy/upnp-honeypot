#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]

#[macro_use] extern crate rocket;

#[get("/rootDesc.xml")]
fn index() -> &'static str {
	include_str!("description.xml")
}

/* TODO: SET HEADERS to include some of and nothing else
 * CONNECTION: close
 * CONTENT-TYPE: text/xml
 * DATE: Tue, 11 Dec 2007 09:13:18 GMT
 * LENGTH: 1057
 * MODIFIED: Tue, 11 Dec 2007 09:13:18 GMT
 * SERVER: Linux/2.6, UPnP/1.0, miniupnpd/1.0
 */
fn main() {
	rocket::ignite()
		.mount("/", routes![index])
		.launch();
}
