use crate::challenges::Challenge;

pub struct Day1 {}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 {}
    }
}

impl Challenge for Day1 {
    fn part_1<T>(&self, input: Vec<T>) -> String
        where T: Into<String>
    {
        input.into_iter()
            .map(|line| line.into().parse::<i64>().unwrap())
            .collect::<Vec<_>>()
            .windows(2)
            .fold(0, |acc, n| match n {
                [x, y] if y > x => acc + 1,
                _ => acc,
            })
            .to_string()
    }

    fn part_2<T>(&self, input: Vec<T>) -> String
        where T: Into<String>
    {
        input.into_iter()
            .map(|line| line.into().parse::<i64>().unwrap())
            .collect::<Vec<_>>()
            .windows(3)
            .map(|window| window.into_iter().sum())
            .collect::<Vec<i64>>()
            .windows(2)
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