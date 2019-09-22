#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate serde;
extern crate serde_json;
extern crate rocket_contrib;

mod state;
use state::GameList;
mod responses;
use responses::{ NewGameResponse, ErrorResponse};

use rocket::State;
use rocket::response::status::NotFound;
use rocket_contrib::json::Json;

#[get("/new_game")]
fn new_game(games: State<GameList>) -> Json<NewGameResponse> {
    let game_id = games.new_game();
    Json(NewGameResponse {
        game_id
    })
}

#[get("/game_state/<id>")]
fn game_state(id: u32, games: State<GameList>) -> Result<Json<ErrorResponse>, NotFound<Json<ErrorResponse>>> {
    let mut state = games.game_state(id);
    match state.borrow_mut() {
        None => Err(NotFound(Json(ErrorResponse::new("game not found")))),
        Some(game) => {
            // let game = game;
            Ok(Json(ErrorResponse::new("game found")))
        },
    }

}

fn main() {
    let games = GameList::new();
    rocket::ignite()
        .manage(games)
        .mount("/", routes![new_game, game_state]).launch();
}
