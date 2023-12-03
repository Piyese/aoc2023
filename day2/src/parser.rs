use nom::{self, IResult, bytes::complete::tag, character::complete::{digit1, alpha1}, sequence::{separated_pair, preceded}, multi::separated_list1, branch::alt};


// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
pub fn parser1(input: &str) -> IResult<&str, Vec<(u32, &str)>> {
    let output = separated_list1(
        alt((tag(";"), tag(","))), 
        preceded(tag(" "), separated_pair(digit1, tag(" "), alpha1)),
    )(input).map(|(nxt, tup)| {
        let it = tup.into_iter().map(|(number, colour)| {
            let no = number.parse::<u32>().expect("failed to parse");
            (no, colour)
        }).collect();
        (nxt, it)
    });
    output
}

#[test]
fn parser_test() {
    let s = " 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
    assert!(parser1(s) == Result::Ok(("", vec![(1, "blue"), (2, "green"), (3, "green"), (4, "blue"), (1, "red"), (1, "green"), (1, 
        "blue")])));
}