#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate serde;
extern crate serde_json;
extern crate rocket_contrib;

mod state;
use state::GameList;
mod responses;
use responses::*;

use rocket::State;
use rocket::response::status::Custom;
use rocket_contrib::json::Json;

#[get("/newGame")]
fn new_game(games: State<GameList>) -> Json<NewGameResponse> {
    let game_id = games.new_game();
    Json(NewGameResponse {
        game_id
    })
}

#[get("/joinGame/<id>")]
fn join_game(id: u32, games: State<GameList>) -> Result<Json<JoinGameResponse>, Custom<Json<ErrorResponse>>> {
    let mut state = games.game_state(id);
    match state.borrow_mut() {
        None => Err(not_found("game not found")),
        Some(game) => {
            match game.new_player() {
                None => Err(forbidden("cannot join game")),
                Some(( player_secret, player_id )) => Ok(Json(JoinGameResponse{ player_secret, player_id })),
            }
        },
    }
}

#[put("/startGame/<id>")]
fn start_game(id: u32, games: State<GameList>) -> Result<Json<String>, Custom<Json<ErrorResponse>>> {
    let mut state = games.game_state(id);
    match state.borrow_mut() {
        None => Err(not_found("game not found")),
        Some(game) => {
            match game.start_game() {
                None => Err(forbidden("no players in game")),
                Some(_) => Ok(Json(String::from(""))),
            }

        },
    }
}

#[get("/gameState/<id>")]
fn game_state(id: u32, games: State<GameList>) -> Result<Json<GameStateResponse>, Custom<Json<ErrorResponse>>> {
    let mut state = games.game_state(id);
    match state.borrow_mut() {
        None => Err(not_found("game not found")),
        Some(game) => {
            Ok(Json(GameStateResponse::new(game)))
        },
    }
}

fn main() {
    let games = GameList::new();
    rocket::ignite()
        .manage(games)
        .mount("/", routes![new_game, join_game, start_game, game_state]).launch();
}
