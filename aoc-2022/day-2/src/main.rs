use std::str::FromStr;

use part2::Round;

mod part2;

fn main() -> color_eyre::Result<()> {
    /*
        First column: opponent
        Second column: how round ends

        X -> Loss
        Y -> Draw
        Z -> Win

        A Y -> (1 + 3) = 4 points (Rock chosen, 1 points) & Draw (3 points)
        B X -> (1 + 0) = 2 points (Rock chosen, 1 point) & Lose (0 point)
        C Z -> (1 + 6) = 3 points (Rock chosen, 1 points) & Win (6 points)

        total = 4 + 1 + 7 = 12 points
    */

    color_eyre::install()?;

    let rounds: Vec<Round> = include_str!("input_test.txt")
        .lines()
        .map(Round::from_str)
        .collect::<Result<_, _>>()?;

    let score: usize = rounds.iter().map(|round| round.our_score()).sum();

    println!("{score:?}");
    Ok(())
}
