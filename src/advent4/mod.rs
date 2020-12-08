
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::collections::{HashSet};
use std::ops::Deref;

pub fn solution() {
    let file = File::open("advent4.txt").unwrap();
    let reader = BufReader::new(file);
    let mut valid = 0;
    let mut fields = HashSet::new();
    fields.insert(&"byr");
    fields.insert(&"iyr");
    fields.insert(&"eyr");
    fields.insert(&"hgt");
    fields.insert(&"hcl");
    fields.insert(&"ecl");
    fields.insert(&"pid");
    let mut counter = 0;
    for line in reader.lines() {
        let raw_line = line.unwrap();
        if raw_line.is_empty() {
            if counter.eq(&7) {
                valid = valid + 1;
            } else {
            }
            counter = 0;
        }
        let parts: Vec<&str> = raw_line.split_ascii_whitespace().collect();


        for part in parts {
            let key_value: Vec<&str> = part.split(':').collect();

            let key= key_value.get(0).unwrap().deref();
            let value= key_value.get(1).unwrap().deref();

            if !fields.contains(&key) {
                continue;
            }

            match key {
                "byr" => {

                    if value.len() == 4 && value.ge("1920") && value.le("2002") {
                        counter = counter + 1;
                    }
                }
                "iyr" => {
                    if value.len() == 4 && value.ge("2010") && value.le("2020") {
                        counter = counter + 1;
                    }
                }
                "eyr" => {
                    if value.len() == 4 && value.ge("2020") && value.le("2030") {
                        counter = counter + 1;
                    }
                }
                "hgt" => {
                    let is_cm = if value.ends_with("cm") {
                        let number_value = value.strip_suffix("cm").unwrap();
                        number_value.ge("150") && number_value.le("193")
                    } else {
                        false
                    };
                    let is_in = if value.ends_with("in") {
                        let number_value = value.strip_suffix("in").unwrap();
                        number_value.ge("59") && number_value.le("76")
                    } else {
                        false
                    };
                    if is_cm || is_in {
                        counter = counter + 1;
                    }
                }
                "hcl" => {
                    if value.starts_with("#") && value.len().eq(&7) {
                        let mut matched: bool = false;
                        for i in 1..7 as usize {
                            let one_char: char = value.chars().nth(i).unwrap();
                            let is_number = one_char.ge(&'0') && one_char.le(&'9');
                            let is_letter = one_char.ge(&'a') && one_char.le(&'f');
                            if is_number || is_letter {
                                matched = true;
                            } else {
                                matched = false;
                                break
                            }
                        }
                        if matched {
                            counter = counter + 1;
                        }
                    }
                }
                "ecl" => {
                    if value.eq("amb")
                        || value.eq("blu")
                        || value.eq("brn")
                        || value.eq("gry")
                        || value.eq("grn")
                        || value.eq("hzl")
                        || value.eq("oth")
                    {
                        counter = counter + 1;
                    }
                }
                "pid" => {
                    let mut matched: bool = false;
                    if value.len() != 9 {
                        continue;
                    }
                    for i in 0..9 as usize {
                        let one_char: char = value.chars().nth(i).unwrap();
                        let is_number = one_char.ge(&'0') && one_char.le(&'9');
                        if is_number {
                            matched = true;
                        } else {
                            matched = false;
                            break
                        }
                    }
                    if matched {
                        counter = counter + 1;
                    }
                }
                &_ => {}
            }

        }
    }
    if counter.eq(&7) {
        valid = valid + 1;
    } else {
    }
    println!("valid {:?}", valid);
}
