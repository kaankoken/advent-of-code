mod part1;

use part1::Round;

fn main() -> color_eyre::Result<()> {
    /*
        First column: opponent's move
        Second column: your move

        A for rock
        B for paper
        C for scissors

        X for rock
        Y for paper
        Z for scissors

       [total score] is the sum of your scores for each round.

        A Y -> (2 + 6) = 8 points Paper (Y, 2 points) & winning (6 points)
        B X -> (1 + 0) = 1 point Rock (X, 1 point) & losing (0 point)
        C Z -> (3 + 3) = 6 points Scissors (Z, 3 points) & tie (3 points)

        total = 8 + 1 + 6 = 15 points
    */

    color_eyre::install()?;

    let rounds: Vec<Round> = include_str!("input.txt")
        .lines()
        .map(|line| line.parse::<Round>())
        .collect::<Result<_, _>>()?;

    /*
        Alternatively
        let rounds: Vec<Round> = include_str!("input.txt")
            .lines()
            .map(Round::from_str)
            .collect::<Result<_, _>>()?;
    */

    let score: usize = rounds.iter().map(|round| round.our_score()).sum();

    /* Alternative solution
       let mut score = 0;

       for round in include_str!("input.txt").lines().map(Round::from_str) {
           score += round?.our_score();
       }
    */
    println!("{score:?}");
    Ok(())
}
