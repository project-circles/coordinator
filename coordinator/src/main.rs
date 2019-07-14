//! This crate encloses the core coordinator web application. 
//#![deny(warnings)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/")]
/// Root web page for the coordinator.
fn index() -> &'static str {
    "Project Circles: Coordinator"
}

/// Main thread running the web application.
fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
