use itertools::Itertools;

fn main() -> eyre::Result<()> {
    let root = std::env::current_exe()?.ancestors().skip(3).next().unwrap().to_owned();

    let count = std::fs::read_to_string(root.join("input"))?.lines()
        .filter_map(|line| line.parse::<u32>().ok())
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count();

    println!("The depth increased {} times", count);

    Ok(())
}
