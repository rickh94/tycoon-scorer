use std::collections::HashMap;

#[derive(Clone)]
pub enum Msg {
    GoOut(usize),
    EndRound,
    NewGame,
    Setup,
    SetupComplete,
    AddPlayer(String, usize),
    MorePlayers,
    SavePlayers,
}


#[derive(Debug)]
pub struct SetupState {
    pub num_of_inputs: usize,
    pub player_names: HashMap<usize, String>,
}

impl SetupState {
    pub fn new() -> Self {
        Self {
            num_of_inputs: 1,
            player_names: HashMap::new(),
        }
    }
}
