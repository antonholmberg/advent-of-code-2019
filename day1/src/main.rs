mod module;

use module::Module;
use std::env::args;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn fuel_from_file<T>(filename: &str, func: T) -> i32
where
    T: FnMut(i32, i32) -> i32,
{
    return read_lines(filename)
        .expect("Could not open file")
        .collect::<io::Result<Vec<String>>>()
        .expect("Could not read lines")
        .iter()
        .map(|line| line.parse::<i32>().unwrap())
        .fold(0, func);
}

fn first_exercise(filename: &str) -> i32 {
    fuel_from_file(filename, |acc, num| acc + num.fuel_consumption())
}

fn second_exercise(filename: &str) -> i32 {
    fuel_from_file(filename, |acc, num| {
        acc + num.fuel_consumption_including_fuel()
    })
}

fn main() -> io::Result<()> {
    let filename = args().nth(1).expect("Missing filename");

    let total_fuel = first_exercise(&filename);
    let total_fuel_including_fuel = second_exercise(&filename);

    println!("Fuel consumption was: {}", total_fuel);
    println!(
        "Fuel consumption including fuel was: {}",
        total_fuel_including_fuel
    );

    Ok(())
}
