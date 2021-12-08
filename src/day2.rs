pub fn dive(input: String) {
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
