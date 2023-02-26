use std::str::FromStr;

use crate::part1::{Move, Outcome};

#[derive(Clone, Copy, Debug)]
pub struct Round {
    opponent: Move,
    ours: Move,
}

impl FromStr for Round {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let (Some(opponent), Some(' '), Some(ours), None) = (chars.next(), chars.next(), chars.next(), chars.next()) else {
            return Err(color_eyre::eyre::eyre!("expected <opponent>SP<ours>EOF, got{s:?}"));
        };

        Ok(Self {
            opponent: opponent.try_into()?,
            ours: ours.try_into()?,
        })
    }
}

impl Round {
    pub fn outcome(self) -> Outcome {
        self.ours.outcome(self.opponent)
    }

    pub fn our_score(self) -> usize {
        self.ours.inherited_points() + self.outcome().inherited_points()
    }
}
