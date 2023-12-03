use crate::parser::parser1;

mod parser;

#[derive(Debug, PartialEq)]
pub struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}
impl Bag {
    pub fn new(red: u32, green: u32, blue :u32 )->Self {
        Self{ red, green, blue}
    }
    pub fn update(&mut self, data: (u32, &str)) {
        match data.1 {
            "red" => {
                if self.red < data.0 {
                    self.red = data.0;
                }
            },
            "green" => {
                if self.green < data.0 {
                    self.green = data.0;
                }
            },
            "blue" => {
                if self.blue < data.0 {
                    self.blue = data.0;
                }
            },
            
            _ => unreachable!()
        }
    }
    pub fn can_fit(&self, other: &Bag) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }
    pub fn product(&self) -> u32 {
        self.red*self.blue*self.green
    }
}

pub fn puzzle1(input: String) -> u32 {
    let r: usize = input.lines().enumerate().map(|(i, mut line)| {
        let play_bag = Bag::new(12, 13, 14);
        let mut new_bag = Bag::new(0,0,0);

        if i >= 99 {
            line = &line[9..];
        }else if i >= 9 {
            line = &line[8..];
        } else {
            line = &line[7..];
        }
        // dbg!(&line);

        let output = parser1(line).unwrap().1;
        for tup in output {
            new_bag.update(tup);
        }
        if play_bag.can_fit(&new_bag) {
            i + 1
        }else {
            0
        }
    }).sum();
    // println!("{r:?}");
    r as u32
}

pub fn puzzle2(input: String) -> u32 {
    let r: u32 = input.lines().enumerate().map(|(i, mut line)| {
        let mut new_bag = Bag::new(0,0,0);

        if i >= 99 {
            line = &line[9..];
        }else if i >= 9 {
            line = &line[8..];
        } else {
            line = &line[7..];
        }
        let output = parser1(line).unwrap().1;

        for tup in output {
            new_bag.update(tup);
        }

        new_bag.product()
    }).sum();
    r
}

#[cfg(test)]
mod tests {
    use day1::file_input;

    use super::*;

    #[test]
    fn it_fits() {
        let bag1 = Bag::new(12, 13, 14);
        let mut bag2 = Bag::new(9, 8, 2);
        assert!(&bag1.can_fit(&bag2));

        bag2.update((13, "green"));
        assert!(&bag1.can_fit(&bag2));
    }

    #[test]
    fn lets_do_one() {
        let input = file_input("../inputs/day2.txt");
        let ans = puzzle1(input);
        dbg!(ans);

        assert!(0==0);
    }

    #[test]
    fn lets_do_two() {
        let input = file_input("../inputs/day2.txt");
        let ans = puzzle2(input);
        dbg!(ans);

        assert!(0==0);
    }
}
