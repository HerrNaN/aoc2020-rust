mod days;
mod util;
mod advent;
use crate::advent::Day;
use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("../cli.yaml");
    let matches = App::from(yaml).get_matches();

    if let Some(s) = matches.value_of("DAY") {
        match s.parse::<Day>() {
            Ok(d) => run_day(d),
            Err(e) => println!("Couldn't parse DAY as a Day")
        }
    }
}

fn run_day(d: Day) {
    
}