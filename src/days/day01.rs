use crate::advent;
use std::collections::HashSet;


pub const DAY01A: advent::Sol<Vec<i32>, i32> = advent::Sol {
    day: 1,
    part: advent::Part::Part1,
    parse: parse1,
    solve: solve1,
    show: |x| x.to_string(),
};

pub const DAY01B: advent::Sol<Vec<i32>, i32> = advent::Sol {
    day: 1,
    part: advent::Part::Part2,
    parse: parse1,
    solve: solve2,
    show: |x| x.to_string(),
};

fn parse1(s: String) -> Vec<i32> {
    s.lines()
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

fn solve1(is: Vec<i32>) -> i32 {
    let s: HashSet<&i32> = is.iter().collect();
    for i in is.iter() {
        let j = 2020-i;
        if s.contains(&j) {
            return i * j
        }
    }
    return -1;
}

fn solve2(is: Vec<i32>) -> i32 {
    let s: HashSet<&i32> = is.iter().collect();
    for i in is.iter() {
        for j in is.iter() {
            let k = 2020-i-j;
            if s.contains(&k) {
                return i * j * k;
            }
        }
    }
    return -1;
}