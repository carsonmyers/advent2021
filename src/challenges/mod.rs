mod day1;
mod day2;

pub trait Challenge {
    fn part_1<T>(&self, input: Vec<T>) -> String
        where T: Into<String>;
    fn part_2<T>(&self, input: Vec<T>) -> String
        where T: Into<String>;
}

pub fn get_challenge(day: u8) -> impl Challenge {
    match day {
        1 => day1::Day1::new(),
        2 => day2::Day2::new(),
        _ => unimplemented!(),
    }
}

pub fn input_from_str(input: &str) -> Vec<&str> {
    input.lines()
        .filter_map(|line| match line.trim() {
            s if s.len() > 0 => Some(s),
            _ => None,
        })
        .collect::<Vec<_>>()
}