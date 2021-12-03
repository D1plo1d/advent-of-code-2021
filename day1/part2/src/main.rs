use itertools::Itertools;

fn main() -> eyre::Result<()> {
    let root = std::env::current_exe()?.ancestors().skip(4).next().unwrap().to_owned();

    let count = std::fs::read_to_string(root.join("depths"))?.lines()
        .filter_map(|line| line.parse::<u32>().ok())
        .tuple_windows()
        .map(|(a, b, c)| [a, b, c].iter().sum::<u32>())
        .tuple_windows()
        .filter(|(a, b)| a < b)
        .count();

    println!("The depth increased {} times", count);

    Ok(())
}
