use std::fmt::{self, Display, Formatter};

use itertools::Itertools;

use crate::challenges::Challenge;

struct Point {
    x: usize,
    y: usize,
}

impl From<&str> for Point {
    fn from(data: &str) -> Self {
        match data.split(",")
            .map(|num| num.parse::<usize>().unwrap())
            .collect_tuple()
        {
            Some((x, y)) => Point { x, y },
            _ => panic!("invalid point: {}", data),
        }
    }
}

struct Line {
    p1: Point,
    p2: Point,
}

impl From<&str> for Line {
    fn from(data: &str) -> Self {
        match data.split_whitespace().collect_tuple() {
            Some((p1, "->", p2)) => Line {
                p1: Point::from(p1),
                p2: Point::from(p2),
            },
            _ => panic!("invalid line: {}", data),
        }
    }
}

struct Bitmap {
    width: usize,
    data: Vec<u8>,
}

impl Bitmap {
    fn new(width: usize, height: usize) -> Self {
        Bitmap {
            width,
            data: vec![0; width * height],
        }
    }

    fn write_line(&mut self, line: Line) {
        if line.p1.x == line.p2.x {
            self.write_v(line.p1.x, line.p1.y, line.p2.y);
        } else if line.p1.y == line.p2.y {
            self.write_h(line.p1.y, line.p1.x, line.p2.x);
        } else {
            self.write_diag(line);
        }
    }

    fn write_h(&mut self, y: usize, x1: usize, x2: usize) {
        let (x_left, x_right) = if x1 < x2 {
            (x1, x2)
        } else {
            (x2, x1)
        };

        let start = self.width * y + x_left;
        for i in 0..x_right - x_left + 1 {
            let idx = start + i;
            let point = self.data.get_mut(idx).unwrap();
            *point += 1;
        }
    }

    fn write_v(&mut self, x: usize, y1: usize, y2: usize) {
        let (y_top, y_bottom) = if y1 < y2 {
            (y1, y2)
        } else {
            (y2, y1)
        };

        let start = self.width * y_top + x;
        for i in 0..y_bottom - y_top + 1 {
            let idx = start + (self.width * i);
            let point = self.data.get_mut(idx).unwrap();
            *point += 1;
        }
    }

    fn write_diag(&mut self, l: Line) {
        let (left, right) = if l.p1.x < l.p2.x {
            (l.p1, l.p2)
        } else {
            (l.p2, l.p1)
        };

        let direction = if right.y > left.y { 1 } else { -1 };
        let start = (self.width * left.y + left.x) as i32;
        let step = |i: i32| i * self.width as i32 * direction;
        for i in 0..(right.x - left.x + 1) as i32 {
            let idx = start + step(i) + i;
            let point = self.data.get_mut(idx as usize).unwrap();
            *point += 1;
        }
    }
}

impl Display for Bitmap {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.data.iter()
            .enumerate()
            .for_each(|(i, x)| {
                if i % self.width == 0 {
                    write!(f, "\n").unwrap();
                }

                match x {
                    0 => write!(f, ".").unwrap(),
                    _ => write!(f, "{}", x).unwrap(),
                }
            });

        Ok(())
    }
}

pub struct Day5 {}

impl Day5 {
    pub fn new() -> Day5 { Day5 {} }
}

impl Challenge for Day5 {
    fn part_1(&self, input: Vec<String>) -> String {
        let mut bitmap = Bitmap::new(1000, 1000);
        input.into_iter()
            .map(|line| Line::from(line.as_str()))
            .filter(|line| line.p1.x == line.p2.x || line.p1.y == line.p2.y)
            .for_each(|line| bitmap.write_line(line));

        bitmap.data.into_iter()
            .map(|x| if x > 1 { 1 } else { 0 })
            .sum::<i32>()
            .to_string()
    }

    fn part_2(&self, input: Vec<String>) -> String {
        let mut bitmap = Bitmap::new(1000, 1000);
        input.into_iter()
            .map(|line| Line::from(line.as_str()))
            .for_each(|line| bitmap.write_line(line));

        bitmap.data.into_iter()
            .map(|x| if x > 1 { 1 } else { 0 })
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::challenges::input_from_str;

    const DATA: &'static str = r"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    ";

    #[test]
    fn test_part_1() {
        let input = input_from_str(DATA);
        let challenge = Day5::new();
        assert_eq!(challenge.part_1(input), "5");
    }

    #[test]
    fn test_part_2() {
        let input = input_from_str(DATA);
        let challenge = Day5::new();
        assert_eq!(challenge.part_2(input), "12");
    }
}