use seed::{*, prelude::*};
use std::collections::HashMap;
use std::cmp::Ordering;

mod player;
mod components;
mod msg;

use crate::player::{Player, Rank};
use crate::components::{header, setup_mode, game_over_mode, score_table};
use crate::msg::{Msg, SetupState};


#[derive(Debug)]
enum Mode {
    Normal,
    Setup,
    GameOver,
}


#[derive(Debug)]
struct Model {
    players: HashMap<usize, Player>,
    round: u8,
    mode: Mode,
    setup_state: SetupState,
    players_out: HashMap<usize, Rank>,
}


impl Default for Model {
    fn default() -> Self {
        let mut players = HashMap::new();
        players.insert(0, Player::new("one"));
        Self {
            players,
            round: 1,
            mode: Mode::Normal,
            setup_state: SetupState::new(),
            players_out: HashMap::new(),
        }
    }
}

impl Model {
    pub fn find_tycoon(&self) -> Option<usize> {
        for (i, player) in self.players.iter() {
            if player.is_tycoon() {
                return Some(*i);
            }
        }
        None
    }

    pub fn find_last_not_out(&self) -> Option<usize> {
        for (i, _) in self.players.iter() {
            if !self.players_out.contains_key(i) {
                return Some(*i);
            }
        }
        None
    }

    // OH THE HUMANITY
    pub fn handle_go_out(&mut self, pid: usize) {
        match self.players_out.len() {
            0 => {
                // First person to go out is tycoon. period.
                self.players_out.insert(pid, Rank::Tycoon);
                // In round one this has no side effects, but otherwise
                if self.round != 1 {
                    match self.find_tycoon() {
                        Some(tid) => {
                            // If the person who went out is not already the tycoon, the tycoon
                            // goes bankrupt and is automatically the beggar
                            if tid != pid {
                                self.players_out.insert(tid, Rank::Beggar);
                            }
                        }
                        None => (),
                    }
                };
            }
            1 => {
                // The only time when this will occur is round 1 or the first out was already
                // the tycoon.
                self.players_out.insert(pid, Rank::Rich);
            }
            2 => {
                // If the tycoon did not go out first, then there will already be a beggar
                let contains_beggar = self.players_out.values().any(|x| *x == Rank::Beggar);
                // if there is a beggar, next to go out is rich, and last is poor
                if contains_beggar {
                    self.players_out.insert(pid, Rank::Rich);
                    match self.find_last_not_out() {
                        Some(lid) => {
                            self.players_out.insert(lid, Rank::Poor);
                        }
                        None => (),
                    }
                } else {
                    // if not, then we have poor then beggar
                    self.players_out.insert(pid, Rank::Poor);
                    match self.find_last_not_out() {
                        Some(lid) => {
                            self.players_out.insert(lid, Rank::Beggar);
                        }
                        None => (),
                    }
                };
            }
            _ => ()
        }
    }

    pub fn handle_end_round(&mut self) {
        for (i, player) in self.players.iter_mut() {
            match self.players_out.get(i) {
                Some(r) => {
                    player.set_rank(r.clone());
                    player.update_score()
                }
                None => (),
            }
        }
        self.players_out = HashMap::new();
        self.round += 1;
    }

    pub fn get_ranking(&self) -> Vec<Player> {
        let mut players: Vec<Player> = self.players.iter().map(|(_, p)| p.clone()).collect();
        players.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(Ordering::Less));
        players
    }

    pub fn new_game(&mut self) {
        for (_, player) in self.players.iter_mut() {
            player.score = 0;
            player.rank = None;
            player.past_ranks = vec![];
        }
        self.round = 1;
        self.players_out = HashMap::new();
        self.mode = Mode::Normal;
    }
}

fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Setup => model.mode = Mode::Setup,
        Msg::SetupComplete => model.mode = Mode::Normal,
        Msg::AddPlayer(name, idx) => {
            model.setup_state.player_names.insert(idx, name);
        }
        Msg::MorePlayers => model.setup_state.num_of_inputs += 1,
        Msg::SavePlayers => {
            for (i, name) in model.setup_state.player_names.iter() {
                model.players.insert(*i, Player::new(name.as_str()));
            }
            model.setup_state.player_names = HashMap::new();
            model.setup_state.num_of_inputs = 1;
            model.new_game();
        }
        Msg::GoOut(pid) => {
            model.handle_go_out(pid);
        }
        Msg::EndRound => {
            if model.players_out.len() < model.players.len() {
                return;
            }
            if model.round == 3 {
                model.mode = Mode::GameOver;
            }
            model.handle_end_round();
        }
        Msg::NewGame => {
            model.new_game();
        }
    }
}




fn view(model: &Model) -> impl IntoNodes<Msg> {
    let can_end_round = model.players_out.len() == model.players.len();
    div![
        header(),
        match model.mode {
            Mode::Normal => {
                div! [
                    class! ["flex", "w-full", "justify-center", "mt-2", "max-w-3xl", "mx-auto", "flex-col"],
                    div! [
                        class! ["flex", "w-full", "justify-center", "mt-2"],
                        button![
                            class! ["px-4", "py-2", "bg-indigo-600", "hover:shadow", "hover:bg-indigo-800", "text-white", "rounded-full", "mx-2"],
                            simple_ev(Ev::Click, Msg::Setup),
                            "Setup"
                        ],
                        button![
                            class! ["px-4", "py-2", "bg-red-600", "hover:shadow", "hover:bg-red-800", "text-white", "rounded-full", "mx-2"],
                            simple_ev(Ev::Click, Msg::NewGame),
                            "New Game"
                        ],
                    ],
                    div! [
                        class! ["flex", "w-full", "justify-center", "mt-2"],
                        score_table(&model.players, &model.players_out),
                    ],
                    div![
                        class!["flex", "w-full", "mt-4", "justify-center", "max-w-3xl", "mx-auto"],
                        button![
                            class! [
                                "px-6",
                                "py-4",
                                "rounded-full",
                                "font-bold",
                                "text-white",
                                "hover:shadow" => can_end_round,
                                "bg-indigo-600" => can_end_round,
                                "hover:bg-indigo-800" => can_end_round,
                                "bg-gray-600" => !can_end_round,
                            ],
                            attrs!{At::Disabled => (!can_end_round).as_at_value()},
                            simple_ev(Ev::Click, Msg::EndRound),
                            "End Round"
                        ],
                    ],
                ]
            },
            Mode::GameOver => {
                div![
                    class! ["flex", "w-full", "justify-center", "mt-2", "max-w-3xl", "mx-auto", "flex-col", "max-w-3xl"],
                    div![
                        class!["flex", "justify-center", "w-full"],
                        button![
                            class! ["px-4", "py-2", "bg-red-600", "flex-grow-0", "hover:shadow", "hover:bg-red-800", "text-white", "rounded-full", "mx-2"],
                            simple_ev(Ev::Click, Msg::NewGame),
                            "New Game"
                        ],
                    ],
                    game_over_mode(model.get_ranking()),
                ]
            }
            Mode::Setup => {
                div! [
                    class! ["flex", "w-full", "justify-center", "mt-2"],
                    setup_mode(&model.setup_state),
                ]
            }
        }
    ]
}

#[wasm_bindgen(start)]
pub fn render() {
    App::builder(update, view).build_and_start();
}
