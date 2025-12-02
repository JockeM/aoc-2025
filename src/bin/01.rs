advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i64> {
    let s = input.split('\n').filter_map(|line| {
        let pos = line.chars().nth(0)? == 'R';
        let number = line[1..].parse::<i64>().ok()?;

        return Some((pos, number));
    });

    let mut dial = 50;
    let mut count = 0;
    for (pos, number) in s {
        if pos {
            dial += number;
        } else {
            dial -= number;
        }
        while dial > 99 {
            dial -= 100;
        }
        while dial < 0 {
            dial += 100;
        }
        if dial == 0 {
            count += 1;
        }
    }

    return Some(count);
}

pub fn part_two(input: &str) -> Option<i64> {
    let s = input.split('\n').filter_map(|line| {
        let pos = line.chars().nth(0)? == 'R';
        let number = line[1..].parse::<i64>().ok()?;

        return Some((pos, number));
    });

    let mut dial = 50;
    let mut count = 0;
    for (pos, number) in s {
        if pos {
            for _ in 0..number {
                dial += 1;
                if dial > 99 {
                    dial = 0;
                }
                if dial == 0 {
                    count += 1;
                }
            }
        } else {
            for _ in 0..number {
                dial -= 1;
                if dial < 0 {
                    dial = 99;
                }
                if dial == 0 {
                    count += 1;
                }
            }
        }
    }

    return Some(count);
}

// too high 5967

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
        assert_eq!(result, Some(6));
    }
}
