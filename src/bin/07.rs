use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let start = input.chars().position(|c| c == 'S').unwrap();
    let lines = input.lines().count();
    let splitters = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '^')
                .map(move |(x, _)| (y, x))
        })
        .collect::<HashSet<_>>();

    let mut splits = 0;
    let mut positions = HashSet::from([start]);
    for current_y in 0..lines {
        let mut next = HashSet::new();
        for &x in positions.iter() {
            if splitters.contains(&(current_y, x)) {
                next.insert(x - 1);
                next.insert(x + 1);
                splits += 1;
            } else {
                next.insert(x);
            }
        }
        positions = next;
    }

    Some(splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let start = input.chars().position(|c| c == 'S').unwrap();
    let stop = input.lines().count();
    let splitters = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '^')
                .map(move |(x, _)| (y, x))
        })
        .collect::<HashSet<_>>();
    let mut cache = HashMap::new();
    Some(get_timelines((0, start), &splitters, &mut cache, stop) + 1)
}

type Pos = (usize, usize);

fn get_timelines(
    position: Pos,
    splitters: &HashSet<Pos>,
    cache: &mut HashMap<Pos, u64>,
    stop: usize,
) -> u64 {
    if let Some(value) = cache.get(&position) {
        return *value;
    }
    let (y, x) = position;

    if y == stop {
        return 0;
    }
    let result = if splitters.contains(&(y, x)) {
        get_timelines((y + 1, x - 1), splitters, cache, stop)
            + get_timelines((y + 1, x + 1), splitters, cache, stop)
            + 1
    } else {
        get_timelines((y + 1, x), splitters, cache, stop)
    };
    cache.insert(position, result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
