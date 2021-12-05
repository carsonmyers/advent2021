mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

pub trait Challenge {
    fn part_1(&self, input: Vec<String>) -> String;
    fn part_2(&self, input: Vec<String>) -> String;
}

pub fn get_challenge(day: u8) -> Box<dyn Challenge> {
    match day {
        1 => Box::new(day1::Day1::new()),
        2 => Box::new(day2::Day2::new()),
        3 => Box::new(day3::Day3::new()),
        4 => Box::new(day4::Day4::new()),
        5 => Box::new(day5::Day5::new()),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
fn input_from_str(input: &str) -> Vec<String> {
    input.lines()
        .filter_map(|line| match line.trim() {
            s if s.len() > 0 => Some(String::from(s)),
            _ => None,
        })
        .collect::<Vec<_>>()
}