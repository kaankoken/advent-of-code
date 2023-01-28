#[derive(Debug)]
struct PathIoError {
    path: String,
    inner: std::io::Error,
}

fn read_file() -> Result<String, PathIoError> {
    let path = "src/input.txt";

    match std::fs::read_to_string(path) {
        Ok(s) => Ok(s),
        Err(e) => Err(PathIoError {
            path: path.into(),
            inner: e,
        }),
    }
}

fn main() {
    // File format
    /*
      1000 \
      2000 --> elf 1
      3000 /

      4000 --> elf 2

      5000 --> elf 3
      6000

      7000 \
      8000 --> elf 4
      9000 /

      10000 --> elf 5

      .
      . --> elf n
    */
    // Each space defines an elf

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

    let x = read_file().unwrap();
}
