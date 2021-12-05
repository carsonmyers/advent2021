use std::convert::From;
use crate::challenges::Challenge;

/// Submarine travelling direction (submarines cannot travel
/// backwards... apparently)
enum Direction {
    Up(u32),
    Down(u32),
    Forward(u32),
}

impl From<String> for Direction {
    fn from(text: String) -> Self {
        // Submarine instructions come in the form "forward 10", with an
        // instruction and distance separated by a space
        let mut parts = text.split(" ");

        // Match the instruction and the result of parsing the second
        // part from the split to create a `Direction`, panicking on anything
        // that doesn't match
        match (parts.next(), parts.next().and_then(|num| num.parse::<u32>().ok())) {
            (Some("up"), Some(n @ _)) => Direction::Up(n),
            (Some("down"), Some(n @ _)) => Direction::Down(n),
            (Some("forward"), Some(n @ _)) => Direction::Forward(n),
            _ => panic!("malformed direction: {}", text),
        }
    }
}

/// Position of a submarine both laterally through the water, and its depth
struct Position {
    horizontal: i32,
    depth: i32,
}

impl Position {
    fn new() -> Self {
        Position {
            horizontal: 0,
            depth: 0,
        }
    }

    /// Use a direction instruction to modify the submarine's position
    fn go(&mut self, direction: Direction) {
        match direction {
            Direction::Up(distance) => self.depth -= distance as i32,
            Direction::Down(distance) => self.depth += distance as i32,
            Direction::Forward(distance) => self.horizontal += distance as i32,
        }
    }
}

/// The position and aim of the submarine
struct Attitude {
    aim: i32,
    position: Position,
}

impl Attitude {
    fn new() -> Self {
        Attitude {
            aim: 0,
            position: Position::new(),
        }
    }

    /// Use a direction instruction to modify the submarine's position -
    /// this is different to `Position::go` in that the submarine's depth
    /// changes depending on its aim
    fn go(&mut self, direction: Direction) {
        match direction {
            Direction::Up(amount) => self.aim -= amount as i32,
            Direction::Down(amount) => self.aim += amount as i32,
            Direction::Forward(distance) => {
                self.position.horizontal += distance as i32;
                self.position.depth += self.aim * distance as i32;
            }
        }
    }
}

pub struct Day2 {}

impl Day2 {
    pub fn new() -> Day2 {
        Day2 {}
    }
}

impl Challenge for Day2 {
    fn part_1(&self, input: Vec<String>) -> String {
        let final_position = input.into_iter()

            // Parse each direction instruction from the input
            .map(|line| Direction::from(line))

            // Execute each instruction to calculate the submarine's final
            // position in the water
            .fold(Position::new(), |mut acc, x| {
                acc.go(x);
                acc
            });

        (final_position.horizontal * final_position.depth).to_string()
    }

    fn part_2(&self, input: Vec<String>) -> String {
        let final_att = input.into_iter()

            // Parse each direction instruction from the input
            .map(|line| Direction::from(line))

            // Execute each instruction using the attitude method to calculate
            // the submarine's final position in the water
            .fold(Attitude::new(), |mut acc, x| {
                acc.go(x);
                acc
            });

        (final_att.position.horizontal * final_att.position.depth).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::input_from_str;

    // Example data from the challenge description
    const DATA: &'static str = r"
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    ";

    #[test]
    fn test_part_1() {
        let input = input_from_str(DATA);
        let challenge = super::Day2::new();
        assert_eq!(challenge.part_1(input), "150");
    }

    #[test]
    fn test_part_2() {
        let input = input_from_str(DATA);
        let challenge = super::Day2::new();
        assert_eq!(challenge.part_2(input), "900");
    }
}