use nom::{character::complete::{ digit1, space1}, bytes::complete::tag, multi::{many1, separated_list1}, sequence::{preceded, separated_pair, tuple}, IResult, branch::alt};


fn puzzle1(input: String) -> u32{
    let sum: u32 = input.lines().map(|input| {
        let (first, second) = line_parser(input);
        let pts = points(first, second);
        pts
    }).sum();
    sum
}

fn puzzle2(){
    todo!()
}


fn line_parser(input: &str)-> (Vec<&str>, Vec<&str>) {
    let output:IResult<&str,(Vec<&str>, Vec<&str>)> = preceded(
        tuple((tag("Card"), space1, digit1, tag(":"), space1)),
        separated_pair(
            separated_list1(space1, digit1), 
            alt((tag(" |  "), tag(" | "))),
            separated_list1(space1, digit1), 
        )
    )(input);
    let output = output.map(|(nxt_input, output)| {
        (output.0, output.1)
    }).unwrap();
    output
}

fn points(first: Vec<&str>, second: Vec<&str>)-> u32  {
    // 1,2,4,8,16,32,64,126,256,512
    let mut instances = 0;
    let mut unchanged = true;
    for x in first {
        if second.contains(&x) {
            instances += 1;
            unchanged = false;
        }
    }
    if unchanged {
        return 0;
    }
    let i = instances - 1;
    u32::pow(2, i)
}

#[cfg(test)]
mod tests {
    use day1::file_input;

    use super::*;

    #[test]
    fn test_one() {
        let data = "Card  31: 22 11 18 71  7 68 74 36 45 14 | 43 50 37 28 23 93 41 27 21 47  3 25 60  2 79 29 68 11 85 22 77 32 18 12 66";
        let s = line_parser(&data);
        assert!(s == (vec!["22", "11", "18", "71", "7", "68", "74", "36", "45", "14"], vec!["43", "50", "37", "28", "23", "93", "41", "27", "21", "47", "3", "25", "60", "2", "79", "29", "68", "11", "85", "22", "77", "32", "18", "12", "66"]));

        let pts = points(s.0, s.1);
        assert!(pts == 8);
    }

    #[test]
    fn puzzle_test() {
        let blob = file_input("../inputs/day4.txt");
        let sum = puzzle1(blob);
        dbg!(sum);

        assert!(true);
    }

}
