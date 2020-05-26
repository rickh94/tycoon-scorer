
#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub name: String,
    pub score: u8,
    pub rank: Option<Rank>,
    pub past_ranks: Vec<Rank>,
}


#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Rank {
    Beggar,
    Poor,
    Rich,
    Tycoon,
}


impl ToString for Rank {
    fn to_string(&self) -> String {
        match self {
            Self::Beggar => "Beggar".into(),
            Self::Poor => "Poor".into(),
            Self::Rich => "Rich".into(),
            Self::Tycoon => "Tycoon".into(),
        }
    }
}


impl Player {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            score: 0,
            rank: None,
            past_ranks: vec![],
        }
    }

    pub fn set_rank(&mut self, rank: Rank) {
        match self.rank.take() {
            Some(r) => self.past_ranks.push(r),
            None => (),
        }
        self.rank = Some(rank);
    }

    pub fn update_score(&mut self) {
        match &self.rank {
            Some(Rank::Beggar) => (),
            Some(Rank::Poor) => self.score += 10,
            Some(Rank::Rich) => self.score += 20,
            Some(Rank::Tycoon) => self.score += 30,
            None => (),
        };
    }

    pub fn is_tycoon(&self) -> bool {
        match self.rank {
            Some(Rank::Tycoon) => true,
            _ => false
        }
    }
}
