use aoc2021::{parser, Vec2};

#[derive(Default)]
struct Submarine {
    pub pos: Vec2,
    pub aim: i64,
}

fn main() {
    let s = std::fs::read_to_string("input/input02.txt").unwrap();
    let commands = parser::submarine_command_list(&s).unwrap().1;
    println!("{:?}", commands);
    let pos = commands.iter().fold(Vec2::default(), |mut a, c| {
        match c {
            parser::SubmarineCommand::Up(i) => a.y -= i,
            parser::SubmarineCommand::Down(i) => a.y += i,
            parser::SubmarineCommand::Forward(i) => a.x += i,
        };
        a
    });

    println!("res: {}", pos.x * pos.y);

    let submarine = commands
        .iter()
        .fold(Submarine::default(), |mut submarine, c| {
            match c {
                parser::SubmarineCommand::Up(i) => submarine.aim -= i,
                parser::SubmarineCommand::Down(i) => submarine.aim += i,
                parser::SubmarineCommand::Forward(i) => {
                    submarine.pos.x += i;
                    submarine.pos.y += submarine.aim * i;
                }
            };
            submarine
        });
    println!("res2: {}", submarine.pos.x * submarine.pos.y);
}
