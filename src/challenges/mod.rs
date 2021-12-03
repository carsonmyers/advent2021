mod day1;
mod day2;

pub trait Challenge {
    fn part_1(&self, input: Vec<String>) -> String;
    fn part_2(&self, input: Vec<String>) -> String;
}

pub fn get_challenge(day: u8) -> Box<dyn Challenge> {
    match day {
        1 => Box::new(day1::Day1::new()),
        2 => Box::new(day2::Day2::new()),
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