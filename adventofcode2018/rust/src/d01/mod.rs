use crate::utils::{StdResult};
use std::collections::HashSet;


fn parse(s: &str) -> StdResult<Vec<i32>> {
    s.split_whitespace().map(|num| {
        let num = if num.starts_with("+") {
            num.trim_start_matches("+")
        } else { num };
        let num = num.parse::<i32>()
            .map_err(|e| format!("Invalid i32 {:?}: {:?}", num, e))?;
        Ok(num)
    }).collect()
}


fn part_1(input: &[i32]) -> i32 {
    input.iter().sum()
}


fn part_2(input: &[i32]) -> i32 {
    let mut set = HashSet::new();
    let mut val = 0;
    for n in input.iter().cycle() {
        set.insert(val);
        val += n;
        if set.contains(&val) {
            return val
        }
    }
    unreachable!()
}


pub fn run() -> StdResult<()> {
    info!("*** Day 1 ***");
    let input = parse(input_file!("d01.txt"))?;

    let (ms1, p1) = time!({
        part_1(&input)
    });
    info!("p1: {}", p1);

    let (ms2, p2) = time!({
        part_2(&input)
    });
    info!("p2: {}", p2);

    info!("[Day 1 runtimes] p1: {}ms, p2: {}ms\n", ms1, ms2);
    Ok(())
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        [
            (vec![1, 1, 1], 3),
            (vec![1, 1, -2], 0),
            (vec![-1, -2, -3], -6),
        ].iter().for_each(|(input, expected)| {
            let res = part_1(input);
            assert_eq!(res, *expected);
        });
    }

    #[test]
    fn test_p2() {
        [
            (vec![1, -1], 0),
            (vec![3, 3, 4, -2, -4], 10),
            (vec![-6, 3, 8, 5, -6], 5),
            (vec![7, 7, -2, -7, -4], 14),
        ].iter().for_each(|(input, expected)| {
            let res = part_2(input);
            assert_eq!(res, *expected);
        });
    }
}
