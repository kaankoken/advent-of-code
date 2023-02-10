use itertools::{FoldWhile, Itertools};

fn main() -> color_eyre::Result<()> {
    // Initialization of color_eyre
    // Initialization of color_eyre
    color_eyre::install()?;

    let max = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u32>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(value)) = it.next() {
                sum = Some(sum.map_or(value, |sum| sum + value));
            }
            sum

            // Alternative
            /* it.fold_while(None, |acc: Option<u32>, v| match v {
                Some(v) => FoldWhile::Continue(Some(acc.unwrap_or_default() + v)),
                None => FoldWhile::Done(acc),
            })
            .into_inner()*/
        })
        .map(std::cmp::Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u32>();

    println!("Max {max:?}");
    Ok(())
}
