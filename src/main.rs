mod handlers;
mod response;

use warp::{http::Method, Filter, Rejection};

type WebResult<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let health_checker = warp::path!("api" / "health")
        .and(warp::get())
        .and_then(handlers::health_checker_handler);
    
    let cors = warp::cors()
        .allow_methods(&[Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_origins(vec!["http://localhost:3000/", "http://localhost:8000/"])
        .allow_headers(vec!["content-type"])
        .allow_credentials(true);
    
    let routes = health_checker
        .with(cors)
        .with(warp::log("api"));

    println!("🚀 Server started successfully");

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}