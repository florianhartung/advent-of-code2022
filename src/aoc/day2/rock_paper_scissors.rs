#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RpsShape {
    Rock,
    Paper,
    Scissors,
}

impl RpsShape {
    fn is_winning_against(&self, other: RpsShape) -> bool {
        if *self == other {
            return false;
        }

        other == self.get_wins_against()
    }

    pub fn get_wins_against(&self) -> RpsShape {
        match *self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    pub fn get_looses_against(&self) -> RpsShape {
        match *self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
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
        let is_win = self.player.is_winning_against(self.opponent);

        let mut score = if is_draw { 3 } else if is_win { 6 } else { 0 };

        score += player_points_by_shape(self.player);

        score
    }
}

fn player_points_by_shape(shape: RpsShape) -> u32 {
    use RpsShape::{Rock, Paper, Scissors};

    match shape {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}