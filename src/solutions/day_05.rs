use core::str;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn parse() -> (HashMap<i32, Vec<i32>>, Vec<Vec<i32>>) {
    let mut inputs = vec![];
    _ = File::open("./src/inputs/day_05.txt")
        .expect("inputs to exist")
        .read_to_end(&mut inputs);
    let (rules, pages) = str::from_utf8(&inputs)
        .expect("file to be valid UTF-8")
        .split_once("\n\n")
        .unwrap();

    let mut map = HashMap::<i32, Vec<i32>>::new();
    rules.lines().for_each(|line| {
        let () = line.split_once('|').map_or((), |(l, r)| {
            let l = l.parse::<i32>().expect("a i32");
            let r = r.parse::<i32>().expect("a i32");
            if let Some(v) = map.get_mut(&l) {
                v.push(r);
            } else {
                map.insert(l, vec![r]);
            }
        });
    });

    let pages = pages
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<i32>().expect("a i32"))
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    (map, pages)
}

pub fn solution_part_one() {
    let (rules, pages) = parse();

    let sum = pages
        .iter()
        .filter(|page| page.is_sorted_by(|a, b| rules[a].contains(b)))
        .map(|page| page[page.len() / 2])
        .sum::<i32>();
    println!("{sum}");
}

pub fn solution_part_two() {
    let (rules, mut pages) = parse();

    let sum = pages
        .iter_mut()
        .filter(|page| !page.is_sorted_by(|a, b| rules[a].contains(b)))
        .map(|page| {
            page.sort_by(|a, b| {
                if rules[a].contains(b) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            page[page.len() / 2]
        })
        .sum::<i32>();
    println!("{sum}");
}
