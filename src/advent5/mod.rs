use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::Deref;

pub fn solution() {
    let file = File::open("advent5.txt").unwrap();
    let reader = BufReader::new(file);
    let mut seats: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let raw_line = line.unwrap();

        let mut row_low: i32 = 0;
        let mut row_high: i32 = 128;

        let mut col_low: i32 = 0;
        let mut col_high: i32 = 8;

        let mut row_mid = 0;
        let mut col_mid = 0;

        for c in raw_line.chars() {
            // println!("row_low = {:?}, row_high = {:?}", row_low, row_high);
            // println!("col_low = {:?}, col_high = {:?}", col_low, col_high);
            row_mid = row_low + (row_high  - row_low) / 2;
            col_mid = col_low + (col_high  - col_low) / 2;
            match c {
                'F' => {
                    row_high = row_mid;
                }
                'B' => {
                    row_low = row_mid;
                }
                'R' => {
                    col_low = col_mid;
                }
                'L' => {
                    col_high = col_mid;
                }
                _ => {}
            }
        }
        seats.push(row_low * 8 + col_low);
    }
    seats.sort();

    for i in 0..seats.len() - 1 {
        if seats[i + 1] - seats[i] != 1 {
            println!("{:?} {:?}", seats[i], seats[i + 1]);
        }
    }
}
