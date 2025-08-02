use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SignupInput{
    pub input_username: String,
    pub input_password: String,
    pub input_name: Option<String>
}