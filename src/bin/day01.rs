use aoc_lib::read_lines_byte;

fn main() {
    println!(
        "{}",
        read_lines_byte("./input/day01.txt")
            .unwrap()
            .iter()
            .map(|line| {
                let test: Vec<char> = line
                    .iter()
                    .map(|char| char::from_u32(*char as u32).unwrap())
                    .filter(|char| char.is_numeric())
                    .collect();
                String::from(
                    test.first().unwrap().to_string() + test.last().unwrap().to_string().as_ref(),
                )
                .parse::<u32>()
                .unwrap()
            })
            .sum::<u32>()
    );
}
