fn main() -> eyre::Result<()> {
    let root = std::env::current_exe()?.ancestors().skip(4).next().unwrap().to_owned();

    let input = std::fs::read_to_string(root.join("input"))?;

    let mut bit_counts = [0i32; 12];

    input.lines().for_each(|line| dbg!(line).char_indices().for_each(|(i, c)| {
        bit_counts[i] += ((c == '1') as i32) * 2 - 1
    }));

    let find = |criteria: fn(bool, i32) -> bool| {
        let mut acc: Vec<&str> = input.lines().collect();
        for (i, count) in bit_counts.iter().enumerate() {
            let mut last = acc.last().unwrap().clone();

            acc = acc.into_iter()
                .filter(|s| criteria(s.chars().skip(i).next() == Some('1'), *count))
                .collect();

            if acc.len() == 1 {
                last = acc.first().unwrap();
            }

            if acc.len() <= 1 {
                return u64::from_str_radix(last, 2).unwrap()
            }
        };
        panic!("Single value not found, remaining: {:?}", acc);
    };

    let o2 = find(|b, ones_count| if ones_count >= 0 {b} else {!b});
    let co2 = find(|b, ones_count| if ones_count >= 0 {!b} else {b});

    println!("The life support rating is {}", dbg!(o2) * dbg!(co2));

    Ok(())
}
