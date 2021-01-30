use std::io::{self, Read};
use std::fs::File;
use crate::advent::Day;

pub fn read_input(d: Day) -> io::Result<String> {
    let filename = match d {
        1..=9 => format!("inputs/0{}.txt", d),
        _    => format!("inputs/{}.txt",d),
    };
    let mut tmp = String::new();
    let mut file = File::open(filename)?;
    file.read_to_string(&mut tmp)?;
    Ok(tmp)
}