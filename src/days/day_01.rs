use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/01.txt");

    let (mut a, b): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| sscanf!(l, "{usize}   {usize}").unwrap())
        .unzip();

    let mut counts = HashMap::new();
    for x in b {
        *counts.entry(x).or_insert(0) += 1;
    }

    let result = a
        .iter()
        .map(|x| x * counts.get(x).copied().unwrap_or(0))
        .sum::<usize>();

    pv!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/01.txt");

    let (mut a, mut b): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| sscanf!(l, "{usize}   {usize}").unwrap())
        .unzip();

    a.sort_unstable();
    b.sort_unstable();

    let result = a.iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum::<usize>();

    pv!(result);
}
