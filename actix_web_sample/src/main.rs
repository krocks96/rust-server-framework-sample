#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    println!("launching server...");
    actix_web_sample::apis::http_start("0.0.0.0", 8080).await
}