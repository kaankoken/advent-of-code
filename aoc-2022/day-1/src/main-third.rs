fn main() -> color_eyre::Result<()> {
    // Initialization of color_eyre
    color_eyre::install()?;

    let lines = include_str!("input.txt").lines().collect::<Vec<_>>();
    let groups = lines.split(|line| line.is_empty()).collect::<Vec<_>>();

    let groups = groups
        .into_iter()
        .map(|group| group.iter().map(|v| v.parse::<u32>().unwrap()).sum::<u32>())
        .max();

    println!("Max {groups:?}");
    Ok(())
}
