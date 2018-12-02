use super::utils::{self, StdResult};


fn part_1(input: &str) -> StdResult<u32> {
    let mut twos = 0;
    let mut threes = 0;
    for line in input.lines() {
        let freqs = utils::freqs(line);

        let mut has2 = false;
        let mut has3 = false;
        for (_, &v) in freqs.iter() {
            if v == 2 { has2 = true; }
            else if v == 3 { has3 = true; }

            if has2 && has3 { break; }
        }
        if has2 { twos += 1; }
        if has3 { threes += 1; }
    }
    Ok(twos * threes)
}


fn positional_diff(a: &[char], b: &[char]) -> u32 {
    a.iter().zip(b.iter()).fold(0, |acc, (a, b)| {
        acc + if a != b { 1 } else { 0 }
    })
}

fn part_2(input: &str) -> StdResult<String> {
    let rows = input.lines().map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    for (i, row) in rows.iter().enumerate() {
        for other in &rows[i..] {
            if positional_diff(row, other) == 1 {
                let res = row.iter().zip(other.iter()).map(|(a, b)| {
                    if a == b { Some(a) } else { None }
                }).filter_map(|a| a).collect::<String>();
                return Ok(res)
            }
        }
    }
    unreachable!()
}

pub fn run() -> StdResult<()> {
    info!("* Day 2 *");

    let input = utils::load_file("../input/d02.txt")?;

    let res = part_1(&input)?;
    info!("p1: {:?}", res);

    let res = part_2(&input)?;
    info!("p2: {:?}", res);
    Ok(())
}
