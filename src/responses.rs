use crate::state::GameState;
use crate::state::CellState;

use serde::Serialize;
use rocket::response::status::Custom;
use rocket::http::Status;
use rocket_contrib::json::Json;

#[serde(rename_all = "camelCase")]
#[derive(Serialize)]
pub struct NewGameResponse {
    pub game_id: u32,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize)]
pub struct CellStateResponse {
    id: u32,
    q: i32,
    r: i32,
    s: i32,
    owner: u32,
    power: u32,
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize)]
pub struct GameStateResponse {
    started: bool,
    cur_player: u32,
    num_players: u32,
    hex: Vec<CellStateResponse>,
}

impl GameStateResponse {
    pub fn new(state: &GameState) -> GameStateResponse {
        GameStateResponse {
            started: state.started,
            cur_player: state.cur_player,
            num_players: state.num_players,
            hex: state.cell_state.iter().map(|cell| {
                let CellState { id, q, r, s, owner, power } = *cell;
                CellStateResponse { id, q, r, s, owner, power }
            }).collect::<Vec<CellStateResponse>>(),
        }
    }
}

#[serde(rename_all = "camelCase")]
#[derive(Serialize)]
pub struct JoinGameResponse {
    pub player_secret: String,
    pub player_id: u32,
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

pub fn not_found(err: &'static str) -> Custom<Json<ErrorResponse>> {
    Custom(Status::NotFound, Json(ErrorResponse::new(err)))
}

pub fn forbidden(err: &'static str) -> Custom<Json<ErrorResponse>> {
    Custom(Status::Forbidden, Json(ErrorResponse::new(err)))
}
