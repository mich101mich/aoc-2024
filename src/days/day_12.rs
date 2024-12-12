use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");

    let mut grid = char_grid(input);

    let w = grid.w() as isize;
    let h = grid.h() as isize;

    let mut queue = vec![];
    let mut count_area = |x: isize, y: isize| {
        let target = grid[p2(x, y)];
        if target == '.' || target.is_ascii_lowercase() {
            return 0;
        }
        let replacement = target.to_ascii_lowercase();
        let mut area = 0;
        let mut perimeter = HashMap::<Dir, HashSet<_>>::new();
        queue.push(p2(x, y));
        grid[p2(x, y)] = replacement;
        while let Some(p) = queue.pop() {
            area += 1;
            for d in Dir::all() {
                let np = p + d;
                let c = grid.get(np).copied().unwrap_or('.');
                if c == target {
                    queue.push(np);
                    grid[np] = replacement;
                } else if c != replacement {
                    perimeter.entry(d).or_default().insert(p);
                }
            }
        }

        let mut sections = 0;

        for (dir, points) in perimeter {
            let mut points = points;
            let side = dir.clockwise();
            while let Some(&p) = points.iter().next() {
                points.remove(&p);
                let mut next = p + side;
                while points.remove(&next) {
                    next += side;
                }
                next = p - side;
                while points.remove(&next) {
                    next -= side;
                }
                sections += 1;
            }
        }

        area * sections
    };

    let mut result = 0;
    for y in 0..h {
        for x in 0..w {
            result += count_area(x, y);
        }
    }

    result!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/12.txt");

    let mut grid = char_grid(input);

    let w = grid.w() as isize;
    let h = grid.h() as isize;

    let mut queue = vec![];
    let mut count_area = |x: isize, y: isize| {
        let target = grid[p2(x, y)];
        if target == '.' || target.is_ascii_lowercase() {
            return 0;
        }
        let replacement = target.to_ascii_lowercase();
        let mut area = 0;
        let mut perimeter = 0;
        queue.push(p2(x, y));
        grid[p2(x, y)] = replacement;
        while let Some(p) = queue.pop() {
            area += 1;
            for d in Dir::all() {
                let np = p + d;
                let c = grid.get(np).copied().unwrap_or('.');
                if c == target {
                    queue.push(np);
                    grid[np] = replacement;
                } else if c != replacement {
                    perimeter += 1;
                }
            }
        }
        area * perimeter
    };

    let mut result = 0;
    for y in 0..h {
        for x in 0..w {
            result += count_area(x, y);
        }
    }

    result!(result);
}
