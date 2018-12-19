#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

extern crate rocket_contrib;
extern crate serde; 
extern crate reqwest;

use rocket_contrib::json::Json;

const HOSTS: &'static [&'static str] = &["http://localhost:8000", "http://localhost:8001"];

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    text: String,
    host: String
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/message", format="application/json", data = "<message>")]
fn receive_message<'a>(message: Json<Message>) -> &'static str {
    println!("{}", message.text);
    println!("{}", message.host);

    let body = reqwest::get(&message.host).unwrap()
        .text().unwrap();

    println!("body = {:?}", body);
    "Message Received"
} 

fn main() {
    rocket::ignite().mount("/", routes![index, receive_message]).launch();
}
