use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");

    let result = input
        .lines()
        .map(|l| sscanf!(l, "{usize}: {str}").unwrap())
        .filter(|&(a, nums)| {
            let mut iter = nums.split(' ');

            let mut possible = HashSet::new();
            let mut next_possible = HashSet::new();
            possible.insert(parse_u(iter.next().unwrap()));

            for next in iter {
                let next_val = parse_u(next);
                for x in possible.iter() {
                    next_possible.insert(x + next_val);
                    next_possible.insert(x * next_val);
                    next_possible.insert(parse_u(&(x.to_string() + next)));
                }
                next_possible.retain(|&x| x <= a);
                std::mem::swap(&mut possible, &mut next_possible);
                next_possible.clear();
            }

            possible.contains(&a)
        })
        .map(|(a, _)| a)
        .sum::<usize>();

    pv!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/07.txt");

    let result = input
        .lines()
        .map(|l| sscanf!(l, "{usize}: {str}").unwrap())
        .filter(|&(a, nums)| {
            let mut iter = nums.split(' ').map(parse_u);

            let mut possible = HashSet::new();
            let mut next_possible = HashSet::new();
            possible.insert(iter.next().unwrap());

            for next in iter {
                for x in possible.iter() {
                    next_possible.insert(x + next);
                    next_possible.insert(x * next);
                }
                next_possible.retain(|&x| x <= a);
                std::mem::swap(&mut possible, &mut next_possible);
                next_possible.clear();
            }

            possible.contains(&a)
        })
        .map(|(a, _)| a)
        .sum::<usize>();

    pv!(result);
}
