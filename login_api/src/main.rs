use std::net::SocketAddr;
use hyper::StatusCode;
use models::NewUser;
use axum::{
    routing::{get, post},
    Router, Json, Error, response::{IntoResponse, self}, http,
};
mod models;
mod db;

async fn login_handler() -> impl IntoResponse {
    "Login handler not implemented"
}

// async fn login_handler(user: Json<User>) -> impl IntoResponse {
//     let user = user.0;
//     let user = User::find_by_username(&user.username).await;
//     match user.is_password_match(&user.password) {
//         true => Ok("Login successful"),
//         false => Ok("Login failed"),
//     }
// }
async fn register_handler(Json(body): Json<NewUser>) -> Result<response::Json<String>, StatusCode> {
    // Wykonaj operacje zapisu użytkownika w bazie danych
    println!("Register handler");
    println!("{:?}", body);
    match body.save_user().await {
        Ok(_) => Ok(response::Json("User created".to_owned())),
        Err(e) => {
            println!("Failed to save user: {}", e);
            Err(http::StatusCode::INTERNAL_SERVER_ERROR)
            
        }
    }
}


#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
                    .route("/", get(|| async { "Hello, World!" }))
                    .route("/login", post(login_handler))
                    .route("/register", post(register_handler));

    let port = 3000;
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    
    println!("Server listening on {}", &address);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
} 