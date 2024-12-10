use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let mut grid = digit_grid(input);
    let neighbourhood = grid.manhattan();

    let result = grid
        .find_all(0)
        .map(|p| {
            let mut positions = HashMap::new();
            let mut next_positions = HashMap::new();
            positions.insert(p, 1);
            for i in 1..=9 {
                for (pos, count) in &positions {
                    for n in neighbourhood.get_all_neighbors(*pos) {
                        if grid.get(n) == Some(&i) {
                            *next_positions.entry(n).or_default() += count;
                        }
                    }
                }
                std::mem::swap(&mut positions, &mut next_positions);
                next_positions.clear();
            }
            positions.values().sum::<usize>()
        })
        .sum::<usize>();

    pv!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/10.txt");

    let mut grid = digit_grid(input);
    let neighbourhood = grid.manhattan();

    let result = grid
        .find_all(0)
        .map(|p| {
            let mut positions = HashSet::new();
            let mut next_positions = HashSet::new();
            positions.insert(p);
            for i in 1..=9 {
                for pos in &positions {
                    for n in neighbourhood.get_all_neighbors(*pos) {
                        if grid.get(n) == Some(&i) {
                            next_positions.insert(n);
                        }
                    }
                }
                std::mem::swap(&mut positions, &mut next_positions);
                next_positions.clear();
            }
            positions.len()
        })
        .sum::<usize>();

    pv!(result);
}
