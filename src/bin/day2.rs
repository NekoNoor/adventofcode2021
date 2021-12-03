use aoc::*;

fn main() {
    let lines = read_input("input/day2");
    //let lines: Vec<Vec<_>> = reader.lines().map(|x| x.unwrap().split(' ').collect::<Vec<_>>()).collect();
    
    let mut pos = 0;
    let mut depth1 = 0;
    let mut depth2 = 0;
    let mut aim = 0;

    for line in lines {
        let instruction = line.split(' ').collect::<Vec<_>>();
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
