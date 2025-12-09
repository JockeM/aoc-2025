use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    input
        .lines()
        .map(|line| {
            line.split_once(',')
                .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
                .unwrap()
        })
        .combinations(2)
        .map(|a| get_rect_area(&a[0], &a[1]))
        .max()
}

fn get_rect_area(a: &(i32, i32), b: &(i32, i32)) -> u64 {
    let width = (a.0 - b.0).abs() as u64;
    let height = (a.1 - b.1).abs() as u64;
    (width + 1) * (height + 1)
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
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
