use std::sync::{Arc, Mutex};

use poem::{handler, web::{Data, Json}};
use db::{models::users, connection::Connect};


use crate::{request_input::SignupInput, request_output::SignupOutput};



#[handler]
pub fn sign_up(Json(data): Json<SignupInput>, Data(con): Data<&Arc<Mutex<Connect>>>) -> Json<SignupOutput>{
    let mut locked_con = con.lock().unwrap();
    let id = locked_con.sign_up(data.input_username, data.input_password, data.input_name).unwrap();
    eprintln!("i got the id: {}", id);
    Json(SignupOutput{
        id
    })
}