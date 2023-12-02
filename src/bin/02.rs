use std::str::FromStr;

advent_of_code::solution!(2);

#[derive(Debug, PartialEq)]
enum Color {
    Red,
    Blue,
    Green,
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Color::Red),
            "blue" => Ok(Color::Blue),
            "green" => Ok(Color::Green),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Cube {
    pub count: u32,
    pub color: Color,
}

#[derive(Debug)]
struct Game {
    pub game_id: u32,
    pub cubes: Vec<Cube>,
    pub valid_count: u32,
}

fn check_red(count: u32) -> bool {
    if count > 12 {
        false
    } else {
        true
    }
}

fn check_green(count: u32) -> bool {
    if count > 13 {
        false
    } else {
        true
    }
}

fn check_blue(count: u32) -> bool {
    if count > 14 {
        false
    } else {
        true
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut games = Vec::new();
    for line in lines.iter() {
        let mut tokens = line.split(" ").collect::<Vec<&str>>();
        if tokens.len() < 2 {
            continue;
        }
        let mut chars = tokens[1].chars();
        chars.next_back();
        let game_id = chars.as_str();
        let mut game = Game {
            game_id: game_id.parse().unwrap(),
            cubes: Vec::new(),
            valid_count: 0,
        };
        let drain = tokens.drain(2..).collect::<Vec<&str>>();
        for slice in drain.chunks(2) {
            let count: u32 = slice[0].parse().unwrap();
            let chars = slice[1]
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>();
            let color = Color::from_str(chars.as_str()).unwrap();
            game.cubes.push(Cube { count, color })
        }
        games.push(game);
    }
    for game in games.iter_mut() {
        for cube in game.cubes.iter() {
            let valid = match cube.color {
                Color::Red => check_red(cube.count),
                Color::Green => check_green(cube.count),
                Color::Blue => check_blue(cube.count),
            };
            if valid {
                game.valid_count += 1;
            }
        }
    }
    Some(
        games
            .iter()
            .filter(|game| game.valid_count == game.cubes.len() as u32)
            .map(|game| game.game_id)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.split("\n").collect::<Vec<&str>>();
    let mut games = Vec::new();
    for line in lines.iter() {
        let mut tokens = line.split(" ").collect::<Vec<&str>>();
        if tokens.len() < 2 {
            continue;
        }
        let mut chars = tokens[1].chars();
        chars.next_back();
        let game_id = chars.as_str();
        let mut game = Game {
            game_id: game_id.parse().unwrap(),
            cubes: Vec::new(),
            valid_count: 0,
        };
        let drain = tokens.drain(2..).collect::<Vec<&str>>();
        for slice in drain.chunks(2) {
            let count: u32 = slice[0].parse().unwrap();
            let chars = slice[1]
                .chars()
                .filter(|c| c.is_alphabetic())
                .collect::<String>();
            let color = Color::from_str(chars.as_str()).unwrap();
            game.cubes.push(Cube { count, color })
        }
        games.push(game);
    }
    let mut power = Vec::new();
    for game in games.iter_mut() {
        let red_cubes: Vec<&Cube> = game
            .cubes
            .iter()
            .filter(|cube| cube.color == Color::Red)
            .collect();
        let green_cubes: Vec<&Cube> = game
            .cubes
            .iter()
            .filter(|cube| cube.color == Color::Green)
            .collect();
        let blue_cubes: Vec<&Cube> = game
            .cubes
            .iter()
            .filter(|cube| cube.color == Color::Blue)
            .collect();
        let red_max = red_cubes
            .iter()
            .max_by(|x, y| x.count.cmp(&y.count))
            .unwrap()
            .count;
        let green_max = green_cubes
            .iter()
            .max_by(|x, y| x.count.cmp(&y.count))
            .unwrap()
            .count;
        let blue_max = blue_cubes
            .iter()
            .max_by(|x, y| x.count.cmp(&y.count))
            .unwrap()
            .count;
        power.push(red_max * green_max * blue_max);
    }
    Some(power.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
