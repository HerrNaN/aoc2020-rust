pub type Day = i32;

#[derive(Hash, Eq, PartialEq, Clone)]
pub enum Part {
    Part1,
    Part2,
}

impl Part {
    fn show(&self) -> String {
        match self {
            Part::Part1 => String::from("Part 1"),
            Part::Part2 => String::from("Part 2")
        }
    }
}
#[derive(Clone)]
pub struct Sol<I,O> {
    pub day:  Day,
    pub part: Part,

    pub parse: fn(String) -> I,
    pub solve: fn(I) -> O,
    pub show:  fn(O) -> String,
}

pub trait Solution {
    fn print_day_and_part(&self);
    fn run(&self, input: String) -> String;
}


impl<I: Clone,O: Clone> Solution for Sol<I,O> {
    fn print_day_and_part(&self) {
        println!("--- Day {day}: {part} ---", day=self.day, part=self.part.show());
    }

    fn run(&self, input: String) -> String {
        return (self.show)((self.solve)((self.parse)(input)));
    }
}
