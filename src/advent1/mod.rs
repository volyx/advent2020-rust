use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::{HashMap};

pub fn solution() {
    let file = File::open("advent1.txt").unwrap();
    let reader = BufReader::new(file);

    let mut two_sums: HashMap<i32, i32> = HashMap::new();
    let mut answer: i32 = -1;
    let mut arr = vec![];
    for line in reader.lines() {
        let year = line.unwrap().parse::<i32>().unwrap();
        arr.push(year);
    }
    arr.sort();
    for i in 0..arr.len() {
        let key = *arr.get(i).unwrap();
        for j in 0..i {
            two_sums.insert(arr.get(i).unwrap() + arr.get(j).unwrap(), key);
        }
        let search = 2020 - key;
        if two_sums.contains_key(&search) {
            let one = two_sums.get(&search).unwrap();
            answer = one * key * (2020 - one - key);
        }
    }
    println!("{}", answer);
}