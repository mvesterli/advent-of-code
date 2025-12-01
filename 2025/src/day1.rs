#![cfg(test)]

#[test]
fn part1() {
    let input = include_str!("../input/day1.txt");
    let input = input
        .lines()
        .map(|l| l.split_at(1))
        .map(|(d, n)| (if d == "L" { -1 } else { 1 }) * n.parse::<i32>().unwrap());

    let mut counts = [0; 100];
    let mut idx = 50;
    for n in input {
        idx = (((idx + n) % 100) + 100) % 100;
        counts[idx as usize] += 1;
    }
    assert_eq!(1139, counts.into_iter().max().unwrap());
}

#[test]
fn part2() {
    let input = include_str!("../input/day1.txt");
    let input = input
        .lines()
        .map(|l| l.split_at(1))
        .map(|(d, n)| (if d == "L" { -1 } else { 1 }) * n.parse::<i32>().unwrap());

    let mut count = 0;
    let mut idx = 50;
    for n in input {
        let prev = idx;
        idx += n;
        count += idx.abs() / 100;
        if idx <= 0 && prev > 0 {
            count += 1;
        }
        idx = ((idx % 100) + 100) % 100;
    }
    assert_eq!(6684, count);
}
