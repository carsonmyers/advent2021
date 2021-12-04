use std::collections::HashMap;
use crate::challenges::Challenge;

pub struct Day3 {}

impl Day3 {
    pub fn new() -> Self { Day3 {} }
}

impl Challenge for Day3 {
    fn part_1(&self, input: Vec<String>) -> String {
        let balance = input.into_iter()
            .fold(HashMap::new(), |mut acc: HashMap<usize, i32>, line| {
                line.chars()
                    .map(|c| match c {
                        '0' => -1,
                        '1' => 1,
                        _ => panic!("invalid binary"),
                    })
                    .rev()
                    .enumerate()
                    .for_each(|(i, n)| match acc.get_mut(&i) {
                        Some(count) => *count += n,
                        None => { acc.insert(i, n); },
                    });
                acc
            });

        let gamma_rate = balance.iter()
            .map(|(i, n)| match *n {
                n if n > 0 => (i, 1),
                _ => (i, 0),
            })
            .fold(0, |acc, (i, n)| acc | n << i);

        let epsilon_rate = balance.iter()
            .map(|(i, n)| match *n {
                n if n < 0 => (i, 1),
                _ => (i, 0),
            })
            .fold(0, |acc, (i, n)| acc | n << i);

        (gamma_rate * epsilon_rate).to_string()
    }

    fn part_2(&self, input: Vec<String>) -> String {
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::challenges::input_from_str;

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

    }
}