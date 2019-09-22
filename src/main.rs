#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod state;
use state::GameList;
use rocket::State;

#[get("/")]
fn index(games: State<GameList>) -> &'static str {
    println!("hi!");
    games.new_game();
    let mut x = games.game_state(10);
    match x.borrow_mut() {
        None => "got none",
        Some(_) => "got some",
    }
}

fn main() {
    let games = GameList::new();
    rocket::ignite()
        .manage(games)
        .mount("/", routes![index]).launch();
}
