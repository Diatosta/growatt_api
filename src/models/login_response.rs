use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct LoginResponse {
    pub result: i32,
}
