use crate::part2::outcome_model::Outcome;

#[derive(Clone, Copy, Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<char> for Move {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Move::Rock),
            'B' => Ok(Move::Paper),
            'C' => Ok(Move::Scissors),
            _ => Err(color_eyre::eyre::eyre!("Invalid move: {value:?}")),
        }
    }
}

impl Move {
    fn beats(self, other: Move) -> bool {
        matches!(
            (self, other),
            (Self::Rock, Self::Scissors)
                | (Self::Paper, Self::Rock)
                | (Self::Scissors, Self::Paper)
        )
    }

    pub fn inherited_points(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    pub fn outcome(self, opponent: Move) -> Outcome {
        if self.beats(opponent) {
            Outcome::Win
        } else if opponent.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
}

impl Move {
    const ALL_MOVES: [Move; 3] = [Move::Rock, Move::Paper, Move::Scissors];

    pub fn winning_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|m| m.beats(self))
            .expect("at least one move to beat us")
    }

    pub fn losing_move(self) -> Self {
        Self::ALL_MOVES
            .iter()
            .copied()
            .find(|&m| self.beats(m))
            .expect("we beat at least one move")
    }

    pub fn drawing_move(self) -> Self {
        self
    }
}
