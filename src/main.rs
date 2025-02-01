use axum::{routing::{get, post}, serve, Router};
use controller::{get_info, login_handler};



mod model;
mod controller;

#[tokio::main]
async fn main() {

    let app = Router::new()
    .route("/login", post(login_handler))
    .route("/infor",get(get_info));

    let listener=tokio::net::TcpListener::bind("0.0.0.0:3038")
    .await.unwrap();

    println!("Listening...");

    let server = serve(listener, app)
    .await.unwrap();

}
