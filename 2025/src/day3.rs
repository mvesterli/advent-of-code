#![cfg(test)]

#[test]
fn part1() {
    let input = include_str!("../input/day3.txt");
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let result: u32 = input
        .iter()
        .map(|bank| {
            let mut max_idx = 0;
            for i in 0..bank.len() - 1 {
                if bank[i] > bank[max_idx] {
                    max_idx = i;
                }
            }
            let second = *bank[max_idx + 1..].iter().max().unwrap();
            bank[max_idx] * 10 + second
        })
        .sum();
    assert_eq!(17092, result);
}

#[test]
fn part2() {
    let input = include_str!("../input/day3.txt");
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let result: u64 = input
        .iter()
        .map(|bank| {
            let mut result = 0u64;

            let mut digit_start = 0;
            for i in 0..12 {
                let mut max_idx = digit_start;
                for j in digit_start + 1..bank.len() - (11 - i) {
                    if bank[j] > bank[max_idx] {
                        max_idx = j;
                    }
                }
                result = result * 10 + bank[max_idx] as u64;
                digit_start = max_idx + 1;
            }

            result
        })
        .sum();
    assert_eq!(170147128753455, result);
}
