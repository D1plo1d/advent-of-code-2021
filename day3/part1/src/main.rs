fn main() -> eyre::Result<()> {
    let root = std::env::current_exe()?.ancestors().skip(4).next().unwrap().to_owned();

    let input = std::fs::read_to_string(root.join("input"))?;

    let mut bit_counts = [0i32; 12];

    input.lines().for_each(|line| line.char_indices().for_each(|(i, c)| {
        bit_counts[i] += ((c == '1') as i32) * 2 - 1
    }));

    let gamma = bit_counts.iter().enumerate()
        .fold(0u64, |gamma, (i, count)| gamma + (((*count > 0) as u64) << (11 - i)));

    println!("The power is {}", dbg!(gamma) * (gamma ^ 0b1111_1111_1111));

    Ok(())
}
