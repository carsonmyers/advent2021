use std::collections::HashMap;
use crate::challenges::Challenge;

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
            match item.chars().skip(sort_index).next() {
                Some('0') => result.left.push(item),
                Some('1') => result.right.push(item),
                _ => panic!("invalid binary number: {}", item),
            }
        }

        result
    }

    fn left(self) -> LeftRight {
        LeftRight::new(self.left, self.sort_index + 1)
    }

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
        let mut o2_lr = LeftRight::new(input.clone(), 0);
        let mut co2_lr = LeftRight::new(input, 0);
        let mut o2_rating: Option<String> = None;
        let mut co2_rating: Option<String> = None;

        while o2_rating.is_none() {
            match (o2_lr.left.len(), o2_lr.right.len()) {
                (1, 0) => o2_rating = o2_lr.left.pop(),
                (0, 1) => o2_rating = o2_lr.right.pop(),
                (1, 1) => o2_rating = o2_lr.right.pop(),
                (l, r) if r >= l => o2_lr = o2_lr.right(),
                _ => o2_lr = o2_lr.left(),
            }
        }

        while co2_rating.is_none() {
            match (co2_lr.left.len(), co2_lr.right.len()) {
                (1, 0) => co2_rating = co2_lr.left.pop(),
                (0, 1) => co2_rating = co2_lr.right.pop(),
                (1, 1) => co2_rating = co2_lr.left.pop(),
                (l, r) if l > r => co2_lr = co2_lr.right(),
                _ => co2_lr = co2_lr.left(),
            }
        }

        vec![o2_rating, co2_rating].into_iter()
            .map(|num_str| i64::from_str_radix(&num_str.unwrap(), 2).unwrap())
            .product::<i64>()
            .to_string()
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
        let input = input_from_str(DATA);
        let challenge = Day3::new();
        assert_eq!(challenge.part_2(input), "230");
    }
}