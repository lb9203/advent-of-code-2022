use crate::input::{my_input};
use array_tool::vec::Intersect;

mod input;

fn main() {
    let input_lines: Vec<&str> = my_input().lines().map(|x| x.trim()).filter(|x| !x.is_empty()).collect();
    let mut running_count_one: u32 = 0;

    for line in input_lines.clone() {
        let items: Vec<char> = line.chars().collect();
        let first: &Vec<char> = &items[..items.len()/2].to_vec();
        let second: &Vec<char> = &items[items.len()/2..].to_vec();

        running_count_one += get_priority(first.to_vec().intersect(second.to_vec())[0]);
    }

    println!("{} is the sum of priorities of all item types found in both compartments.", running_count_one);

    let mut running_count_two: u32 = 0;
    for chunk in input_lines.clone().chunks(3) {
        let first: Vec<char> = chunk[0].chars().collect();
        let second: Vec<char> = chunk[1].chars().collect();
        let third: Vec<char> = chunk[2].chars().collect();

        running_count_two += get_priority(first.intersect(second.intersect(third))[0]);
    }

    println!("{} is the sum of priorities of all item types that correspond to the Elves' badges.", running_count_two);
}

fn get_priority(c: char) -> u32 {
    if !c.is_alphabetic() || !c.is_ascii() {
        panic!("{} is not a valid item type.", c)
    }

    return c as u32 - (if c.is_lowercase() { 96 } else { 38 });
}