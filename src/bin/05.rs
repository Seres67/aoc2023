use std::cmp::Ordering;
advent_of_code::solution!(5);

#[derive(Debug)]
struct Map {
    pub dest: u64,
    pub source: u64,
    pub len: u64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();
    let seeds_str: String = lines.clone().take(1).collect();
    let seeds_pos = seeds_str.find(": ").unwrap();
    let seeds_split = seeds_str[seeds_pos + 2..].split(' ');
    let mut seeds: Vec<u64> = Vec::new();
    for seed in seeds_split {
        seeds.push(seed.parse().unwrap());
    }
    let mut maps: Vec<Vec<Map>> = Vec::new();
    maps.push(Vec::new());
    let mut i: usize = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if !line.chars().next().unwrap().is_ascii_digit() {
            maps.push(Vec::new());
            i += 1;
            continue;
        }
        let ranges = line.split(' ').collect::<Vec<&str>>();
        maps[i].push(Map {
            dest: ranges[0].parse().unwrap(),
            source: ranges[1].parse().unwrap(),
            len: ranges[2].parse().unwrap(),
        });
    }
    let mut i = 0;
    while i < maps.len() {
        for seed in seeds.iter_mut() {
            for map in &maps[i] {
                let source = map.source;
                let dest = map.dest;
                let len = source + map.len;
                match source.cmp(&dest) {
                    Ordering::Greater => {
                        let add = source.checked_sub(dest);
                        let valid = (source..=len).contains(seed);
                        if valid {
                            let add = add.unwrap();
                            *seed -= add;
                        }
                    }
                    Ordering::Less => {
                        let add = dest.checked_sub(source);
                        let valid = (source..=len).contains(seed);
                        if valid {
                            let add = add.unwrap();
                            *seed += add;
                        }
                    }
                    _ => (),
                }
            }
        }
        i += 1;
    }
    seeds.sort();
    Some(*seeds.first().unwrap())
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone)]
struct SeedCollection {
    pub start: u64,
    pub len: u64,
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();
    let seeds_str: String = lines.clone().take(1).collect();
    let seeds_pos = seeds_str.find(": ").unwrap();
    let seeds_split: Vec<u64> = seeds_str[seeds_pos + 2..]
        .split(' ')
        .map(|f| f.parse().unwrap())
        .collect();
    let mut seeds: Vec<SeedCollection> = Vec::new();
    for seed_pair in seeds_split.chunks(2) {
        seeds.push(SeedCollection {
            start: seed_pair[0],
            len: seed_pair[1],
        })
    }
    let mut maps: Vec<Vec<Map>> = Vec::new();
    maps.push(Vec::new());
    let mut i = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if !line.chars().next().unwrap().is_ascii_digit() {
            maps.push(Vec::new());
            i += 1;
            continue;
        }
        let ranges = line.split(' ').collect::<Vec<&str>>();
        maps[i].push(Map {
            dest: ranges[0].parse().unwrap(),
            source: ranges[1].parse().unwrap(),
            len: ranges[2].parse().unwrap(),
        });
    }
    for map in maps.iter_mut() {
        map.sort_unstable_by(|x, y| x.source.cmp(&y.source));
    }
    let mut i = 0;
    while i < maps.len() {
        let mut tmp = Vec::new();
        for seed in seeds.iter_mut() {
            for map in maps[i].iter() {
                if map.source >= seed.start && map.source < seed.start + seed.len {
                    if map.source > seed.start {
                        tmp.push(SeedCollection {
                            start: seed.start,
                            len: map.source - seed.start,
                        });
                        seed.len -= map.source - seed.start;
                        seed.start = map.source;
                    }
                    let low = map.len.min(seed.len);
                    tmp.push(SeedCollection {
                        start: map.dest,
                        len: low,
                    });
                    seed.start += low;
                    seed.len -= low;
                } else if map.source < seed.start && map.source + map.len > seed.start {
                    let low = (map.source + map.len - seed.start).min(seed.len);
                    tmp.push(SeedCollection {
                        start: seed.start - map.source + map.dest,
                        len: low,
                    });
                    seed.start += low;
                    seed.len -= low;
                }
            }
            if seed.len > 0 {
                tmp.push(SeedCollection {
                    start: seed.start,
                    len: seed.len,
                });
            }
        }
        seeds = tmp;
        i += 1;
    }
    seeds.sort_unstable_by(|x, y| x.start.cmp(&y.start));
    Some(seeds.first().unwrap().start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
