use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/04.txt");

    let mut grid = char_grid(input);

    let (w, h) = (grid.w(), grid.h());

    let mut count = 0;
    for y in 0..h - 2 {
        for x in 0..w - 2 {
            if grid[p2(x + 1, y + 1)] != 'A' {
                continue;
            }
            let forward = [grid[p2(x, y)], grid[p2(x + 2, y + 2)]];
            let backward = [grid[p2(x + 2, y)], grid[p2(x, y + 2)]];
            if (forward == ['M', 'S'] || forward == ['S', 'M'])
                && (backward == ['M', 'S'] || backward == ['S', 'M'])
            {
                count += 1;
            }
        }
    }

    pv!(count);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/04.txt");

    let mut grid = char_grid(input);

    let (w, h) = (grid.w(), grid.h());

    fn count_iter<'a>(mut iter: impl Iterator<Item = &'a char>, count: &mut usize) {
        let mut queue = VecDeque::new();
        queue.extend(iter.by_ref().copied().take(3));
        for c in iter {
            queue.push_back(*c);
            if queue == ['X', 'M', 'A', 'S'] || queue == ['S', 'A', 'M', 'X'] {
                *count += 1;
            }
            queue.pop_front();
        }
    }

    let mut count = 0;
    for r in 0..h {
        count_iter(grid.row(r), &mut count);
    }
    for c in 0..w {
        count_iter(grid.col(c), &mut count);
    }

    let w = w as isize;
    let h = h as isize;
    for x in -h - 1..w + h + 1 {
        count_iter((0..h).filter_map(|y| grid.get(p2(x + y, y))), &mut count);
        count_iter((0..h).filter_map(|y| grid.get(p2(x - y, y))), &mut count);
    }

    pv!(count);
}
