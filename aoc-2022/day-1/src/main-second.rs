fn main() -> color_eyre::Result<()> {
    // Initialization of color_eyre
    color_eyre::install()?;

    let mut max = 0;

    for group in include_str!("input.txt")
        .replace("\r\n", "\n")
        .split("\n\n")
    {
        let mut sum = 0;
        for line in group.lines() {
            let value = line.parse::<u32>()?;
            sum += value;
        }

        if max < sum {
            max = sum;
        }
    }

    println!("Max {max}");
    Ok(())
}
