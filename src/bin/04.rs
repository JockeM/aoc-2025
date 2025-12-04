use std::collections::HashMap;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((y as i32, x as i32), c)))
        .filter(|(_, c)| *c == '@')
        .collect::<HashMap<_, _>>();

    println!("{lines:?}");

    let mut count = 0;
    for (y, x) in lines.keys() {
        let total = vec![
            lines.get(&(y + 1, x + 1)).is_some(),
            lines.get(&(y + 1, x - 1)).is_some(),
            lines.get(&(y + 1, *x)).is_some(),
            lines.get(&(y - 1, x + 1)).is_some(),
            lines.get(&(y - 1, x - 1)).is_some(),
            lines.get(&(y - 1, *x)).is_some(),
            lines.get(&(*y, x + 1)).is_some(),
            lines.get(&(*y, x - 1)).is_some(),
        ];
        let bs = total.iter().filter(|b| **b).count();

        println!("{y} {x} {bs}");
        if bs <= 4 {
            count += 1;
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
