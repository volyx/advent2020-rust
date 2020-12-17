use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solution() {
    let file = File::open("advent6.txt").unwrap();
    let reader = BufReader::new(file);

    let mut freq:[usize; 26] = [0; 26];
    let mut passenger_count = 0;
    let mut sum = 0;
    for line in reader.lines() {
        let raw_line = line.unwrap();
        if raw_line.is_empty() {
            for i in 0..freq.len() {
                if freq[i] == passenger_count {
                    sum = sum + 1;
                }
                freq[i] = 0;
            }
            passenger_count = 0;
        } else {
            passenger_count = passenger_count + 1;
            for c in raw_line.chars() {
                let index = (c as usize) - ('a' as usize);
                freq[index] = freq[index] + 1;
            }
        }
    }
    println!("{:?}", sum);
}
