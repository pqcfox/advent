fn find_air_code(codes: &Vec<&str>, is_o2: bool) -> String {
    let n = codes[0].len();
    let mut bits = String::from("");
    for i in 0..n {
        let valids: Vec<&str> = codes
            .iter()
            .filter(|c| &c[..i] == bits)
            .cloned()
            .collect();

        if valids.len() == 1 {
            bits = valids[0].to_string();
            break;
        } 

        let one_count = valids
            .iter() 
            .filter(|c| &c[i..i+1] == "1")
            .count();
        let is_one_maj = 2 * one_count >= valids.len();
        let is_one = if is_o2 {is_one_maj} else {!is_one_maj};
        bits.push(if is_one {'1'} else {'0'});
    }
    return bits;
}

pub fn binary_diagnostic(input: String) {
    let codes: Vec<&str> = input.lines().collect();
    let n = codes[0].len();

    // PART 1
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for i in 0..n {
        let one_count = codes
            .iter()
            .filter(|c| c.chars().nth(i).unwrap() == '1')
            .count();
        if 2 * one_count >= codes.len() {
            gamma += 1 << (n - i - 1);
        } else {
            epsilon += 1 << (n - i - 1);
        }
    }

    println!("Submarine is consuming {} power.", gamma * epsilon);

    // PART 2
    let o2_bits = find_air_code(&codes, true);
    let co2_bits = find_air_code(&codes, false);
    let o2 = usize::from_str_radix(&o2_bits, 2).unwrap();
    let co2 = usize::from_str_radix(&co2_bits, 2).unwrap();

    println!("Submarine life support is at {:?}.", o2 * co2);
}
