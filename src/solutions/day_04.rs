use std::fs::File;
use std::io::{BufRead, BufReader};
use std::isize;

pub fn parse() -> Vec<Vec<char>> {
    let file = File::open("./src/inputs/day_04.txt").expect("input to exist");
    let buf = BufReader::new(file);

    buf.lines()
        .map(|line| line.expect("line to exist").chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

pub fn solution_part_one() {
    let p = parse();
    // think of this like a graph where X is the origin
    let directions = Vec::<(isize, isize)>::from([
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ]);
    // first char could be anything but I chose X
    let target = Vec::<char>::from(['X', 'M', 'A', 'S']);

    let mut xmas = 0;
    for (cx, line) in p.iter().enumerate() {
        let cx = cx as isize;
        for (cy, ch) in line.iter().enumerate() {
            if *ch != 'X' {
                continue;
            }
            let cy = cy as isize;
            xmas += directions
                .iter()
                // x direction y direction
                .filter(|(x, y)| {
                    // start at 1 because we know were already on an X
                    (1..4).all(|multiplier| {
                        let Some(c) = p
                            .get((cx + (x * multiplier)) as usize)
                            .and_then(|col: &Vec<char>| col.get((cy + (y * multiplier)) as usize))
                        else {
                            return false;
                        };
                        *c == target[multiplier as usize]
                    })
                })
                .count();
        }
    }
    println!("XMAS count: {xmas}");
}

pub fn solution_part_two() {
    let p = parse();

    let mut xmas = 0;
    for (cx, line) in p.iter().enumerate() {
        let cx = cx as isize;
        for (cy, ch) in line.iter().enumerate() {
            if *ch != 'A' {
                continue;
            }
            let cy = cy as isize;
            let diag_1 = {
                let (Some(l), Some(r)) = (
                    p.get((cx + 1) as usize).and_then(|col| col.get((cy + 1) as usize)),
                    p.get((cx - 1) as usize).and_then(|col| col.get((cy - 1) as usize)),
                ) else {
                    continue;
                };
                [l, r]
            };
            let diag_2 = {
                let (Some(l), Some(r)) = (
                    p.get((cx - 1) as usize).and_then(|col| col.get((cy + 1) as usize)),
                    p.get((cx + 1) as usize).and_then(|col| col.get((cy - 1) as usize)),
                ) else {
                    continue;
                };
                [l, r]
            };
            [diag_1, diag_2]
                .iter()
                .all(|diag| match diag[0] {
                    'M' if *diag[1] == 'S' => true,
                    'S' if *diag[1] == 'M' => true,
                    _ => false,
                })
                .then(|| xmas += 1);
        }
    }
    println!("XMAS count: {xmas}");
}
