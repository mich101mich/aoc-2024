use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");
    // let input = "";

    let stones = input.split_whitespace().map(parse_u).to_vec();

    let mut cutoffs = vec![];
    {
        let mut x = 1;
        for i in 0..10 {
            cutoffs.push(x);
            x *= 10;
        }
    }

    let mut stones = stones.into_iter().map(|s| (s, 1)).to_map();
    let mut next_stones = HashMap::new();
    for i in 0..75 {
        if i % 5 == 0 {
            pv!(i, stones.len());
        }
        for (s, count) in stones.drain() {
            if s == 0 {
                *next_stones.entry(1).or_insert(0) += count;
                continue;
            }
            let num_digits = s.ilog10() + 1;
            if num_digits % 2 == 0 {
                let cutoff = cutoffs[num_digits as usize / 2];
                *next_stones.entry(s / cutoff).or_insert(0) += count;
                *next_stones.entry(s % cutoff).or_insert(0) += count;
            } else {
                *next_stones.entry(s * 2024).or_insert(0) += count;
            }
        }
        std::mem::swap(&mut stones, &mut next_stones);
        next_stones.clear();
    }

    let result = stones.values().sum::<usize>();
    result!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/11.txt");

    let mut stones = input.split_whitespace().map(parse_u).to_vec();
    let mut next_stones = vec![];

    for _ in 0..25 {
        for &s in &stones {
            if s == 0 {
                next_stones.push(1);
                continue;
            }
            let num_digits = s.ilog10() + 1;
            if num_digits % 2 == 0 {
                let cutoff = 10usize.pow(num_digits / 2);
                next_stones.push(s / cutoff);
                next_stones.push(s % cutoff);
            } else {
                next_stones.push(s * 2024);
            }
        }
        std::mem::swap(&mut stones, &mut next_stones);
        next_stones.clear();
    }

    result!(stones.len());
}
