use crate::input::{my_input};
use array_tool::vec::Intersect;

mod input;

fn main() {
    println!("{} is the sum of priorities of all item types found in both compartments.", get_shared_item_priority_sum(my_input()));

    println!("{} is the sum of priorities of all item types that correspond to the Elves' badges.", get_badge_priority_sum(my_input()));
}

fn get_shared_item_priority_sum(input: &str) -> u32 {
    parse_input(input).iter().map(|x| get_shared_item_priority(*x)).sum()
}

fn get_badge_priority_sum(input: &str) -> u32 {
    parse_input(input).chunks(3).map(|elves| get_badge_priority(elves[0], elves[1], elves[2])).sum()
}

fn parse_input(input: &str) -> Vec<&str> {
    input.lines().map(|x| x.trim()).filter(|x| !x.is_empty()).collect()
}

fn get_badge_priority(first: &str, second: &str, third: &str) -> u32 {
    let first_items: Vec<char> = first.chars().collect();
    let second_items: Vec<char> = second.chars().collect();
    let third_items: Vec<char> = third.chars().collect();

    first_items.intersect(second_items.intersect(third_items)).iter().map(|x| get_priority(*x)).sum()
}

fn get_shared_item_priority(line: &str) -> u32 {
    let items: Vec<char> = line.chars().collect();
    let first: &Vec<char> = &items[..items.len()/2].to_vec();
    let second: &Vec<char> = &items[items.len()/2..].to_vec();
    let shared: Vec<char> = first.to_vec().intersect(second.to_vec());

    shared.iter().map(|x| get_priority(*x)).sum::<u32>()
}

fn get_priority(c: char) -> u32 {
    if !c.is_alphabetic() || !c.is_ascii() {
        panic!("{} is not a valid item type.", c)
    }

    return c as u32 - (if c.is_lowercase() { 96 } else { 38 });
}

#[cfg(test)]
mod tests {
    use crate::{get_badge_priority_sum, get_shared_item_priority_sum};

    const TEST_INPUT: &str = "
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    ";

    #[test]
    fn test_shared_item_sum() {
        assert_eq!(get_shared_item_priority_sum(TEST_INPUT), 157)
    }

    #[test]
    fn test_badge_sum() {
        assert_eq!(get_badge_priority_sum(TEST_INPUT), 70)
    }
}