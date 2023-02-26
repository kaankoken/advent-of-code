#[derive(Clone, Copy, Debug)]
pub enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    pub fn inherited_points(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}
