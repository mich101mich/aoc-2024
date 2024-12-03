use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");

    let mul_re = Regex::new(r"mul\(\d{1,3}\,\d{1,3}\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut dos = do_re.find_iter(input).map(|m| m.start()).peekable();
    let mut donts = dont_re.find_iter(input).map(|m| m.start()).peekable();
    let mut last_do = None;
    let mut last_dont = None;

    let result = mul_re
        .find_iter(input)
        .filter(|m| {
            let start = m.start();
            while let Some(next_do) = dos.next_if(|&d| d < start) {
                last_do = Some(next_do);
            }
            while let Some(next_dont) = donts.next_if(|&d| d < start) {
                last_dont = Some(next_dont);
            }
            match (last_do, last_dont) {
                (Some(do_), Some(dont_)) => do_ > dont_,
                (Some(do_), None) => true,
                (None, Some(dont_)) => false,
                _ => true,
            }
        })
        .map(|m| sscanf!(m.as_str(), "mul({usize},{usize})").unwrap())
        .map(|(a, b)| a * b)
        .sum::<usize>();

    pv!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/03.txt");

    let re = Regex::new(r"mul\(\d{1,3}\,\d{1,3}\)").unwrap();
    let result = re
        .find_iter(input)
        .map(|m| sscanf!(m.as_str(), "mul({usize},{usize})").unwrap())
        .map(|(a, b)| a * b)
        .sum::<usize>();

    pv!(result);
}
