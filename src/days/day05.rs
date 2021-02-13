use crate::advent;

pub const DAY05A: advent::Sol<Vec<Seat>, i32> = advent::Sol {
    day: 5,
    part: advent::Part::Part1,
    parse: parse,
    solve: solve1,
    show: i32::to_string,
};

pub const DAY05B: advent::Sol<Vec<Seat>, i32> = advent::Sol {
    day: 5,
    part: advent::Part::Part2,
    parse: parse,
    solve: solve2,
    show: i32::to_string,
};

type Seat = String;

fn solve2(seats: Vec<Seat>) -> i32 {
    let mut ids: Vec<i32> = seats.into_iter().map(seat_id).collect();
    ids.sort();
    for i in 0..ids.len() {
        if ids[i+1] != ids[i] + 1 {
            return ids[i] + 1;
        }
    }
    return -1;
}


fn solve1(seats: Vec<Seat>) -> i32 {
    match seats.into_iter().map(seat_id).max() {
        Some(max) => max,
        None => -1
    }
}

fn seat_id(seat: Seat) -> i32 {
    let bin: String = seat.chars().map(|c| match c {
        'B'|'R' => '1',
        'F'|'L' => '0',
        _ => c
    }).collect();
    isize::from_str_radix(bin.as_str(), 2).unwrap() as i32
}

fn parse(input: String) -> Vec<Seat> {
    input.trim_end().lines().map(str::to_string).collect()
}