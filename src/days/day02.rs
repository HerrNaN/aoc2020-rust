use crate::advent;

type Password = String;
type Policy = (i32, i32, char);

pub const DAY02A: advent::Sol<Vec<(Policy, Password)>, i32> = advent::Sol {
    day: 2,
    part: advent::Part::Part1,
    parse: parse,
    solve: solve1,
    show: |x| x.to_string(),
};

pub const DAY02B: advent::Sol<Vec<(Policy, Password)>, i32> = advent::Sol {
    day: 2,
    part: advent::Part::Part2,
    parse: parse,
    solve: solve2,
    show: |x| x.to_string(),
};

fn solve2(entries: Vec<(Policy, Password)>) -> i32 {
    entries.into_iter().filter(|(pol, pass)| is_valid2(*pol, pass.to_string())).count() as i32
}

fn is_valid2((a,b,c): Policy, pass: Password) -> bool {
    pass.as_bytes()[(a-1) as usize] as char == c && pass.as_bytes()[(b-1) as usize] as char != c 
        || pass.as_bytes()[(a-1) as usize] as char != c && pass.as_bytes()[(b-1) as usize] as char == c
}

fn solve1(entries: Vec<(Policy, Password)>) -> i32 {
    entries.into_iter().map(|(pol, pass)| if is_valid1(pol, pass) {1} else {0}).sum()
}

fn is_valid1((from, to, c): Policy, pass: Password) -> bool {
    let count = pass.chars().filter(|a| *a == c).count() as i32;
    return count >= from && count <= to
}

fn parse(input: String) -> Vec<(Policy, Password)> {
    input.lines()
        .map(|l| parse_line(l)).collect()
}

fn parse_line(line: &str) -> (Policy, Password) {
    let (a, b, c, s) = scan_fmt!(line, "{d}-{d} {[a-z]}: {}", i32, i32, char, String).unwrap();
    return ((a,b,c), s);
}