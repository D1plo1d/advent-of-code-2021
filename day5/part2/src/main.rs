use std::collections::BTreeMap;
use nom::character::complete as nc;
use nom::bytes::complete as nb;
use nom::sequence::separated_pair;

fn main() -> eyre::Result<()> {
    let root = std::env::current_exe()?.ancestors().skip(4).next().unwrap().to_owned();

    let input = std::fs::read_to_string(root.join("input"))?;

    // Parsing
    let u32_pair = || separated_pair::<_, _, _, _, nom::error::Error<&str>, _, _, _>(
        nc::u32,
        nb::tag(","),
        nc::u32,
    );

    let locations: BTreeMap<(u32, u32), u32> = input.lines()
        .map(|line| {
            separated_pair(u32_pair(), nb::tag(" -> "), u32_pair())(line).unwrap().1
        })
        .fold(BTreeMap::new(), |mut locations, vent| {
            let mut x = vent.0.0.min(vent.1.0);
            let mut y = vent.0.1.min(vent.1.1);

            let max_x = vent.0.0.max(vent.1.0);
            let max_y = vent.0.1.max(vent.1.1);

            loop {
                *locations.entry((x, y)).or_insert(0) += 1;

                if x == max_x && y == max_y {
                    break;
                }

                if x < max_x {
                    x += 1;
                }
                if y < max_y {
                    y += 1;
                }
            }

            locations
        });

    dbg!(locations.len());

    let overlaps = locations.into_iter()
        .filter(|(_xy, count)| *count > 1)
        .count();

    println!("Overlaps {}", overlaps);

    Ok(())
}
