advent_of_code::solution!(3);

fn check_symbol(input: &str, i: usize, x: usize) -> bool {
    let haystack = ['#', '$', '%', '&', '*', '+', '-', '/', '=', '@'];
    let left = input.chars().nth(i.wrapping_sub(1)).unwrap_or(' ');
    let right = input.chars().nth(i + 1).unwrap_or(' ');
    let up = input.chars().nth(i.wrapping_sub(x)).unwrap_or(' ');
    let upleft = input.chars().nth(i.wrapping_sub(x + 1)).unwrap_or(' ');
    let upright = input.chars().nth(i.wrapping_sub(x - 1)).unwrap_or(' ');
    let down = input.chars().nth(i.wrapping_add(x)).unwrap_or(' ');
    let downleft = input.chars().nth(i.wrapping_add(x + 1)).unwrap_or(' ');
    let downright = input.chars().nth(i.wrapping_add(x - 1)).unwrap_or(' ');
    let str = format!("{left}{right}{up}{upleft}{upright}{down}{downleft}{downright}");
    str.contains(haystack)
}

pub fn part_one(input: &str) -> Option<u32> {
    let x = input.lines().count();

    let input = input.chars().filter(|c| *c != '\n').collect::<String>();
    let mut tmp = String::new();
    let mut valid = false;
    let mut valid_numbers: Vec<u32> = Vec::new();
    for (i, char) in input.chars().enumerate() {
        if char.is_ascii_digit() {
            tmp.push(char);
            if check_symbol(&input, i, x) {
                valid = true;
            }
        } else if valid {
            valid_numbers.push(tmp.parse().unwrap());
            valid = false;
            tmp.clear();
        } else {
            valid = false;
            tmp.clear();
        }
    }
    Some(valid_numbers.iter().sum())
}

fn check_star(input: &str, i: usize, x: usize) -> usize {
    fn get_char(input: &str, index: usize) -> char {
        input.chars().nth(index).unwrap_or(' ')
    }

    let get_direction_char = |offset| get_char(input, i.wrapping_add(offset));

    let directions = [-1, 0, 1];
    for &dx in &directions {
        for &dy in &directions {
            if dx == 0 && dy == 0 {
                continue;
            }

            let direction_char = get_direction_char((dy * x as isize + dx) as usize);
            if direction_char == '*' {
                let pos: isize = dy * (x as isize) + dx;
                return i.wrapping_add(pos as usize);
            }
        }
    }

    0
}

#[derive(Debug)]
struct ValidNumber {
    pub num: i32,
    pub star_pos: isize,
}

pub fn part_two(input: &str) -> Option<u32> {
    let x = input.lines().count();
    let input = input.chars().filter(|c| *c != '\n').collect::<String>();
    let mut valid_number = false;
    let mut tmp: String = String::default();
    let mut tmp_star = 0;
    let mut valid_numbers: Vec<ValidNumber> = Vec::new();

    for (i, char) in input.chars().enumerate() {
        if char.is_ascii_digit() {
            tmp.push(char);
            if tmp_star == 0 {
                tmp_star = check_star(&input, i, x);
                if tmp_star != 0 {
                    valid_number = true;
                }
            }
        } else if valid_number {
            valid_numbers.push(ValidNumber {
                num: tmp.parse().unwrap(),
                star_pos: tmp_star as isize,
            });
            tmp.clear();
            valid_number = false;
            tmp_star = 0;
        } else {
            tmp.clear();
            valid_number = false;
            tmp_star = 0;
        }
    }
    let mut sum = 0;
    valid_numbers.sort_by(|x, y| x.star_pos.cmp(&y.star_pos));
    for i in 0..valid_numbers.len() - 1 {
        if valid_numbers[i].star_pos == valid_numbers[i + 1].star_pos {
            sum += valid_numbers[i].num * valid_numbers[i + 1].num;
        }
    }
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
