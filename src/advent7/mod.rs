use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::ops::{Deref, Add, AddAssign};
use std::borrow::Cow;
use std::rc::Rc;
use std::cell::Cell;
use std::fmt;

pub fn solution() {
    let file = File::open("advent7.txt").unwrap();
    let reader = BufReader::new(file);

    let mut bags: Vec<Bag> = Vec::new();
    for line in reader.lines() {
        let raw_line = line.unwrap();
        let key_values: Vec<&str> = raw_line.split(" bags contain ").collect();

        let child_bags: Vec<&str> = key_values[1].split(",").collect();

        let mut children: Vec<Bag> = Vec::new();


        for child_bag in child_bags {
            if child_bag.eq("no other bags.") {

            } else {
                let tokens: Vec<&str> = child_bag.split_ascii_whitespace().collect();
                let id: String = format!("{} {}", tokens[1], tokens[2]);
                let mut count = 0;
                if !child_bag.contains("no other") {
                    count = tokens[0].parse::<i32>().unwrap();
                }
                children.push(Bag {id, count, children: Vec::default()});
            }
        }
        bags.push(Bag {id: key_values[0].deref().to_string(), count: 0, children})
    }

    for b in &bags {
        println!("{:?}", b);
    }

    part_2(bags);

}

fn part_2(bags: Vec<Bag>) {
    let mut count: Cell<i32> = Cell::from(0);
    let my_bag = "shiny gold";

    topological_find(&bags, my_bag, & mut count, 0, 1);

    println!("count {:?}", count.get());
}

fn topological_find(bags: &Vec<Bag>, current_bag: &str, count: & mut Cell<i32>, level: usize, multiplier: i32) {
    for bag in bags {
        if bag.id.eq(current_bag) {

            let current_bag : &Bag = bags.iter()
                .filter(|&b| b.id.eq(&bag.id))
                .next().unwrap();

            if current_bag.children.is_empty() {
                return;
            }
            println!("{} find children {:?} of {}", "\t".repeat(level), current_bag.children, bag);

            let mut child_count = 0;
            for bag_child in current_bag.children.as_slice() {

                topological_find(&bags, &bag_child.id, count, level + 1, bag_child.count);
                child_count = child_count + bag_child.count;
            }
            count.set(count.get() + child_count * multiplier);
            println!("{} count +{}", "\t".repeat(level), child_count * multiplier);
            // println!("{} count +1", "\t".repeat(level));
            // count.set(count.get() + 1);
        }
    }
}

fn part_1(bags: Vec<Bag>) {
    let mut count = 0;
    let my_bag = "shiny gold";
    for bag in bags.clone() {
        if bag.id.eq(my_bag) {
            println!("find by id");
            continue;
        }
        println!("start {:?}", bag);
        if find_bag(&bag, &bags, my_bag, 1) {
            count = count + 1;
            println!("count {:?}", bag);
        }
    }

    println!("count {:?}", count);
}

fn find_bag(bag: &Bag, bags_map: &Vec<Bag>, name: &str, level: usize) -> bool {

    if bag.id.eq(name) {
        println!("{} find by id", "\t".repeat(level));
        return true;
    }

    let current_bag : &Bag = bags_map.iter()
        .filter(|&b| b.id.eq(&bag.id))
        .next().unwrap();

    if current_bag.children.is_empty() {
        return false;
    }
    println!("{} find children {:?} of {:?}", "\t".repeat(level), current_bag.children, bag);

    for bag_child in current_bag.children.as_slice() {
        println!("{} go inside children of {:?}", "\t".repeat(level), bag_child);
        if find_bag(&bag_child, bags_map, name, level + 1) == true {
            return true;
        }
    }

    false
}

#[derive(Debug, Clone)]
struct Bag {
    id: String,
    children: Vec<Bag>,
    count: i32
}

impl fmt::Display for Bag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.id, self.count)
    }
}