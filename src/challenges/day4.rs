use std::collections::HashSet;

use crate::challenges::Challenge;

struct Board {
    calls: HashSet<i32>,
    data: Vec<Vec<i32>>,
}

impl Board {
    fn new(data: Vec<Vec<i32>>) -> Self {
        Board {
            calls: HashSet::new(),
            data,
        }
    }

    fn call(&mut self, number: i32) -> bool {
        for (i, row) in self.data.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == number {
                    self.calls.insert(number);
                    return self.check(i, j);
                }
            }
        }

        false
    }

    fn check(&self, i: usize, j: usize) -> bool {
        if self.calls.len() < 5 {
            return false;
        }

        let row_matches = self.data.get(i).unwrap().iter()
            .filter(|num| self.calls.contains(num))
            .count();
        if row_matches == 5 {
            return true;
        }

        let col_matches = self.data.iter()
            .filter(|row| self.calls.contains(row.get(j).unwrap()))
            .count();
        if col_matches == 5 {
            return true;
        }

        false
    }

    fn sum(&self) -> i32 {
        self.data.iter()
            .map(|row| row.iter()
                .filter(|num| !self.calls.contains(*num))
                .sum::<i32>())
            .sum::<i32>()
    }
}

pub struct Day4 {}

impl Day4 {
    pub fn new() -> Self {
        Day4 {}
    }

    fn parse_input(input: Vec<String>) -> (Vec<i32>, Vec<Board>) {
        let mut iterator = input.into_iter();

        let calls = iterator.next().unwrap()
            .split(",")
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        let boards = iterator
            .map(|line| String::from(line.trim()))
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .chunks(5)
            .map(|chunk| Board::new(chunk.into_iter()
                .map(|line| line.split_whitespace()
                    .map(|num| num.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>())
                .collect::<Vec<_>>()))
            .collect::<Vec<_>>();

        (calls, boards)
    }
}

impl Challenge for Day4 {
    fn part_1(&self, input: Vec<String>) -> String {
        let (calls, mut boards) = Day4::parse_input(input);

        for call in calls {
            let results = boards.iter_mut()
                .filter_map(|board| {
                    if board.call(call) {
                        Some(board)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if results.len() > 0 {
                return (results.get(0).unwrap().sum() * call).to_string();
            }
        };

        String::from("no result :(")
    }

    fn part_2(&self, input: Vec<String>) -> String {
        let (calls, mut boards) = Day4::parse_input(input);

        let mut completed = Vec::new();
        for call in calls {
            boards = boards.into_iter()
                .filter_map(|mut board| {
                    if board.call(call) {
                        completed.push((call, board));
                        None
                    } else {
                        Some(board)
                    }
                })
                .collect::<Vec<_>>();
        }

        let (call, last) = completed.last().unwrap();
        (last.sum() * call).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::input_from_str;

    const DATA: &'static str = r"
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
    ";

    #[test]
    fn test_part_1() {
        let input = input_from_str(DATA);
        let challenge = Day4::new();
        assert_eq!(challenge.part_1(input), "4512");
    }

    #[test]
    fn test_part_2() {
        let input = input_from_str(DATA);
        let challenge = Day4::new();
        assert_eq!(challenge.part_2(input), "1924");
    }
}