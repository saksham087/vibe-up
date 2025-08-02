use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)]
pub struct SignupOutput{
    pub id: String
}