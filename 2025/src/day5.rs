#![cfg(test)]

fn parse_input(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();

    let mut reading_ranges = true;
    for line in input.lines() {
        if line.is_empty() {
            reading_ranges = false;
        } else if reading_ranges {
            let parts: Vec<i64> = line.split('-').map(|n| n.parse().unwrap()).collect();
            ranges.push((parts[0], parts[1]));
        } else {
            ids.push(line.parse().unwrap());
        }
    }
    (ranges, ids)
}

#[test]
fn part1() {
    let input = include_str!("../input/day5.txt");
    let (ranges, ids) = parse_input(input);
    let res = ids
        .iter()
        .filter(|id| ranges.iter().any(|r| **id >= r.0 && **id <= r.1))
        .count();
    assert_eq!(525, res);
}

#[test]
fn part2() {
    let input = include_str!("../input/day5.txt");
    let (mut ranges, _) = parse_input(input);
    ranges.sort_by_key(|r| r.0);

    let mut res = 0;
    let mut next = ranges[0].0;
    for r in ranges {
        if r.0 > next {
            next = r.0;
        }
        if r.1 >= next {
            res += r.1 - next + 1;
            next = r.1 + 1;
        }
    }
    assert_eq!(333892124923577, res);
}
