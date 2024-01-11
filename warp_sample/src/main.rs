use std::convert::Infallible;
use warp::{Filter, Reply};
use serde::Serialize;

#[derive(Serialize)]
struct Message {
    message: String,
}


async fn hello() -> Result<impl warp::Reply, Infallible> {
    let message = "warp: Hello, world!".to_string();
    Ok(warp::reply::json(&message))
}

fn routes() -> impl Filter<Extract = impl Reply, Error = warp::Rejection> + Clone {
    warp::path!("hello")
        .and(warp::get())
        .and_then(hello)
}

#[tokio::main]
async fn main() {
    println!("launching server...");
    let routes = routes();
    warp::serve(routes)
        .run(([0, 0, 0, 0], 8085))
        .await;
}