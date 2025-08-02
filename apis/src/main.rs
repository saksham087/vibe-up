use std::sync::{Arc, Mutex};

use poem::{listener::TcpListener, post, EndpointExt, Route, Server};
use crate::routes::user::sign_up;
use db::connection::Connect;
pub mod request_input;
pub mod request_output;
pub mod routes;



#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let con = Arc::new(Mutex::new(Connect::default()));
    let app = Route::new()
        .at("/api/sign_up", post(sign_up))
        .data(con);
    
    
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}