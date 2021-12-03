fn main() -> eyre::Result<()> {
    let mut root = std::env::current_exe()?;

    for _ in 0..3 {
        root.pop();
    }

    let file = std::fs::read_to_string(dbg!(root.join("input")))?;

    let mut prev_depth = 0;
    let mut count = 0;

    for line in file.lines().filter(|line| line.len() != 0) {
        let depth: u32 = line.parse()?;
        if prev_depth > depth {
            count += 1;
        }
        prev_depth = depth;
    }

    println!("The depth increased {} times", count);

    Ok(())
}
