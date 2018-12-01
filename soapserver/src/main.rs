#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]

#[macro_use] extern crate rocket;

#[get("/description.xml")]
fn index() -> &'static str {
	include_str!("description.xml")
}

/* TODO: SET HEADERS to include some of and nothing else
 * CONNECTION: close
 * CONTENT-TYPE: text/xml
 * DATE: Tue, 11 Dec 2007 09:13:18 GMT
 * LENGTH: 1057
 * MODIFIED: Tue, 11 Dec 2007 09:13:18 GMT
 * SERVER: Linux/2.6.5-it0, UPnP/1.0, Intel SDK for UPnP devices /1.2
 */
fn main() {
	rocket::ignite()
		.mount("/", routes![index])
		.launch();
}
