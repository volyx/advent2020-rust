use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solution() {
    let file = File::open("advent9.txt").unwrap();
    let reader = BufReader::new(file);
    let preamble: usize = 25;
    let mut preamble_window = Vec::with_capacity(preamble);
    let mut i: usize = 0;
    let mut current_index = 0;
    let mut number: usize = 0;
    let mut numbers: Vec<usize> = Vec::new();
    for line in reader.lines() {
        number = line.unwrap().parse::<usize>().unwrap();
        numbers.push(number);

        current_index = i % preamble;

        if i >= preamble {
            if !find_sum(&preamble_window, number) {
                break;
            }
        }

        if preamble_window.len() == preamble {
            preamble_window.remove(current_index);
            preamble_window.insert(current_index, number);
        } else {
            preamble_window.push(number);
        }

        i = i + 1;
    }
    println!("{:?}", number);

    let mut max_size = 0;
    let mut start_i = 0;
    let mut min_el = 0;
    let mut max_el = 0;
    for i in 0..numbers.len() - 1 {
        let mut j = i + 1;

        let mut i_sum = numbers[i];
        let mut min = numbers[i];
        let mut max = numbers[i];
        while i_sum < number {
            i_sum = i_sum + numbers[j];
            min = std::cmp::min(min, numbers[j]);
            max = std::cmp::max(max, numbers[j]);
            j = j + 1;
        }

        if i_sum == number {
            if j - i > max_size {
                start_i = i;
                max_size = j - i;
                max_el = max;
                min_el = min;
            }
        }
    }

    println!("max_size {:?} start_i {:?}", max_size, start_i);
    println!("{:?}", min_el + max_el);
}

fn find_sum(preamble_window: &[usize], sum: usize) -> bool {
    for i in 0..preamble_window.len() - 1 {
        for j in i + 1..preamble_window.len() {
            if (preamble_window[i] + preamble_window[j]) == sum {
                return true
            }
        }
    }

    false
}

