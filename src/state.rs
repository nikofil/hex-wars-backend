use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::sync::MutexGuard;
use rand;
use rand::Rng;
use rand::distributions::Alphanumeric;

pub struct CellState {
    pub id: u32,
    pub q: i32,
    pub r: i32,
    pub s: i32,
    pub owner: u32,
    pub power: u32,
}

pub struct GameState {
    pub started: bool,
    pub cur_player: u32,
    pub num_players: u32,
    pub cell_state: Vec<CellState>,
    max_players: u32,
    player_secrets: Vec<String>,
}

impl GameState {
    fn new() -> GameState {
        GameState {
            started: false,
            cur_player: 0,
            num_players: 0,
            max_players: 5,
            player_secrets: Vec::new(),
            cell_state: Vec::new(),
        }
    }

    pub fn new_player(&mut self) -> Option<(String, u32)> {
        if self.num_players < self.max_players && !self.started {
            let secret: String = rand::thread_rng().sample_iter(&Alphanumeric).take(32).collect();
            self.player_secrets.push(secret.clone());
            let player_num = self.num_players;
            self.num_players += 1;
            Some((secret, player_num))
        } else {
            None
        }
    }

    pub fn start_game(&mut self) -> Option<()> {
        if self.num_players > 0 {
            let mut id = 0;
            let mut rng = rand::thread_rng();
            for q in -3..3 {
                for r in -3..3 {
                    self.cell_state.push(CellState {
                        id,
                        q,
                        r,
                        s: -q - r,
                        owner: rng.gen_range(0, self.num_players),
                        power: rng.gen_range(0, 5),
                    });
                    id += 1;
                }
            }
            self.started = true;
            Some(())
        } else {
            None
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
        let game_list = GameList {
            games: Arc::new(Mutex::new(HashMap::new()))
        };
        game_list.games.lock().unwrap().insert(1, GameState::new());
        game_list
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
