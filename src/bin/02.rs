advent_of_code::solution!(2);
use itertools::*;

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input.trim().split(',').map(|range| {
        range
            .split_once('-')
            .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
            .unwrap()
    });
    let mut total = 0;
    for (a, b) in ranges {
        for n in a..=b {
            let str = n.to_string();
            let length = str.len();
            if !length.is_multiple_of(2) {
                continue;
            }
            let half = length / 2;
            if str[..half] == str[half..] {
                total += n;
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u128> {
    let ranges = input.trim().split(',').map(|range| {
        range
            .split_once('-')
            .map(|(a, b)| {
                let a1 = a.parse::<u128>();
                let b1 = b.parse::<u128>();
                (a1.unwrap(), b1.unwrap())
            })
            .unwrap()
    });

    let mut total = 0;
    for (a, b) in ranges {
        for n in a..=b {
            if is_invalid(n) {
                total += n;
            }
        }
    }

    Some(total)
}

fn is_invalid(n: u128) -> bool {
    let str = n.to_string();
    let len = str.len();
    for i in 1..len {
        if len.is_multiple_of(i)
            && str.as_bytes().chunks(i).all_equal() {
                return true;
            }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
