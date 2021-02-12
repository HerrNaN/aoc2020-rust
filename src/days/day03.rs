use crate::advent;

type Area = Vec<Vec<u8>>;

const TREE: u8 = b'#';

pub const DAY03A: advent::Sol<Area, i32> = advent::Sol {
    day: 3,
    part: advent::Part::Part1,
    parse: parse,
    solve: solve1,
    show: i32::to_string,
};

pub const DAY03B: advent::Sol<Area, i64> = advent::Sol {
    day: 3,
    part: advent::Part::Part2,
    parse: parse,
    solve: solve2,
    show: i64::to_string,
};

fn solve2(area: Area) -> i64 {
    vec![(1,1), (3,1), (5,1), (7,1), (1,2)].into_iter()
        .map(|(r, d)| run_slope(&area, r, d) as i64).product()
}

fn solve1(area: Area) -> i32 {
    run_slope(&area, 3, 1)
}

fn run_slope(area: &Area, right: i32, down: i32) -> i32 {
    let width = area[0].len() as i32;
    let mut pos: (i32, i32) = (0,0);
    let mut c = 0;
    while pos.0 < area.len() as i32 {
        if area[pos.0 as usize][pos.1 as usize] == TREE {
            c = c+1;
        }
        pos = (pos.0 + down, (pos.1 + right) % width );
    }
    return c;
}

fn parse(input: String) -> Area {
    input.lines()
        .map(|l| parse_line(l)).collect()
}

fn parse_line(line: &str) -> Vec<u8> {
    line.bytes().collect()
}