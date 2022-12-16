#[derive(PartialEq, Eq)]
pub enum RpsShape {
    ROCK,
    PAPER,
    SCISSORS,
}

impl RpsShape {
    fn wins_against(&self, other: RpsShape) -> bool {
        if *self == other {
            return false;
        }

        match *self {
            Self::ROCK => other == Self::SCISSORS,
            Self::PAPER => other == Self::ROCK,
            Self::SCISSORS => other == Self::PAPER,
        }
    }
}

pub struct RpsRound {
    pub(crate) opponent: RpsShape,
    pub(crate) player: RpsShape,
}

impl RpsRound {
    pub fn into_score(self) -> u32 {
        let is_draw = self.player == self.opponent;
        let is_win = self.player.wins_against(self.opponent);

        let mut score = if is_draw { 3 } else if is_win { 6 } else { 0 };

        score += player_points_by_shape(self.player);

        score
    }
}

fn player_points_by_shape(shape: RpsShape) -> u32 {
    use RpsShape::{ROCK, PAPER, SCISSORS};

    match shape {
        ROCK => 1,
        PAPER => 2,
        SCISSORS => 3,
    }
}