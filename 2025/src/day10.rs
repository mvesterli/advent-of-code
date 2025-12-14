#![cfg(test)]

use std::{collections::VecDeque, env::var};

fn parse_line(line: &str) -> (i32, Vec<Vec<usize>>, Vec<usize>) {
    let mut iter = line.split_ascii_whitespace().map(|s| &s[1..s.len() - 1]);
    let mut req = 0;
    for (idx, c) in iter.next().unwrap().chars().enumerate() {
        if c == '#' {
            req |= 1 << idx;
        }
    }
    let mut rows: Vec<_> = iter
        .map(|s| s.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();
    let jolt = rows.pop().unwrap();
    (req, rows, jolt)
}

#[test]
fn part1() {
    let input = include_str!("../input/day10.txt");
    let input: Vec<_> = input.lines().map(parse_line).collect();
    let mut sum = 0;
    for (req, buttons, _) in input {
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        q.push_back((0, 0));
        sum += loop {
            let (state, dist) = q.pop_front().unwrap();
            if state == req {
                break dist;
            }
            for button in &buttons {
                let mut new_state = state;
                for i in button {
                    new_state ^= 1 << i;
                }
                q.push_back((new_state, dist + 1));
            }
        };
    }
    assert_eq!(571, sum);
}

#[derive(Clone, Debug)]
struct Equation {
    vars: Vec<f64>,
    constant: f64,
}

impl Equation {
    fn create_all(buttons: &[Vec<usize>], req: &[usize]) -> Vec<Self> {
        let mut equations: Vec<Equation> = req
            .iter()
            .map(|r| Equation {
                vars: vec![0.0; buttons.len()],
                constant: -(*r as f64),
            })
            .collect();
        for i in 0..buttons.len() {
            for j in &buttons[i] {
                equations[*j].vars[i] += 1.0;
            }
        }
        equations
    }

    fn first_var(&self) -> Option<usize> {
        for i in 0..self.vars.len() {
            if self.vars[i].abs() > 0.1 {
                return Some(i);
            }
        }
        None
    }

    fn isolate(&self, var: usize) -> Self {
        let mut eq = self.clone();
        let factor = eq.vars[var];
        eq.vars[var] = 0.0;
        eq.constant = -eq.constant / factor;
        for v in &mut eq.vars {
            *v = -*v / factor;
        }
        eq
    }

    fn substitute(&mut self, idx: usize, eq: &Equation) {
        let factor = self.vars[idx];
        self.vars[idx] = 0.0;
        self.constant += factor * eq.constant;
        for i in 0..self.vars.len() {
            self.vars[i] += factor * eq.vars[i];
        }
    }

    fn evaluate(&self, assignments: &[f64]) -> f64 {
        (0..assignments.len())
            .map(|i| assignments[i] * self.vars[i])
            .sum::<f64>()
            + self.constant
    }
}

fn create_substitutions(equations: &[Equation]) -> Vec<(usize, Equation)> {
    let mut res = Vec::new();

    let mut sub_eqs = equations.to_owned();
    for eq_idx in 0..sub_eqs.len() {
        if let Some(idx) = sub_eqs[eq_idx].first_var() {
            let sub = sub_eqs[eq_idx].isolate(idx);
            for eq in &mut sub_eqs[eq_idx + 1..] {
                eq.substitute(idx, &sub);
            }
            res.push((idx, sub));
        }
    }
    res
}

#[test]
fn part2() {
    let input = include_str!("../input/day10.txt");
    let input: Vec<_> = input.lines().map(parse_line).collect();
    let mut sum = 0;
    let mut idx = 0;
    for (_, buttons, req) in input {
        println!("Running line {idx}");
        idx += 1;

        let equations = Equation::create_all(&buttons, &req);
        let substitutions = create_substitutions(&equations);

        let mut max = vec![999999999; buttons.len()];
        for eq in &equations {
            for i in 0..eq.vars.len() {
                if eq.vars[i] != 0.0 {
                    max[i] = max[i].min(eq.constant.abs().round() as i64);
                }
            }
        }
        let searched: Vec<_> = (0..buttons.len())
            .filter(|i| substitutions.iter().all(|p| p.0 != *i))
            .map(|i| (i, max[i]))
            .collect();
        println!("To search: {:?}", searched);

        fn search(
            to_search: &[(usize, i64)],
            equations: &[Equation],
            substitutions: &[(usize, Equation)],
            assignments: &mut [f64],
        ) -> Option<f64> {
            if to_search.is_empty() {
                for (idx, eq) in substitutions.iter().rev() {
                    assignments[*idx] = eq.evaluate(assignments).round();
                }
                if assignments.iter().any(|v| *v < 0.0) {
                    return None;
                }
                if equations
                    .iter()
                    .any(|e| e.evaluate(assignments).abs() > 1e-6)
                {
                    return None;
                }
                Some(assignments.iter().copied().sum())
            } else {
                let mut best = None;
                for i in 0..=to_search[0].1 {
                    assignments[to_search[0].0] = i as f64;
                    if let Some(r) = search(&to_search[1..], equations, substitutions, assignments)
                    {
                        if best.is_none() || r < best.unwrap() {
                            best = Some(r);
                        }
                    }
                }
                best
            }
        }

        let mut assignments = vec![0.0; buttons.len()];
        let res = search(&searched, &equations, &substitutions, &mut assignments).unwrap();
        sum += res.round() as i32;
    }
    assert_eq!(20869, sum);
}
