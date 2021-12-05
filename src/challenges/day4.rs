use std::collections::HashSet;

use crate::challenges::Challenge;

/// Bingo board that keeps track of all the matching numbers called
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

    /// Call a number in a bingo game.
    ///
    /// Returns whether bingo has been achieved
    fn call(&mut self, number: i32) -> bool {
        for (i, row) in self.data.iter().enumerate() {
            for (j, col) in row.iter().enumerate() {
                if *col == number {
                    // Dab the number on the card
                    self.calls.insert(number);

                    // Check for bingo for this row and column
                    return self.check(i, j);
                }
            }
        }

        false
    }

    /// Returns whether bingo has been achieved on a particular row/column
    fn check(&self, i: usize, j: usize) -> bool {
        // You can't get bingo with fewer than 5 dabs
        if self.calls.len() < 5 {
            return false;
        }

        // Count the number of dabs on the specified row - 5 dabs is bingo
        let row_matches = self.data.get(i).unwrap().iter()
            .filter(|num| self.calls.contains(num))
            .count();
        if row_matches == 5 {
            return true;
        }

        // Count the number of dabs on the specified column - 5 wins
        let col_matches = self.data.iter()
            .filter(|row| self.calls.contains(row.get(j).unwrap()))
            .count();
        if col_matches == 5 {
            return true;
        }

        false
    }

    /// Sum all the un-dabbed numbers on the board
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

    /// Parse the input data to create a callout sequence and several
    /// bingo boards. The input starts with a comma-delimited list of call
    /// numbers, then a series of space-separated matrices representing bingo
    /// cards. Each item is separated by a blank line:
    ///
    /// ```
    /// 1,23,44,12,17,0,9,...
    ///
    ///  1 23  0 15 21
    /// 11 54 47  6  9
    /// ...
    /// ```
    ///
    /// Returns a tuple containing the callout numbers and the vector of boards
    fn parse_input(input: Vec<String>) -> (Vec<i32>, Vec<Board>) {
        let mut iterator = input.into_iter();

        // The first line of the input contains the call numbers
        let calls = iterator.next().unwrap()

            // Numbers are comma separated
            .split(",")

            // Parse each number
            .map(|num| num.parse::<i32>().unwrap())
            .collect::<Vec<_>>();

        // All the remaining data is bingo cards
        let boards = iterator

            // Some lines begin with a space for alignment. Be sure to trim
            // all extraneous whitespace from each line
            .map(|line| String::from(line.trim()))

            // Skip blank lines - we know all the remaining data is groups of 5
            // lines of 5 numbers each
            .filter(|line| !line.is_empty())

            // Iterate over 5-line chunks of the input to parse the board data
            .collect::<Vec<_>>()
            .chunks(5)

            // Construct a board from each chunk
            .map(|chunk| Board::new(chunk.into_iter()

                // Bingo board numbers are whitespace separated
                .map(|line| line.split_whitespace()
                    .map(|num| num.parse::<i32>().unwrap())

                    // Collect cells into a row of numbers
                    .collect::<Vec<i32>>())

                // Collect rows into a board
                .collect::<Vec<_>>()))

            // Collect boards into a vector
            .collect::<Vec<_>>();

        (calls, boards)
    }
}

impl Challenge for Day4 {
    fn part_1(&self, input: Vec<String>) -> String {
        let (calls, mut boards) = Day4::parse_input(input);

        // Repeat the check for any winning boards for each call number
        // until a winner is found
        for call in calls {
            let results = boards.iter_mut()

                // Filter out all non-winning boards
                .filter_map(|board| {
                    if board.call(call) {
                        Some(board)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            // End the game as soon as a winner is found
            if results.len() > 0 {
                return (results.get(0).unwrap().sum() * call).to_string();
            }
        };

        panic!("no winner!");
    }

    fn part_2(&self, input: Vec<String>) -> String {
        let (calls, mut boards) = Day4::parse_input(input);

        // Collect winning boards in the order that they won, along with
        // the winning number that was called
        let mut completed = Vec::new();

        for call in calls {
            // Replace the set of boards on each iteration, since they will
            // continue to spuriously win if we keep calling numbers on them
            boards = boards.into_iter()

                // Filter out winning boards so they're not re-checked
                .filter_map(|mut board| {
                    if board.call(call) {
                        // Move winners to the completed pile with the winning
                        // number
                        completed.push((call, board));
                        None
                    } else {
                        // Otherwise return them to the candidate pile
                        Some(board)
                    }
                })
                .collect::<Vec<_>>();
        }

        // Get the last winning card and score it
        let (call, last) = completed.last().unwrap();
        (last.sum() * call).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::input_from_str;

    // Example data from the challenge description
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