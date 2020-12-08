use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::Deref;

pub fn solution() {
    let file = File::open("advent2.txt").unwrap();
    let reader = BufReader::new(file);
    let mut valid: i32 = 0;
    for line in reader.lines() {
        let raw_line = line.unwrap();
        let parts: Vec<&str> = raw_line.split_ascii_whitespace().collect();

        let range = parts.get(0).unwrap().deref();

        let numbers: Vec<&str> = range.split('-').collect();

        let min = numbers.get(0).unwrap().deref().parse::<usize>().unwrap() - 1;
        let max = numbers.get(1).unwrap().deref().parse::<usize>().unwrap() - 1;

        let character = parts.get(1).unwrap().deref();

        let c = character.chars().next().unwrap();

        let char_vec: Vec<char> = parts.get(2).unwrap().chars().collect();

        let mut count = 0;

        let first: &char = char_vec.get(min).unwrap();
        if c.eq(&first) {
            count = count + 1;
        }

        let second: &char = char_vec.get(max as usize).unwrap();
        if c.eq(&second) {
            count = count + 1;
        }

        // println!("{:?} {:?} {:?} {:?}", char_vec, min, max, c);

        if count.eq(&1) {
            valid = valid + 1;
        }
    }
    println!("{}", valid)
}