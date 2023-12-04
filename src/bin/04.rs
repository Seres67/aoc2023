advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.lines().collect::<Vec<&str>>().iter() {
        let mut tmp: u32 = 0;
        let pos = line.find(":").unwrap();
        let split_numbers = &line[pos + 1..].split("|").collect::<Vec<&str>>();
        let winning_numbers = split_numbers[0];
        let numbers: &str = split_numbers[1];
        let numbers: Vec<char> = numbers.chars().collect();
        let numbers = &numbers.chunks(3);
        for number in numbers.clone().into_iter() {
            let mut number: String = number.iter().collect();
            if number.is_empty() {
                continue;
            }
            if number.len() == 2 {
                number.push(' ');
            }
            if winning_numbers.contains(number.as_str()) {
                if tmp == 0 {
                    tmp += 1;
                } else if tmp != 0 {
                    tmp *= 2;
                }
            }
        }
        count += tmp;
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let len = input.lines().count();
    let mut cards: Vec<u32> = vec![1; len];
    for (card_id, line) in input.lines().collect::<Vec<&str>>().iter().enumerate() {
        let mut tmp: u32 = 0;
        let pos = line.find(":").unwrap();
        let split_numbers = &line[pos..].split("|").collect::<Vec<&str>>();
        let winning_numbers = split_numbers[0];
        let numbers = split_numbers[1];
        let numbers: Vec<char> = numbers.chars().skip(1).collect();
        let numbers = &numbers.chunks(3);
        for number in numbers.clone().into_iter() {
            let mut number: String = number.iter().collect();
            if number.is_empty() {
                continue;
            }
            if number.len() == 2 {
                number.push(' ');
            }
            if winning_numbers.contains(number.as_str()) {
                tmp += 1;
            }
        }
        for i in card_id + 1..=card_id + tmp as usize {
            cards[i] += cards[card_id];
        }
    }
    Some(cards.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
