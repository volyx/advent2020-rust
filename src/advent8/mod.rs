use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::HashSet;
// 1766
pub fn solution() {
    let file = File::open("advent8.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let raw_line = line.unwrap();
        lines.push(raw_line);
    }

    let res = check_program(lines.clone());
    println!("acc {:?}", res.0);

    for i in 0..lines.len() {
        let cmd = lines.get(i).unwrap();
        let mut program: Vec<String> = Vec::new();
        program = lines.clone();
        if cmd.contains("nop") {
            program[i] = program[i].replace("nop", &"jmp");
        } else if cmd.contains("jmp") {
            program[i] = program[i].replace("jmp", &"nop");
        } else {
            continue;
        }
        // println!("check i {:?} program {:?}", i, program);
        let res = check_program(program);

        if res.1 {
            println!("acc {:?}", res.0);
            break;
        }
    }
}


fn check_program(lines: Vec<String>) -> (i32, bool) {
    let mut acc: i32 = 0;
    let mut set: HashSet<usize> = HashSet::new();
    let mut i = 0;
    while i < lines.len() {
        let line = lines.get(i).unwrap();
        let key_value: Vec<&str> = line.split_ascii_whitespace().collect();
        let value = key_value[1];
        if set.contains(&i) {
           return (acc, false)
        }
        set.insert(i);

        match key_value[0] {
            "acc" => {
                let sign: char = value.chars().nth(0).unwrap();
                let step = value[1..].parse::<usize>().unwrap();
                match sign {
                    '+' => {
                        acc = acc + step as i32;
                    }
                    '-' => {
                        acc = acc - step as i32;
                    }
                    _ => {}
                }
                i = i + 1;
            }
            "nop" => {
                i = i + 1;
            }
            "jmp" => {
                let sign: char = value.chars().nth(0).unwrap();
                let i_step = value[1..].parse::<usize>().unwrap();
                match sign {
                    '+' => {
                        i = i + i_step;
                    }
                    '-' => {
                        i = i - i_step;
                    }
                    _ => {}
                }
            }
            &_ => {}
        }
    }
    (acc, true)
}