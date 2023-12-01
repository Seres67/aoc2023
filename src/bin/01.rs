advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let splitted = input.lines();
    let mut num_array: Vec<char> = Vec::new();
    let mut calibrations_numbers: Vec<u32> = Vec::new();
    for str in splitted.into_iter() {
        for char in str.chars() {
            if char.is_digit(10) {
                num_array.push(char);
            }
        }
        let number: String = format!(
            "{}{}",
            num_array.first().unwrap(),
            num_array.last().unwrap_or(num_array.first().unwrap())
        );
        calibrations_numbers.push(number.parse().unwrap());
        num_array.clear();
    }
    let sum = calibrations_numbers.iter().sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    // replacing like this works, but why?
    let splitted = input
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
        .replace("zero", "zero0zero");
    let splitted = splitted.lines();
    let mut num_array: Vec<char> = Vec::new();
    let mut calibrations_numbers: Vec<u32> = Vec::new();
    for str in splitted.into_iter() {
        for char in str.chars() {
            if char.is_digit(10) {
                num_array.push(char);
            }
        }
        let number: String = format!(
            "{}{}",
            num_array.first().unwrap(),
            num_array.last().unwrap_or(num_array.first().unwrap())
        );
        calibrations_numbers.push(number.parse().unwrap());
        num_array.clear();
    }
    let sum = calibrations_numbers.iter().sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
