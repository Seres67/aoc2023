advent_of_code::solution!(7);

#[derive(Debug)]
struct Hand {
    pub cards: String,
    pub bid: u32,
    pub power: u32,
}

fn eval(hands: &mut [Hand]) {
    let cards = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    for hand in hands.iter_mut() {
        let mut cards_count: Vec<u32> = vec![0; 13];
        for (i, card) in hand.cards.chars().enumerate() {
            let pos = cards.iter().position(|c| *c == card).unwrap();
            cards_count[pos] += 1;
            hand.power |= (pos as u32) << ((4 - i as u32) * 4);
        }
        cards_count.sort();
        cards_count.reverse();
        let power = match cards_count[0] {
            5 => 6,
            4 => 5,
            3 => {
                if cards_count[1] == 2 {
                    4
                } else {
                    3
                }
            }
            2 => {
                if cards_count[1] == 2 {
                    2
                } else {
                    1
                }
            }
            _ => 0,
        };
        hand.power |= power << 20;
    }

    hands.sort_unstable_by_key(|x| x.power);
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let split = line.split(' ').collect::<Vec<&str>>();
            let cards = split[0];
            let bid = split[1].parse().unwrap();
            Hand {
                cards: cards.to_string(),
                bid,
                power: 0,
            }
        })
        .collect();
    eval(&mut hands);
    let out = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + ((i + 1) * hand.bid as usize));
    Some(out as u32)
}
fn eval2(hands: &mut [Hand]) {
    let cards = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];
    for hand in hands.iter_mut() {
        let mut jokers = 0;
        let mut cards_count: Vec<u32> = vec![0; 13];
        for (i, card) in hand.cards.chars().enumerate() {
            let pos = cards.iter().position(|c| *c == card).unwrap();
            if pos == 0 {
                jokers += 1;
            } else {
                cards_count[pos] += 1;
            }
            hand.power |= (pos as u32) << ((4 - i as u32) * 4);
        }
        cards_count.sort();
        cards_count.reverse();
        let power = match cards_count[0] + jokers {
            5 => 6,
            4 => 5,
            3 => {
                if cards_count[1] == 2 {
                    4
                } else {
                    3
                }
            }
            2 => {
                if cards_count[1] == 2 {
                    2
                } else {
                    1
                }
            }
            _ => 0,
        };
        hand.power |= power << 20;
    }

    hands.sort_unstable_by_key(|x| x.power);
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let split = line.split(' ').collect::<Vec<&str>>();
            let cards = split[0];
            let bid = split[1].parse().unwrap();
            Hand {
                cards: cards.to_string(),
                bid,
                power: 0,
            }
        })
        .collect();
    eval2(&mut hands);
    let out = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + ((i + 1) * hand.bid as usize));
    Some(out as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
