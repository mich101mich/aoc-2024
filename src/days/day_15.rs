use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");

    let (grid, dirs) = input.split_once("\n\n").unwrap();

    let grid = grid
        .replace('#', "##")
        .replace('O', "[]")
        .replace('.', "..")
        .replace('@', "@.");
    let mut grid = char_grid(&grid);
    let dirs = dirs.chars().filter_map(Dir::from_char);

    let bounds = grid.bounds();

    let start = grid.find('@').unwrap();
    grid[start] = '.';

    let mut pos = start;
    let mut boxes = HashSet::<Point>::new();
    let mut boxes_todo: Vec<Point> = vec![];
    for dir in dirs {
        let next = pos + dir;
        match grid[next] {
            '#' => continue,
            '.' => {
                pos = next;
                continue;
            }
            '[' => boxes_todo.push(next),
            _ => boxes_todo.push(next + Left),
        }
        boxes.insert(boxes_todo[0]);

        let mut possible = true;
        while let Some(b) = boxes_todo.pop() {
            if let Some(n) = dir.bounded_add(b, bounds).filter(|_| dir != Right) {
                if let Some(next_box) = match grid[n] {
                    '#' => {
                        possible = false;
                        break;
                    }
                    '.' => None,
                    '[' => Some(n),
                    _ => Some(n + Left),
                } {
                    if boxes.insert(next_box) {
                        boxes_todo.push(next_box);
                    }
                }
            }
            if let Some(n) = dir.bounded_add(b + Right, bounds).filter(|_| dir != Left) {
                if let Some(next_box) = match grid[n] {
                    '#' => {
                        possible = false;
                        break;
                    }
                    '.' => None,
                    '[' => Some(n),
                    _ => Some(n + Left),
                } {
                    if boxes.insert(next_box) {
                        boxes_todo.push(next_box);
                    }
                }
            }
        }

        if possible {
            pos = next;
            for b in boxes.iter() {
                grid[b] = '.';
                grid[*b + Right] = '.';
            }
            for b in boxes.iter() {
                grid[*b + dir] = '[';
                grid[*b + Right + dir] = ']';
            }
        }

        boxes.clear();
        boxes_todo.clear();
    }

    let result = grid
        .grid_iter_index()
        .filter(|&(_, c)| *c == '[')
        .map(|(p, _)| p.y * 100 + p.x)
        .sum::<usize>();

    result!(result);
}
use crate::utils::*;

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/15.txt");
    let (grid, dirs) = input.split_once("\n\n").unwrap();

    let mut grid = char_grid(grid);
    let dirs = dirs.chars().filter_map(Dir::from_char);

    let bounds = grid.bounds();

    let start = grid.find('@').unwrap();
    grid[start] = '.';

    let mut pos = start;
    for dir in dirs {
        let next = pos + dir;
        if grid[next] == '#' {
            continue;
        } else if grid[next] == 'O' {
            let mut p = next;
            while let Some(n) = dir.bounded_add(p, bounds) {
                p = n;
                if grid[p] != 'O' {
                    break;
                }
            }
            if grid[p] != '.' {
                continue;
            }
            grid[p] = 'O';
            grid[next] = '.';
        }
        pos = next;
    }

    let result = grid
        .grid_iter_index()
        .filter(|&(_, c)| *c == 'O')
        .map(|(p, _)| p.y * 100 + p.x)
        .sum::<usize>();

    result!(result);
}
