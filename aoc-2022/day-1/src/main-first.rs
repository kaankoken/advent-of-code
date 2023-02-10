mod util;

use crate::util::read_file;

fn main() -> color_eyre::Result<()> {
    // TODO: Read a file
    // TODO: Find the max calories that elf carries

    /*
      First solution that came into my mind
      - Read file as string
      - Create an vec or array
      - Set start index to 0
      - Until you saw the space or newline add value to certain index
      - After you saw the space or newline increase index by 1
      - Do it until traverse all items
      - Sort the array or vec on decreasing order
      - Get the first element as your answer

      Finding the resolution will be O(1). However, in the worst case scenario,
      Creating array or vec will be O(n) -> space complexity will be O(n) &
      To do the summation needs to traverse the list -> time complexity will be
      O(n) as well.
    */

    // Initialization of color_eyre
    color_eyre::install()?;

    let file_string = read_file(String::from("src/input.txt"))?;

    let mut max = 0;
    // Replace for windows
    for group in file_string.replace("\r\n", "\n").split("\n\n") {
        // println!("Line:");
        let mut sum = 0;
        for line in group.lines() {
            let value = line.parse::<u32>()?;
            //println!("* {value}",);
            sum += value;
        }

        if max < sum {
            max = sum;
        }
    }

    println!("Max: {max}");

    Ok(())
}
