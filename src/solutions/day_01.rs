use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq)]
pub enum Column {
    Left,
    Right,
}

pub fn parse(list: Vec<String>, col: &Column) -> Vec<i32> {
    list.iter()
        .map(|n| {
            let n = n.split_whitespace().collect::<Vec<&str>>();
            if *col == Column::Left {
                n[0].parse::<i32>().expect("to convert")
            } else {
                n[1].parse::<i32>().expect("to convert")
            }
        })
        .collect::<Vec<i32>>()
}

#[allow(dead_code)]
pub fn solution_part_one() {
    let inputs = File::open("./src/inputs/day_01.txt").expect("input to exist");
    let buf = BufReader::new(inputs);

    let list = buf
        .lines()
        .map(|line| line.expect("Could not parse line from input"))
        .collect::<Vec<String>>();

    let mut l = parse(list.clone(), &Column::Left);
    let mut r = parse(list, &Column::Right);

    if l.len() != r.len() {
        println!("Didn't parse correctly");
        return;
    }

    l.sort();
    r.sort();

    let mut sum = 0;

    for i in 0..l.len() {
        sum += (l[i] - r[i]).abs();
    }

    println!("Answer: {sum}");
}

pub fn solution_part_two() {
    let inputs = File::open("./src/inputs/day_01.txt").expect("input to exist");
    let buf = BufReader::new(inputs);

    let list = buf
        .lines()
        .map(|line| line.expect("Could not parse line from input"))
        .collect::<Vec<String>>();

    let l = parse(list.clone(), &Column::Left);
    let r = parse(list, &Column::Right);

    if l.len() != r.len() {
        println!("Didn't parse correctly");
        return;
    }

    // would ideally use a usize as the value
    // but to avoid casting use a i32 here
    let mut freq = HashMap::<i32, i32>::with_capacity(r.len());
    r.iter().for_each(|n| *freq.entry(*n).or_insert(0) += 1);

    let sum = l.iter().fold(0, |mut sum, n| {
        freq.contains_key(n).then(|| sum += n * freq[&n]);
        sum
    });

    println!("Solution: {sum}");
}
