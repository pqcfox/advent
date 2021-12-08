use std::env;
use std::fs;
mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: usize = args.get(1)
        .expect("Too few arguments")
        .parse()
        .expect("Can't parse day number");

    let input_name = format!("inputs/day{}.txt", day);
    let input = fs::read_to_string(input_name).expect("Couldn't read file");

    let fns: Vec<&dyn Fn(String)> = vec![
        &day1::sonar_sweep,
        &day2::dive,
        &day3::binary_diagnostic,
        &day4::giant_squid
    ];

    let day_fn = fns.get(day - 1).expect("Invalid day");
    day_fn(input);
}
