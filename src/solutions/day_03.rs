use core::str;
use std::{fs::File, io::Read};

pub fn solution_part_one() {
    let mut input = vec![];
    _ = File::open("./src/inputs/day_03.txt")
        .expect("inputs to exist")
        .read_to_end(&mut input);
    let mut input = str::from_utf8(&input).expect("File read to be successful");

    let mut answer = 0;
    let mut i = 0;
    let len = input.len();
    while i < len {
        if input.starts_with("mul(") {
            // skip mul(
            input = &input[4..];
            i += 4;

            let Some((n, _)) = input[..4].split_once(',') else {
                continue;
            };
            let l = n.parse::<i32>().expect("a number");

            // len of l + 1 because of the comma
            let remainder = &input[n.len() + 1..];
            let Some((remainder, _)) = ({
                // 5 because len is inclusive
                if input.len() - 5 > 0 {
                    remainder[..4].split_once(')')
                } else {
                    remainder.split_once(')')
                }
            }) else {
                continue;
            };
            let r = remainder.parse::<i32>().expect("a number");
            answer += l * r;

            // now we can skip ahead the len of the numbers we just
            // parsed -2 again because len is inclusive
            input = &input[remainder.len() + n.len() - 2..];
            i += remainder.len() + n.len() - 2;
        } else {
            input = &input[1..];
            i += 1;
        }
    }
    println!("Answer: {answer}");
}

pub fn solution_part_two() {
    let mut input = vec![];
    _ = File::open("./src/inputs/day_03.txt")
        .expect("inputs to exist")
        .read_to_end(&mut input);
    let mut input = str::from_utf8(&input).expect("File read to be successful");

    let mut answer = 0;
    let mut i = 0;
    let len = input.len();
    let mut do_mul = true;
    while i < len {
        if input.starts_with("mul(") && do_mul {
            // skip mul(
            input = &input[4..];
            i += 4;

            let Some((n, _)) = input[..4].split_once(',') else {
                continue;
            };
            let l = n.parse::<i32>().expect("a number");

            // len of l + 1 because of the comma
            let remainder = &input[n.len() + 1..];
            let Some((remainder, _)) = ({
                // 5 because len is inclusive
                if input.len() - 5 > 0 {
                    remainder[..4].split_once(')')
                } else {
                    remainder.split_once(')')
                }
            }) else {
                continue;
            };
            let r = remainder.parse::<i32>().expect("a number");
            answer += l * r;

            // now we can skip ahead the len of the numbers we just
            // parsed -2 again because len is inclusive
            input = &input[remainder.len() + n.len() - 2..];
            i += remainder.len() + n.len() - 2;
        } else if input.starts_with("do()") {
            // slice and increment for the len of the starts with
            do_mul = true;
            input = &input[4..];
            i += 4;
        } else if input.starts_with("don't()") {
            do_mul = false;
            input = &input[7..];
            i += 7;
        } else {
            input = &input[1..];
            i += 1;
        }
    }
    println!("Answer: {answer}");
}
