use std::io::*;
use std::fs::File;

fn main() {
    let file = File::open("./input/day2");
    let reader = BufReader::new(file.unwrap());

    //let lines: Vec<Vec<_>> = reader.lines().map(|x| x.unwrap().split(' ').collect::<Vec<_>>()).collect();
    
    let mut pos = 0;
    let mut depth1 = 0;
    let mut depth2 = 0;
    let mut aim = 0;

    for line in reader.lines() {
        let current = line.unwrap();
        let instruction = current.split(' ').collect::<Vec<_>>();
        let argument = instruction[1].parse::<i32>().unwrap();
        match instruction[0] {
            "forward" => {
                pos += argument;
                depth2 += aim * argument;
            },
            "down" => {
                depth1 += argument;
                aim += argument;
            },
            "up" => {
                depth1 -= argument;
                aim -= argument;
            },
            _=> println!("unknown instruction")
        }
    }
    println!("part1: {}\npart2: {}", pos * depth1, pos * depth2);
}
