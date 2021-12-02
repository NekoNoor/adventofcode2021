use std::io::*;
use std::fs::File;

fn count_increment(input: &Vec<i32>) -> usize {
    return input.windows(2).filter(|t| t[0] < t[1]).count();
}

fn main() {
    let file = File::open("./input/day1");
    let reader = BufReader::new(file.unwrap());

    let vector: Vec<i32> = reader.lines().map(|x| x.unwrap().parse().unwrap()).collect();
    let sum_vector: Vec<i32> = vector.windows(3).map(|x| x.iter().sum::<i32>()).collect();

    println!("part1: {}\npart2: {}", count_increment(&vector), count_increment(&sum_vector));
}
