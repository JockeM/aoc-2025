advent_of_code::solution!(1);

#[inline(always)]
fn parse_int_until_nl_or_eof<I>(it: &mut I) -> i64
where
    I: Iterator<Item = u8>,
{
    let mut acc: i64 = 0;
    for b in it.by_ref() {
        if b == b'\n' {
            break;
        }
        acc = acc * 10 + (b - b'0') as i64;
    }
    acc
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut it = input.bytes();
    let mut zeros = 0i64;
    let mut dial = 50i64;

    while let Some(first) = it.next() {
        let pos = first == b'R';
        let number = parse_int_until_nl_or_eof(&mut it);

        if pos {
            dial += number;
            if dial >= 100 {
                dial %= 100;
            }
        } else {
            dial -= number;
            if dial < 0 {
                dial = dial.rem_euclid(100);
            }
        }
        if dial == 0 {
            zeros += 1;
        }
    }

    Some(zeros)
}


#[inline(always)]
fn count_zero_hits(dial: i64, n: i64, pos: bool) -> i64 {
    if n <= 0 { return 0; }
    let mut k0 = if pos {
        (100 - dial) % 100
    } else {
        dial % 100
    };
    if k0 == 0 { k0 = 100; }
    if n < k0 { 0 } else { 1 + (n - k0) / 100 }
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut it = input.bytes();

    let mut dial: i64 = 50;
    let mut count: i64 = 0;

    while let Some(first) = it.next() {
        let pos = first == b'R';
        let n = parse_int_until_nl_or_eof(&mut it);
        count += count_zero_hits(dial, n, pos);
        if pos {
            dial = (dial + n) % 100;
        } else {
            dial = (dial - n).rem_euclid(100);
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
        assert_eq!(result, Some(6));
    }
}
