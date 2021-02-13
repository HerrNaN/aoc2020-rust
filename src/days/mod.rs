use crate::Day;
use crate::advent::Part;
use crate::advent::Solution;
use crate::advent::Part::Part1;
use crate::advent::Part::Part2;
use std::collections::HashMap;
use std::boxed::Box;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

pub fn day_map() -> HashMap<Day,HashMap<Part,Box<dyn Solution>>> {
    vec![
        (1, vec![(Part1, Box::new(day01::DAY01A) as Box<dyn Solution>),
                 (Part2, Box::new(day01::DAY01B) as Box<dyn Solution>)
                 ].into_iter().collect()),
        (2, vec![(Part1, Box::new(day02::DAY02A) as Box<dyn Solution>),
                 (Part2, Box::new(day02::DAY02B) as Box<dyn Solution>)
                 ].into_iter().collect()),
        (3, vec![(Part1, Box::new(day03::DAY03A) as Box<dyn Solution>),
                 (Part2, Box::new(day03::DAY03B) as Box<dyn Solution>)
                 ].into_iter().collect()),
        (4, vec![(Part1, Box::new(day04::DAY04A) as Box<dyn Solution>),
                 (Part2, Box::new(day04::DAY04B) as Box<dyn Solution>)
                 ].into_iter().collect()),
        (5, vec![(Part1, Box::new(day05::DAY05A) as Box<dyn Solution>),
                 (Part2, Box::new(day05::DAY05B) as Box<dyn Solution>)
                 ].into_iter().collect()),
        ].into_iter().collect()
}
