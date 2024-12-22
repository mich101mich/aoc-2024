use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");

    // // initial implementation

    // const PRUNE_LIMIT: isize = 16777216;
    // const PRUNE_MASK: isize = PRUNE_LIMIT - 1;
    // fn process(mut x: isize) -> isize {
    //     x = (x ^ (x << 6)) & PRUNE_MASK;
    //     x = (x ^ (x >> 5)) & PRUNE_MASK;
    //     x = (x ^ (x << 11)) & PRUNE_MASK;
    //     x
    // }
    // fn process_get_diff_10(x: &mut isize) -> isize {
    //     let next = process(*x);
    //     let diff = (next % 10) - (*x % 10);
    //     *x = next;
    //     diff
    // }

    // let mut sequences = HashMap::new();
    // let mut seen = HashSet::new();
    // for mut x in input.lines().map(parse) {
    //     seen.clear();
    //     let mut sequence = [0; 4];
    //     for i in 0..4 {
    //         sequence[i] = process_get_diff_10(&mut x);
    //     }
    //     seen.insert(sequence);
    //     *sequences.entry(sequence).or_insert(0) += x % 10;
    //     for _ in 4..2000 {
    //         sequence.rotate_left(1);
    //         sequence[3] = process_get_diff_10(&mut x);
    //         if seen.insert(sequence) {
    //             *sequences.entry(sequence).or_insert(0) += x % 10;
    //         }
    //     }
    // }

    // sequences.retain(|k, _| k[0] != 99);

    // let result = sequences.values().max().unwrap();
    // result!(result);

    // optimized implementation: ~300ms -> ~60ms in release mode ...but 3s -> 11s in debug mode 😅
    const PRUNE_LIMIT: usize = 16777216;
    const PRUNE_MASK: usize = PRUNE_LIMIT - 1;
    fn process(mut x: usize) -> usize {
        x = (x ^ (x << 6)) & PRUNE_MASK;
        x = (x ^ (x >> 5)) & PRUNE_MASK;
        x = (x ^ (x << 11)) & PRUNE_MASK;
        x
    }
    fn process_get_diff_10(x: &mut usize) -> usize {
        let next = process(*x);
        let diff = (next % 10) + 10 - (*x % 10); // +10 to avoid negative values
        *x = next;
        diff
    }

    // each diff goes from -9 to 9, with +10 it goes from 1 to 19 => 5 bits per diff
    const LIMIT: usize = 1 << 20;
    const MASK: usize = LIMIT - 1;
    let mut sequences = vec![0; LIMIT];
    let mut seen = vec![false; LIMIT];
    for mut x in input.lines().map(|l| parse_u(l) as usize) {
        seen.fill(false);
        let mut sequence = usize::MAX;
        for i in 0..2000 {
            sequence = ((sequence << 5) | process_get_diff_10(&mut x)) & MASK;
            if i >= 4 && !seen[sequence] {
                seen[sequence] = true;
                sequences[sequence] += x % 10;
            }
        }
    }

    let result = sequences.iter().max().unwrap();
    result!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/22.txt");

    const PRUNE_LIMIT: usize = 16777216;
    const PRUNE_MASK: usize = PRUNE_LIMIT - 1;

    fn process(mut x: usize) -> usize {
        x = (x ^ (x << 6)) & PRUNE_MASK;
        x = (x ^ (x >> 5)) & PRUNE_MASK;
        x = (x ^ (x << 11)) & PRUNE_MASK;
        x
    }

    let result = input
        .lines()
        .map(parse_u)
        .map(|mut x| {
            for _ in 0..2000 {
                x = process(x);
            }
            x
        })
        .sum::<usize>();

    result!(result);
}
