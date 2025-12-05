use itertools::Itertools;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, values) = input.split_once("\n\n").unwrap();
    let ranges = ranges
        .lines()
        .map(|line| {
            line.split_once("-")
                .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
                .unwrap()
        })
        .collect_vec();

    let mut sum = 0;
    for value in values.lines().map(|v| v.parse::<u64>().unwrap()) {
        for &(low, high) in ranges.iter() {
            if value >= low && value <= high {
                sum += 1;
                break;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = input.split_once("\n\n")
        .map(|(ranges, _)| ranges)
        .unwrap()
        .lines()
        .map(|line| {
            line.split_once("-")
                .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
                .unwrap()
        })
        .sorted_unstable_by(|a, b| a.0.cmp(&b.0))
        .collect_vec();

    let mut current_high = 0;
    let mut count = 0;
    for &(low, high) in &ranges {
        if low > current_high {
            count += high - low + 1;
            current_high = high;
        } else if high > current_high {
            count += high - current_high;
            current_high = high;
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
