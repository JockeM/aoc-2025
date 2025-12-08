use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut it = input.lines().rev();
    let operators = it.next().unwrap().split_whitespace();
    let number_rows = it
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    Some(
        operators
            .enumerate()
            .map(|(i, op)| {
                let numbers = number_rows.iter().map(|row| row.iter().nth(i).unwrap());
                if op == "*" {
                    numbers.copied().product::<u64>()
                } else {
                    numbers.copied().sum()
                }
            })
            .sum(),
    )
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
