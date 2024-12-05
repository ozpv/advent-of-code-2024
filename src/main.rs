use advent_of_code_2024::solutions::*;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let begin = Instant::now();

    {
        day_05::solution_part_two();
    }

    let end = begin.elapsed();

    println!("{end:.2?}");

    Ok(())
}
