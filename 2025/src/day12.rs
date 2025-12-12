#![cfg(test)]

#[test]
fn part1() {
    let input = include_str!("../input/day12.txt");
    let input: Vec<Vec<i32>> = input
        .lines()
        .skip_while(|l| !l.contains('x'))
        .map(|l| l.split(&['x', ':', ' ']).flat_map(|n| n.parse()).collect())
        .collect();

    let mut lower_bound = 0;
    let mut upper_bound = 0;
    for row in input {
        let x = row[0];
        let y = row[1];
        let presents = &row[2..];

        let sum: i32 = presents.iter().cloned().sum();
        let sizes = [5, 7, 7, 7, 7, 6];
        let cells: i32 = (0..presents.len()).map(|i| presents[i] * sizes[i]).sum();
        let slots = (x / 3) * (y / 3);

        if slots >= sum {
            lower_bound += 1;
        }
        if cells <= x * y {
            upper_bound += 1;
        }
    }

    assert_eq!(587, lower_bound);
    assert_eq!(587, upper_bound);
}
