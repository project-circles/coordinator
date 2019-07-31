//! This crate encloses the core coordinator web application.
#![deny(warnings)]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use failure::Error;
use std::thread;
use websocket::sync::Server;
use websocket::Message;

// String that holds address that websocket binds to.
static WEBSOCKET_ADDRESS: &str = "0.0.0.0:9090";

#[get("/")]
/// Root web page for the coordinator.
fn index() -> &'static str {
    "Project Circles: Coordinator"
}

/// Main thread running the web application and websockets server.
fn main() -> Result<(), Error> {
    // Spawn a thread to start web interface front end.
    thread::spawn(move || {
        rocket::ignite().mount("/", routes![index]).launch();
    });

    // Start websocket server where node communication occurs.
    let coordinator = Server::bind(WEBSOCKET_ADDRESS)?;

    // Spawn a thread for each websocket connection.
    for connection in coordinator.filter_map(Result::ok) {
        thread::spawn(move || {
            println!("Client has connected");
            let mut client = connection.accept().expect("Error: Connection failed");
            let message = Message::text("Hello, client!");
            client
                .send_message(&message)
                .expect("Error: Message send failed");
        });
    }
    Ok(())
}
