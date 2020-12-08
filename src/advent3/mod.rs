
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn solution() {
    let file = File::open("advent3.txt").unwrap();
    let reader = BufReader::new(file);
    let mut maze: Vec<Vec<usize>> = Vec::new();
    for line in reader.lines() {
        let mut maze_line: Vec<usize> = Vec::new();
        let raw_line = line.unwrap();
        let mut i: i32 = 0;
        for c in raw_line.chars() {
            let value: usize;
            match c {
                '.' => {
                    value = 0;
                },
                '#' => {
                    value = 1;
                },
                _ => {
                    println!("{:?}", c);
                    panic!()
                }
            }
            maze_line.insert(i as usize, value);
            i = i + 1;
        }
        // println!("{:?}", &maze_line);
        maze.push(maze_line);
    }

    /*
    Right 1, down 1.
    Right 3, down 1. (This is the slope you already checked.)
    Right 5, down 1.
    Right 7, down 1.
    Right 1, down 2.
*/

    let mut answer:i128 = 1;
    for tuple in vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)] {
        let row_step = tuple.0;
        let col_step = tuple.1;
        let mut i = 0;
        let mut j = 0;
        let mut count = 0;

        while i < maze.len() - 1 {
            let row = maze.get(i + row_step).unwrap();
            let value = row.get((j + col_step) % row.len()).unwrap();
            if value.eq(&1) {
                count = count + 1;
            }
            j = j + col_step;
            i = i + row_step;
        }
        answer = answer * count;
    }

    println!("{:?}", answer);
}