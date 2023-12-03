use std::{iter::Iterator, fs::File, io::{BufReader, Read}, path::Path};

pub fn file_input(path: &str)->String {
    let path = Path::new(path);
    let file = File::open(path).expect("failed to open file");
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    contents
}

/// method 1
pub fn calib_values(input: &str) -> u8 {
    let mut list: Vec<u8> = vec![];
    for x in input.chars() {
        if x.is_ascii_digit(){
            let digit = x.to_string().parse::<u8>().unwrap();
            if list.is_empty()||list.len() == 1 {
                list.push(digit);
            } else if list.len() > 1 {
                list[1] = digit;
            }
        }
    }
    let mut value: u8 = 0;
    if list.len() == 1 {
        value = format!("{}{}",list[0],list[0]).parse::<u8>().unwrap();
    } else {
        value = format!("{}{}",list[0],list[1]).parse::<u8>().unwrap();
    }
    value
}

pub fn total_sum(input: &str) -> u32 {
    let mut list:Vec<u32> = Vec::new();
    for line in input.lines() {
        list.push(calib_values(line).into());
    }
    let sum = list.iter().fold(0, |acc, x| acc + x);
    sum
}
///


pub fn puzzle1(contents: String)->u32{
    // /*Method 1*/total_sum(contents.as_str())
    
    // method 2
    let output = contents.lines().map(|line| {
        let mut it = line.chars().filter_map(|character| {
            character.to_digit(10)
        });
        let first = it.next().expect("failed!!!");
        let last = it.last();

        if let Some(x) = last {
            format!("{}{}",first, x).parse::<u32>().unwrap()
        } else {
            format!("{}{}",first, first).parse::<u32>().unwrap()
        }
    }).sum::<u32>();
    output
}

pub fn puzzle2(contents: String)->u32{
    // total_sum(contents.as_str())
    let output = contents.lines().map(|line| {
        let line = line
            .replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9");
        

        let mut it = line.chars().filter_map(|character| {
            character.to_digit(10)
        });
        let first = it.next().expect("failed!!!");
        let last = it.last();

        if let Some(x) = last {
            format!("{}{}",first, x).parse::<u32>().unwrap()
        } else {
            format!("{}{}",first, first).parse::<u32>().unwrap()
        }
    }).sum::<u32>();
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits() {
        assert!(calib_values("a1b2c3d4e5f") == 15_u8 );
    }

    #[test]
    fn example_blob() {
        let eg = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert!(total_sum(eg) == 142);
    }
    #[test]
    fn example_blob2() {
        let eg = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen".to_string();
        let ans = puzzle2(eg);
        dbg!(&ans);
        assert!(ans == 281);
    }

    
}
