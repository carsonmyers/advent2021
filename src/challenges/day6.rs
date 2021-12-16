use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};

use crate::challenges::Challenge;

pub struct Community {
    ages: HashMap<u8, usize>,
}

impl Community {
    fn new(individuals: Vec<u8>) -> Self {
        let mut age_map = HashMap::new();
        for age in individuals {
            match age_map.get_mut(&age) {
                Some(n) => *n += 1,
                None => { age_map.insert(age, 1); },
            };
        }

        Community { ages: age_map }
    }
}

impl Display for Community {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let max = self.ages.keys().max().unwrap();
        for age in 0..max + 1 {
            write!(f, "\t{}", age).unwrap();
        }
        write!(f, "\n").unwrap();

        for age in 0..max + 1 {
            write!(f, "\t").unwrap();
            match self.ages.get(&age) {
                Some(n) => write!(f, "{}", n).unwrap(),
                None => write!(f, ".").unwrap(),
            }
        }
        write!(f, "\n").unwrap();

        Ok(())
    }
}

impl Iterator for Community {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut update = HashMap::new();
        for (age, amount) in self.ages.iter() {
            match *age {
                0 => {
                    update.insert(6, update.get(&6)
                        .map_or(*amount, |n| n + amount));
                    update.insert(8, update.get(&8)
                        .map_or(*amount, |n| n + amount));
                },
                a @ _ => {
                    update.insert(a - 1, update.get(&(a-1))
                        .map_or(*amount, |n| n + amount));
                },
            };
        }

        self.ages = update;
        Some(self.ages.values().sum())
    }
}

pub struct Day6 {}

impl Day6 {
    pub fn new() -> Self {
        Day6 {}
    }
}

impl Challenge for Day6 {
    fn part_1(&self, input: Vec<String>) -> String {
        let community = Community::new(input.into_iter()
            .flat_map(|line| line.split(",")
                .map(|num| num.parse::<u8>().unwrap())
                .collect::<Vec<u8>>())
            .collect::<Vec<_>>());

        println!("{}", community);
        community.skip(79).next().unwrap().to_string()
    }

    fn part_2(&self, input: Vec<String>) -> String {
        let community = Community::new(input.into_iter()
            .flat_map(|line| line.split(",")
                .map(|num| num.parse::<u8>().unwrap())
                .collect::<Vec<u8>>())
            .collect::<Vec<_>>());

        community.skip(255).next().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::input_from_str;

    const DATA: &'static str = "3,4,3,1,2";

    #[test]
    fn test_part_1() {
        let input = input_from_str(DATA);
        let challenge = Day6::new();
        assert_eq!(challenge.part_1(input), "5934");
    }

    #[test]
    fn test_part_2() {
        let input = input_from_str(DATA);
        let challenge = Day6::new();
        assert_eq!(challenge.part_2(input), "26984457539");
    }
}