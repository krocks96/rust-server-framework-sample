use rocket::*;
use rocket::serde::{Serialize, json::Json};
use rocket::http::Status;

#[derive(Serialize)]
struct Message {
    message: String,
}

fn configure(addr: &str, port: u16) -> rocket::Config {
    rocket::Config {
        address: addr.parse().unwrap(),
        port: port,
        ..rocket::Config::default()
    }
}

#[get("/hello")]
fn hello() -> (Status, Json<Message>) {
    let data = Message {
        message: "Rocket: Hello, world!".to_string(),
    };
    (Status::Ok, Json(data))
}

#[rocket::main]
async fn main() {
    println!("launching server...");
    let config = configure("0.0.0.0", 8082);
    rocket::build()
        .configure(config)
        .mount("/", routes![hello])
        .launch()
        .await
        .unwrap();
}
