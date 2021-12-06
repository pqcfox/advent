use std::env;
use std::fs;

fn sonar_sweep(input: String) {
    let depths: Vec<u32> = input.lines()
        .map(|d| d.parse().unwrap())
        .collect();

    // PART 1
    let downs = depths.iter()
        .zip(depths.iter().skip(1))
        .filter(|(d0, d1)| d1 > d0)
        .count();
    println!("Depth increased {} times.", downs);

    // PART 2
    let sums: Vec<u32> = depths.iter()
        .zip(depths.iter().skip(1))
        .zip(depths.iter().skip(2))
        .map(|((d0, d1), d2)| d0 + d1 + d2)
        .collect();
    let sum_downs = sums.iter()
        .zip(sums.iter().skip(1))
        .filter(|(s0, s1)| s1 > s0)
        .count();
    println!("Window sum increased {} times.", sum_downs);
}

fn dive(input: String) {
    let moves: Vec<(&str, i32)> = input.lines()
        .map(|c| c.split(" ").collect::<Vec<&str>>())
        .map(|p| (p[0], p[1].parse().unwrap()))
        .collect();

    // PART 1
    let end_pos = moves.iter().map(|&(a, d)| match a {
            "forward" => (d, 0),
            "up" => (0, -d),
            "down" => (0, d),
            _ => (0, 0)       
        }).fold((0, 0), |pos, m| (pos.0 + m.0, pos.1 + m.1));
    println!("Product of final horizontal position by depth is {:?}.",
        end_pos.0 * end_pos.1); 

    // PART 2
    let true_pos = moves.iter().fold(((0, 0), 0), |(pos, aim), &(a, d)| match a {
        "forward" => ((pos.0 + d, pos.1 + d * aim), aim),
        "up" => (pos, aim - d),
        "down" => (pos, aim + d),
        _ => (pos, aim)
    }).0;
    println!("Product of true final horizontal position by depth is {:?}.",
        true_pos.0 * true_pos.1); 
}

fn binary_diagnostic(input: String) {

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let day: usize = args.get(1)
        .expect("Too few arguments")
        .parse()
        .expect("Can't parse day number");

    let input_name = format!("inputs/day{}.txt", day);
    let input = fs::read_to_string(input_name).expect("Couldn't read file");

    let fns: Vec<&dyn Fn(String)> = vec![
        &sonar_sweep,
        &dive,
        &binary_diagnostic
    ];

    let day_fn = fns.get(day - 1).expect("Invalid day");
    day_fn(input);
}
