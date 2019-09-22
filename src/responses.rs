use serde::Serialize;

#[serde(rename_all = "camelCase")]
#[derive(Serialize)]
pub struct NewGameResponse {
    pub game_id: u32,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    error: String,
}

impl ErrorResponse {
    pub fn new(error: &str) -> ErrorResponse {
        let error = String::from(error);
        ErrorResponse { error }
    }
}
