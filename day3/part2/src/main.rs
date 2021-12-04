fn main() -> eyre::Result<()> {
    let root = std::env::current_exe()?.ancestors().skip(4).next().unwrap().to_owned();

    let input = std::fs::read_to_string(root.join("input"))?;

    let mut bit_counts = [0i32; 12];

    input.lines().for_each(|line| line.char_indices().for_each(|(i, c)| {
        bit_counts[i] += ((c == '1') as i32) * 2 - 1
    }));

    let find = |criteria: fn(char, i32) -> bool| bit_counts.iter()
        .enumerate()
        .fold(input.lines().collect(), |acc: Vec<&str>, (i, count)| {
            let last = acc.last().unwrap().clone();
            let vals: Vec<_> = acc.into_iter()
                .filter(|s| criteria(s.chars().skip(i).next().unwrap(), *count))
                .collect();

            if vals.is_empty() { vec![last] } else { vals }
        })
        .pop()
        .and_then(|s| u64::from_str_radix(s, 2).ok())
        .unwrap();

    let o2 = find(|c, count| (count >= 0) == (c == '1'));
    let co2 = find(|c, count| (count >= 0) == (c == '0'));

    println!("The life support rating is {}", dbg!(o2) * dbg!(co2));

    Ok(())
}
