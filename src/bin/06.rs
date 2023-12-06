advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let times_pos = lines[0].find(": ").unwrap() + 2;
    let distance_pos = lines[1].find(": ").unwrap() + 2;
    let times = lines[0][times_pos..]
        .split(' ')
        .filter(|t| !t.is_empty())
        .map(|t| t.trim_start().parse().unwrap())
        .collect::<Vec<u32>>();
    let distances = lines[1][distance_pos..]
        .split(' ')
        .filter(|d| !d.is_empty())
        .map(|d| d.parse().unwrap())
        .collect::<Vec<u32>>();
    let zip = times.iter().zip(distances.iter());
    let mut count = 1;
    for (time, distance) in zip {
        let mut tmp_count = 0;
        for i in 0..*time {
            let speed = i;
            let move_time = time - i;
            if speed * move_time > *distance {
                tmp_count += 1;
            }
        }
        count *= tmp_count;
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let times_pos = lines[0].find(": ").unwrap() + 2;
    let distance_pos = lines[1].find(": ").unwrap() + 2;
    let time = lines[0][times_pos..]
        .chars()
        .filter(|c| *c != ' ')
        .collect::<String>()
        .parse()
        .unwrap();
    let distance = lines[1][distance_pos..]
        .chars()
        .filter(|c| *c != ' ')
        .collect::<String>();
    let distance: u64 = distance.parse().unwrap();
    let mut count = 0;
    for i in 0..time {
        let speed = i;
        let move_time = time - i;
        if speed * move_time > distance {
            count += 1;
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
