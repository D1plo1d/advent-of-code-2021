use itertools::Itertools;

fn main() -> eyre::Result<()> {
    let root = std::env::current_exe()?.ancestors().skip(4).next().unwrap().to_owned();

    let input = std::fs::read_to_string(root.join("input"))?;

    let mut aim = 0;
    let mut axes = [0, 0];

    for (cmd, val) in input.lines().map(|line| line.split(" ").collect_tuple().unwrap()) {
        let val = (((cmd != "up") as i32) * 2 - 1) * val.parse::<i32>()?;

        if cmd == "forward" {
            axes = [axes[0] + val, axes[1] + val * aim]
        } else {
            aim += val;
        }
    }

    println!("The horizontal times vertical movement is {}", axes[0] * axes[1]);

    Ok(())
}
