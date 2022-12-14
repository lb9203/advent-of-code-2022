mod input;

use crate::input::get_input;
use std::num::IntErrorKind::Empty;

fn main() {
    let input = get_input();

    let calories = top(input, 3);

    println!("The Elf with the most calories has {} calories.\n", calories[0]);

    let sum: i32 = calories.iter().sum();

    println!("The top 3 Elves have a combined {} calories.", sum)
}

fn top(input: &str, number: usize) -> Vec<i32> {
    let mut calories: Vec<i32> = vec![];
    let mut running_count = 0;
    for line in input.lines() {
        match line.trim().parse::<i32>() {
            Ok(n) => {
                running_count += n
            }
            Err(e) => {
                if e.kind() != &Empty {
                    panic!("Can't parse number from input.")
                }

                calories.push(running_count);

                running_count = 0
            }
        }
    }

    calories.sort();
    calories.reverse();
    calories.truncate(number);

    return calories
}

#[cfg(test)]
mod tests {
    use crate::top;

    const TEST_INPUT: &str = "
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    ";

    #[test]
    fn test_day_1_1() {
        assert_eq!(top(TEST_INPUT, 1)[0], 24000);
    }

    #[test]
    fn test_day_1_2() {
        assert_eq!(top(TEST_INPUT, 3).iter().sum::<i32>(), 45000);
    }
}