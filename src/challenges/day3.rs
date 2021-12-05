use std::collections::HashMap;
use crate::challenges::Challenge;

/// Simple sorting data structure - 0's go on the left and 1's go on the right,
/// with the digit used to partition the input being specified by `sort_index`
struct LeftRight {
    sort_index: usize,
    left: Vec<String>,
    right: Vec<String>,
}

impl LeftRight {
    fn new(input: Vec<String>, sort_index: usize) -> LeftRight {
        let mut result = LeftRight {
            sort_index,
            left: Vec::new(),
            right: Vec::new(),
        };

        for item in input {
            // The digits of the input are numbered from left to right,
            // somewhat counterintuitively.
            match item.chars().skip(sort_index).next() {
                // Zeroes go in the left bucket
                Some('0') => result.left.push(item),

                // Ones go in the right bucket
                Some('1') => result.right.push(item),

                // There's no such thing as 2
                _ => panic!("invalid binary number: {}", item),
            }
        }

        result
    }

    /// Convenience method to discard the 1's and re-split the 0's on the
    /// next sort index
    fn left(self) -> LeftRight {
        LeftRight::new(self.left, self.sort_index + 1)
    }

    /// Convenience method to discard the 0's and re-split the 1's on the
    /// next sort index
    fn right(self) -> LeftRight {
        LeftRight::new(self.right, self.sort_index + 1)
    }
}

pub struct Day3 {}

impl Day3 {
    pub fn new() -> Self { Day3 {} }
}

impl Challenge for Day3 {
    fn part_1(&self, input: Vec<String>) -> String {
        // Count the number of 1's and 0's in each column by adding 1 for a
        // 1, and subtracting 1 for a zero
        let balance = input.into_iter()

            // Fold all the lines into a hashmap mapping the digit position to
            // the 1's and 0's balance
            .fold(HashMap::new(), |mut acc: HashMap<usize, i32>, line| {
                line.chars()

                    // Map each character of the line to a balancing value
                    .map(|c| match c {
                        '0' => -1,
                        '1' => 1,
                        _ => panic!("invalid binary"),
                    })

                    // Reverse the direction of the numbers so that they're
                    // indexed from right-to-left, which makes it easier to
                    // convert the binary string into an integer later
                    .rev()

                    // Collect the balance values in the accumulator
                    .enumerate()
                    .for_each(|(i, n)| match acc.get_mut(&i) {
                        Some(count) => *count += n,
                        None => { acc.insert(i, n); },
                    });
                acc
            });

        let gamma_rate = balance.iter()

            // Map the + or - balance values into 1's and 0's respectively
            .map(|(i, n)| match *n {
                n if n > 0 => (i, 1),
                _ => (i, 0),
            })

            // Fold the digits from the last step into an integer with bit math
            .fold(0, |acc, (i, n)| acc | n << i);

        let epsilon_rate = balance.iter()

            // Map the + or - balance values into 0's and 1's respectively
            .map(|(i, n)| match *n {
                n if n < 0 => (i, 1),
                _ => (i, 0),
            })

            // Fold the digits from the last step into an integer with bit math
            .fold(0, |acc, (i, n)| acc | n << i);

        (gamma_rate * epsilon_rate).to_string()
    }

    fn part_2(&self, input: Vec<String>) -> String {
        // Create a left-right struct for the o2 generator and co2 scruber data
        let mut o2_lr = LeftRight::new(input.clone(), 0);
        let mut co2_lr = LeftRight::new(input, 0);

        // The ratings for o2 and co2 data will be populated once a result is
        // located
        let mut o2_rating: Option<String> = None;
        let mut co2_rating: Option<String> = None;

        while o2_rating.is_none() {
            match (o2_lr.left.len(), o2_lr.right.len()) {

                // Base case: only one number is left (or one on each side)
                // so the search is over and the rating can be populated
                (1, 0) => o2_rating = o2_lr.left.pop(),
                (0, 1) => o2_rating = o2_lr.right.pop(),
                (1, 1) => o2_rating = o2_lr.right.pop(),

                // Split the most popular lr bucket, slightly favouring 1's
                (l, r) if r >= l => o2_lr = o2_lr.right(),
                _ => o2_lr = o2_lr.left(),
            }
        }

        while co2_rating.is_none() {
            match (co2_lr.left.len(), co2_lr.right.len()) {

                // Base case: only one number is left (or one on each side)
                // so the search is over and the rating can be populated
                (1, 0) => co2_rating = co2_lr.left.pop(),
                (0, 1) => co2_rating = co2_lr.right.pop(),
                (1, 1) => co2_rating = co2_lr.left.pop(),

                // Split the least popular lr bucket, slightly favouring 0's
                (l, r) if l > r => co2_lr = co2_lr.right(),
                _ => co2_lr = co2_lr.left(),
            }
        }

        vec![o2_rating, co2_rating].into_iter()

            // Parse the binary string into an integer
            .map(|num_str| i64::from_str_radix(&num_str.unwrap(), 2).unwrap())

            // The answer to the puzzle is the product of the two ratings
            .product::<i64>()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::challenges::input_from_str;

    // Example data from the challenge description
    const DATA: &'static str = r"
        00100
        11110
        10110
        10111
        10101
        01111
        00111
        11100
        10000
        11001
        00010
        01010
    ";

    #[test]
    fn test_part_1() {
        let input = input_from_str(DATA);
        let challenge = Day3::new();
        assert_eq!(challenge.part_1(input), "198");
    }

    #[test]
    fn test_part_2() {
        let input = input_from_str(DATA);
        let challenge = Day3::new();
        assert_eq!(challenge.part_2(input), "230");
    }
}