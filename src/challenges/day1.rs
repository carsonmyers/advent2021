use crate::challenges::Challenge;

pub struct Day1 {}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 {}
    }
}

impl Challenge for Day1 {
    fn part_1(&self, input: Vec<String>) -> String {
        input.into_iter()

            // Each line of the input is an integer
            .map(|line| line.parse::<i64>().unwrap())

            // Iterate over every pair of sequential numbers
            .collect::<Vec<_>>()
            .windows(2)

            // Count every time an entry is larger than the previous
            .fold(0, |acc, n| match n {
                [x, y] if y > x => acc + 1,
                _ => acc,
            })
            .to_string()
    }

    fn part_2(&self, input: Vec<String>) -> String {
        input.into_iter()

            // Parse every entry into an integer
            .map(|line| line.parse::<i64>().unwrap())

            // Iterate over every group of 3 sequential numbers to create a
            // three measurement sliding window.
            .collect::<Vec<_>>()
            .windows(3)

            // Collapse each window into its sum
            .map(|window| window.into_iter().sum())

            // Iterate over every pair of sums from the previous steps
            .collect::<Vec<i64>>()
            .windows(2)

            // Count every time a sum has increased
            .fold(0, |acc, n| match n {
                [x, y] if y > x => acc + 1,
                _ => acc,
            })
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::input_from_str;

    // Example data from the challenge description
    const DATA: &'static str = r"
        199
        200
        208
        210
        200
        207
        240
        269
        260
        263
    ";

    #[test]
    fn test_part_1() {
        let input = input_from_str(DATA);
        let challenge = Day1::new();
        assert_eq!(challenge.part_1(input), "7");
    }

    #[test]
    fn test_part_2() {
        let input = input_from_str(DATA);
        let challenge = Day1::new();
        assert_eq!(challenge.part_2(input), "5");
    }
}