use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::sync::MutexGuard;
use rand;

#[derive(Debug)]
pub struct GameState {
    started: bool,
    cur_player: u32,
    num_players: u32,
    max_players: u32,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            started: false,
            cur_player: 0,
            num_players: 0,
            max_players: 5,
        }
    }
}

pub struct GameList {
    games: Arc<Mutex<HashMap<u32, GameState>>>,
}

pub struct GameStateGuard<'a> {
    idx: u32,
    guard: MutexGuard<'a, HashMap<u32, GameState>>,
}

impl GameStateGuard<'_> {
    pub fn borrow_mut(&mut self) -> Option<&mut GameState> {
        self.guard.get_mut(&self.idx)
    }
}

impl GameList {
    pub fn new() -> GameList {
        GameList {
            games: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    pub fn new_game(&self) -> u32 {
        let id = rand::random::<u32>();
        self.games.lock().unwrap().insert(id, GameState::new());
        eprintln!("Made new game with id {}", id);
        id
    }

    pub fn game_state(&self, idx: u32) -> GameStateGuard {
        GameStateGuard {
            guard: self.games.lock().unwrap(),
            idx
        }
    }
}
