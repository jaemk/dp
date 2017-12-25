/*!
http://adventofcode.com/2017/day/24
*/
use std::collections::HashSet;


static INPUT: &'static str = include_str!("../input.txt");
static TEST_INPUT: &'static str = r##"
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10
"##;


type Error = Box<std::error::Error>;
type Result<T> = std::result::Result<T, Error>;


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Comp {
    id: usize,
    ports: [u64; 2],
}
impl Comp {
    fn strength(&self) -> u64 {
        self.ports.iter().sum()
    }

    fn has_zero(&self) -> bool {
        self.ports.iter().any(|&p| p == 0)
    }

    fn has_port(&self, port: u64) -> bool {
        self.ports.iter().any(|&p| p == port)
    }

    fn other_port_value(&self, port: u64) -> u64 {
        if self.ports.iter().all(|&p| p == port) {
            return port
        }
        if self.ports[0] == port { self.ports[1] }
        else { self.ports[0] }
    }
}


struct Components {
    inner: Vec<Comp>,
}
impl std::str::FromStr for Components {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let inner = s.trim().lines().enumerate().map(|(n, line)| {
            let mut parts = line.trim().split("/");
            let a = parts.next().ok_or_else(|| "Missing port count")?.parse::<u64>()?;
            let b = parts.next().ok_or_else(|| "Missing port count")?.parse::<u64>()?;
            Ok(Comp { id: n, ports: [a, b] })
        }).collect::<Result<Vec<Comp>>>()?;
        Ok(Self { inner })
    }
}
impl std::ops::Deref for Components {
    type Target = Vec<Comp>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Components {
    fn from_vec(inner: Vec<Comp>) -> Self {
        Self { inner }
    }

    /// Returns (zeros, others)
    fn split(self) -> (Self, Self) {
        let mut zeros = vec![];
        let mut others = vec![];
        for comp in self.inner.into_iter() {
            if comp.has_zero() {
                zeros.push(comp);
            } else {
                others.push(comp);
            }
        }
        (Self::from_vec(zeros), Self::from_vec(others))
    }
}


mod part1 {
    use super::*;

    fn walk(mut max: u64, strength: u64, used: &mut HashSet<Comp>, base: &Comp, port_in_use: u64, comps: &Components) -> u64 {
        used.insert(base.clone());
        let port = base.other_port_value(port_in_use);
        let comp_strength = base.strength();
        let possibles = comps.iter()
            .filter(|comp| ! used.contains(comp))
            .filter(|comp| comp.has_port(port)).collect::<Vec<_>>();
        let cur_strength = strength + comp_strength;
        if possibles.len() == 0 {
            if cur_strength > max {
                max = cur_strength;
            }
        } else {
            for comp in &possibles {
                max = walk(max, cur_strength, used, comp, port, comps);
            }
        }
        used.remove(base);
        max
    }

    pub fn solve(input: &str) -> Result<u64> {
        let comps = input.parse::<Components>()?;
        let (zeros, comps) = comps.split();

        let mut max = 0;
        for base in zeros.iter() {
            let port = base.other_port_value(0);
            let possibles = comps.iter().filter(|comp| comp.has_port(port)).collect::<Vec<_>>();
            let mut used: HashSet<Comp> = HashSet::new();
            for comp in &possibles {
                let branch_max = walk(port, port, &mut used, comp, port, &comps);
                if branch_max > max { max = branch_max; }
            }
        }
        Ok(max)
    }
}


mod part2 {
    use super::*;

    fn walk(mut max_strength: u64, strength: u64, mut max_length: usize, length: usize,
            used: &mut HashSet<Comp>, base: &Comp, port_in_use: u64, comps: &Components) -> (u64, usize) {
        used.insert(base.clone());
        let comp_strength = base.strength();
        let cur_strength = strength + comp_strength;
        let cur_length = length + 1;
        let port = base.other_port_value(port_in_use);
        let possibles = comps.iter()
            .filter(|comp| ! used.contains(comp))
            .filter(|comp| comp.has_port(port)).collect::<Vec<_>>();
        if possibles.len() == 0 {
            if cur_length > max_length {
                max_strength = cur_strength;
                max_length = cur_length;
            } else if cur_length == max_length {
                if cur_strength > max_strength {
                    max_strength = cur_strength;
                }
            }
        } else {
            for comp in &possibles {
                let (_max_strength, _max_length) = walk(max_strength, cur_strength,
                                                        max_length, cur_length,
                                                        used, comp, port, comps);
                max_strength = _max_strength;
                max_length = _max_length;
            }
        }
        used.remove(base);
        (max_strength, max_length)
    }

    pub fn solve(input: &str) -> Result<u64> {
        let comps = input.parse::<Components>()?;
        let (zeros, comps) = comps.split();

        let mut max_strength = 0;
        let mut max_length = 0;
        for base in zeros.iter() {
            let port = base.other_port_value(0);
            let possibles = comps.iter().filter(|comp| comp.has_port(port)).collect::<Vec<_>>();
            let mut used: HashSet<Comp> = HashSet::new();
            for comp in &possibles {
                let (branch_max_len_str, branch_max_len) = walk(port, port,
                                                                1, 1,
                                                                &mut used, comp, port, &comps);
                if branch_max_len > max_length {
                    max_strength = branch_max_len_str;
                    max_length = branch_max_len;
                } else if branch_max_len == max_length {
                    if branch_max_len_str > max_strength {
                        max_strength = branch_max_len_str;
                    }
                }
            }
        }
        Ok(max_strength)
    }
}


fn time<T, F: Fn() -> Result<T>>(f: F) -> Result<(f64, T)> {
    use std::time;
    let start = time::Instant::now();
    let res = f()?;
    let elap = start.elapsed();
    let ms = elap.as_secs() as f64 * 1_000. + elap.subsec_nanos() as f64 / 1_000_000.;
    let ms = (ms * 10_000.).round() / 10_000.;
    Ok((ms, res))
}


fn run() -> Result<()> {
    let (ms, t1) = time(|| Ok(part1::solve(TEST_INPUT)?))?;
    println!("d24-t1: [{:>12}ms] {} == 31", ms, t1);

    let (ms, t2) = time(|| Ok(part2::solve(TEST_INPUT)?))?;
    println!("d24-t2: [{:>12}ms] {} == 19", ms, t2);

    let (ms, p1) = time(|| Ok(part1::solve(INPUT)?))?;
    println!("d24-p1: [{:>12}ms] {}", ms, p1);

    let (ms, p2) = time(|| Ok(part2::solve(INPUT)?))?;
    println!("d24-p2: [{:>12}ms] {}", ms, p2);
    Ok(())
}


pub fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let ans = part1::solve(TEST_INPUT).expect("p1 failed");
        assert_eq!(ans, 31, "expected strength: 31");
    }

    #[test]
    fn p2() {
        let ans = part2::solve(TEST_INPUT).expect("p2 failed");
        assert_eq!(ans, 19, "expected strength: 19");
    }
}

