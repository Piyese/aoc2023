use std::iter::from_fn;

use day1::{puzzle1, puzzle2, file_input};


fn main() {
    let c = file_input("inputs/day1-1.txt");
    let ans1 = puzzle1(c);
    // let ans2 = puzzle2();
    dbg!(ans1);
    // dbg!(ans2);
    // let mut a = "fiveseventhree2llxsgzeight".to_string();
    // let mut index = 0;
    // let line_iter = from_fn( move||{
    //     let cut_line = &a[index..];
    //     let result = if cut_line.starts_with("one") {
    //         Some('1')
    //     } else if cut_line.starts_with("two") {
    //         Some('2')
    //     } else if cut_line.starts_with("three") {
    //         Some('3')
    //     }else if cut_line.starts_with("four") {
    //         Some('4')
    //     }else if cut_line.starts_with("five") {
    //         Some('5')
    //     }else if cut_line.starts_with("six") {
    //         Some('6')
    //     }else if cut_line.starts_with("seven") {
    //         Some('7')
    //     }else if cut_line.starts_with("eight") {
    //         Some('8')
    //     }else if cut_line.starts_with("nine") {
    //         Some('9')
    //     } else {
    //         None
    //     };
    //     dbg!(&a);
    //     dbg!(&result);
    //     dbg!(&index);
    //     index+=1;
    //     result
    // });
    // let mut it = line_iter.filter_map(|character| {
    //     character.to_digit(10)
    // });
    // let first = it.next().expect("failed!!!");
    // let last = it.last();

    // let r = if let Some(x) = last {
    //     format!("{}{}",first, x).parse::<u32>().unwrap()
    // } else {
    //     format!("{}{}",first, first).parse::<u32>().unwrap()
    // };
}