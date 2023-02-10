use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    // Initialization of color_eyre
    color_eyre::install()?;

    let max = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u32>().ok())
        .batching(|it| it.map_while(|x| x).sum1::<u32>())
        .map(std::cmp::Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u32>();

    println!("Max {max:?}");
    Ok(())
}
