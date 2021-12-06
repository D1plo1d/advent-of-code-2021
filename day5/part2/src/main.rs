use std::collections::BTreeMap;
use nom::character::complete as nc;
use nom::bytes::complete as nb;
use nom::sequence::separated_pair;

fn main() -> eyre::Result<()> {
    let root = std::env::current_exe()?.ancestors().skip(4).next().unwrap().to_owned();

    let input = std::fs::read_to_string(root.join("input"))?;

    // Parsing
    let i32_pair = || separated_pair::<_, _, _, _, nom::error::Error<&str>, _, _, _>(
        nc::i32,
        nb::tag(","),
        nc::i32,
    );

    let locations: BTreeMap<(i32, i32), i32> = input.lines()
        .map(|line| {
            let (input, vent) = separated_pair(i32_pair(), nb::tag(" -> "), i32_pair())(line).unwrap();
            if input.len() > 0 {
                panic!("Unparsed input: {:?}", input);
            }
            vent
        })
        .fold(BTreeMap::new(), |mut locations, vent| {
            let mut x = vent.0.0;
            let mut y = vent.0.1;

            let max_x = vent.1.0;
            let max_y = vent.1.1;

            let x_inc = (max_x - x).signum();
            let y_inc = (max_y - y).signum();
            // dbg!((&x, &max_x, &x_inc));
            // dbg!((&y, &max_y, &y_inc));

            loop {
                *locations.entry((x, y)).or_insert(0) += 1;
                // dbg!((&x, &max_x, &x_inc));
                // dbg!((&y, &max_y, &y_inc));

                if x == max_x && y == max_y {
                    break;
                }

                if x != max_x {
                    x += x_inc;
                }
                if y != max_y {
                    y += y_inc;
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
