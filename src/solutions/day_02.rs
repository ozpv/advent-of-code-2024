use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn parse(lines: Vec<String>) -> Vec<Vec<i32>> {
    lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i32>().expect("there to be a number"))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>()
}

#[allow(dead_code)]
pub fn solution_part_one() {
    let file = File::open("./src/inputs/day_02.txt").expect("Inputs to exist");
    let buf = BufReader::new(file);

    let p = parse(
        buf.lines()
            .map(|line| line.expect("Failed to parse line"))
            .collect::<Vec<String>>(),
    );

    let mut safe_count = 0;
    //              last  curr  creas
    let mut last = (None, None, None);
    let mut safe = true;

    for list in p {
        for n in list {
            // insert nums
            if last.0.is_none() {
                last.0 = Some(n);
                continue;
            }
            last.1 = last.0;
            last.0 = Some(n);

            // by now it should be Some
            let (last0, last1) = (last.0.unwrap(), last.1.unwrap());
            if last0.abs_diff(last1) > 3 {
                // not safe
                safe = false;
                break;
            }

            let c = last0.cmp(&last1);

            if last.2.is_none() && c.is_ne() {
                last.2 = Some(c);
            } else if c.is_eq() || Some(c) != last.2 {
                // creasing was not the same or equal
                safe = false;
                break;
            }
        }
        safe.then(|| safe_count += 1);
        // reset
        safe = true;
        last = (None, None, None);
    }
    println!("Safe: {safe_count}");
}

pub fn solution_part_two() {
    let file = File::open("./src/inputs/day_02.txt").expect("Inputs to exist");
    let buf = BufReader::new(file);

    let p = parse(
        buf.lines()
            .map(|line| line.expect("Failed to parse line"))
            .collect::<Vec<String>>(),
    );

    let mut safe_count = 0;
    let mut unsafe_count = 0;
    let mut safe = true;
    let mut cm = None::<Ordering>;

    for list in p {
        for ns in list.windows(3) {
            // previous current next
            // extract the i32 from the slice
            // I know this sucks
            let p = &ns[0..1].iter().sum::<i32>();
            let c = &ns[1..2].iter().sum::<i32>();
            let n = &ns[2..].iter().sum::<i32>();

            // previous current
            let pc = p.cmp(c);
            cm.is_none().then(|| cm = Some(pc));
            // current next
            let cn = c.cmp(n);

            if p.abs_diff(*c) > 3 {
                safe = false;
            }

            if pc != cn || pc.is_eq() || cn.is_eq() {
                if Some(p.cmp(n)) == cm && p.abs_diff(*n) <= 3 {
                    unsafe_count += 1;
                    safe = false;
                } else {
                    safe = false;
                    break;
                }
            }
        }
        // 2 because it will still be in the window
        // for one more iteration
        (safe || unsafe_count == 2).then(|| safe_count += 1);
        // reset
        safe = true;
        unsafe_count = 0;
        cm = None;
    }
    println!("Safe: {safe_count}");
}
