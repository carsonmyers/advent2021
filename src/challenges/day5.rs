use crate::challenges::Challenge;

pub struct Day5 {}

impl Day5 {
    pub fn new() -> Day5 { Day5 {} }
}

impl Challenge for Day5 {
    fn part_1(&self, _input: Vec<String>) -> String {
        unimplemented!();
    }

    fn part_2(&self, _input: Vec<String>) -> String {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::input_from_str;

    const DATA: &'static str = r"";

    #[test]
    fn test_part_1() {
        let input = input_from_str(DATA);
        let challenge = Day5::new();
        assert_eq!(challenge.part_1(input), "");
    }

    #[test]
    fn test_part_2() {
        let input = input_from_str(DATA);
        let challenge = Day5::new();
        assert_eq!(challenge.part_2(input), "");
    }
}