pub fn sonar_sweep(input: String) {
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
