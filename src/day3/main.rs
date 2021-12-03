use std::fs::File;
use std::io::*;

fn main() {
    let file = File::open("./input/day3");
    let reader = BufReader::new(file.unwrap());

    let mut chars: Vec<Vec<char>> = vec![Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    for line in reader.lines() {
        let linechars: Vec<char> = line.unwrap().chars().collect();
        for i in 0..12 {
            chars[i].push(linechars[i]);
        }
    }

    let mut gamma_str: String = String::new();
    let mut epsilon_str: String = String::new();
    for i in 0..12 {
        let count0 = chars[i].iter().filter(|&&x| x == '0').count();
        let count1 = chars[i].iter().filter(|&&x| x == '1').count();
        println!("chars[{}], count0: {}, count1: {}", i, count0, count1);
        if count0 > count1 {    
            gamma_str.push('0');
            epsilon_str.push('1');
        } else {
            gamma_str.push('1');
            epsilon_str.push('0');
        }
    }

    let gamma = usize::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon_str, 2).unwrap();

    println!("gamma_str: {}, epsilon_str: {}", gamma_str, epsilon_str);
    println!("gamma: {}, epsilon: {}", gamma, epsilon);
    println!("part1: {}", gamma * epsilon);
}
