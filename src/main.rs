mod days;
mod util;
mod advent;
use crate::util::input::read_input;
use crate::advent::Part;
use crate::advent::Part::Part2;
use crate::advent::Part::Part1;
use crate::advent::Day;
use clap::{App, load_yaml};

fn main() {
    let yaml = load_yaml!("../cli.yaml");
    let matches = App::from(yaml).get_matches();

    if let Some(ds) = matches.value_of("day"){
        match ds.parse::<i32>() {
            Ok(d) => {
                if let Some(ps) = matches.value_of("part") {
                    match ps.parse::<i32>() {
                        Ok(1) => run_part(d, Part1),
                        Ok(2) => run_part(d, Part2),
                        _ => println!("couldn't parse {} as a Part", ps)
                    }
                } else {
                    run_day(d)
                }
            },
            _ => println!("Couldn't parse {} as a Day", ds)
        }
    }
}

fn run_day(d: Day) {
    run_part(d, Part1);
    run_part(d, Part2);
}

fn run_part(d: Day, p: Part) {
    let map = days::day_map();
    let sol = map.get(&d).unwrap().get(&p).unwrap();
    (*sol).print_day_and_part();
    let input = read_input(d).unwrap();
    let ans = (*sol).run(input);
    println!("{}",ans)
}