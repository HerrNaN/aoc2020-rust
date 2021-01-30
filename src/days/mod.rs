use crate::Day;
use crate::advent::Part;
use crate::advent::Solution;
use crate::advent::Part::Part1;
use crate::advent::Part::Part2;
use std::collections::HashMap;
use std::boxed::Box;

pub mod day01;

pub fn day_map() -> HashMap<Day,HashMap<Part,Box<dyn Solution>>> {
    vec![
        (1 as Day, 
            vec![  
                (Part1, day01::DAY01A),
                (Part2, day01::DAY01A),
            ].into_iter().collect()
        ),
    ].into_iter().collect()
}
