use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");

    let grid = char_grid(input);
    let mut antinodes = grid.map(|_| false);

    let mut freq: HashMap<char, Vec<_>> = HashMap::new();
    for (pos, c) in grid.grid_iter_index() {
        if *c == '.' {
            continue;
        }
        freq.entry(*c)
            .or_default()
            .push(pos.cast::<isize>().unwrap());
    }

    for positions in freq.values() {
        for a in positions {
            for b in positions {
                if a == b {
                    continue;
                }
                let mut diff = a - b;
                let mut p = *a;
                while let Some(left) = antinodes.get_mut(p) {
                    *left = true;
                    p += diff;
                }
            }
        }
    }

    let result = antinodes.count();
    pv!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/08.txt");

    let grid = char_grid(input);
    let mut antinodes = grid.map(|_| false);

    let mut freq: HashMap<char, Vec<_>> = HashMap::new();
    for (pos, c) in grid.grid_iter_index() {
        if *c == '.' {
            continue;
        }
        freq.entry(*c)
            .or_default()
            .push(pos.cast::<isize>().unwrap());
    }

    for positions in freq.values() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let a = positions[i];
                let b = positions[j];
                if let Some(left) = antinodes.get_mut(a + (a - b)) {
                    *left = true;
                }
                if let Some(right) = antinodes.get_mut(b + (b - a)) {
                    *right = true;
                }
            }
        }
    }

    let result = antinodes.count();
    pv!(result);
}
