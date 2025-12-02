#![cfg(test)]

fn is_invalid(n: i64) -> bool {
    let s = n.to_string();
    if s.len() % 2 == 1 {
        return false;
    }
    s[..s.len() / 2] == s[s.len() / 2..]
}

#[test]
fn part1() {
    let input = include_str!("../input/day2.txt");
    let input: Vec<Vec<i64>> = input
        .split(',')
        .map(|s| s.split('-').map(|n| n.parse().unwrap()).collect())
        .collect();
    let res: i64 = input
        .iter()
        .map(|l| (l[0]..=l[1]).filter(|n| is_invalid(*n)).sum::<i64>())
        .sum();
    assert_eq!(26255179562, res);
}

fn is_repeated_pattern(s: &str, pattern_len: usize) -> bool {
    if s.len() % pattern_len != 0 {
        return false;
    }
    let pattern = &s[0..pattern_len];
    let mut start = pattern_len;
    while start < s.len() {
        if &s[start..start + pattern_len] != pattern {
            return false;
        }
        start += pattern_len;
    }
    true
}

fn is_invalid2(n: i64) -> bool {
    let s = n.to_string();
    for pattern_len in 1..=s.len() / 2 {
        if is_repeated_pattern(&s, pattern_len) {
            return true;
        }
    }
    false
}

#[test]
fn part2() {
    let input = include_str!("../input/day2.txt");
    let input: Vec<Vec<i64>> = input
        .split(',')
        .map(|s| s.split('-').map(|n| n.parse().unwrap()).collect())
        .collect();
    let res: i64 = input
        .iter()
        .map(|l| (l[0]..=l[1]).filter(|n| is_invalid2(*n)).sum::<i64>())
        .sum();
    assert_eq!(31680313976, res);
}
