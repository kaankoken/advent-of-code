use crate::part2::Move;

#[derive(Clone, Copy, Debug)]
pub enum Outcome {
    Loss,
    Draw,
    Win,
}

impl TryFrom<char> for Outcome {
    type Error = color_eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(color_eyre::eyre::eyre!("not a valid outcome {value:?}")),
        }
    }
}

impl Outcome {
    pub fn inherited_points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }

    pub fn matching_move(self, opponent: Move) -> Move {
        match self {
            Outcome::Win => opponent.winning_move(),
            Outcome::Draw => opponent.drawing_move(),
            Outcome::Loss => opponent.losing_move(),
        }
    }
}
