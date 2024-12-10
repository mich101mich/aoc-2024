use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");

    let mut grid = char_grid(input);

    let start = grid.find('^').unwrap();
    grid[start] = 'X';

    let mut pos = start;
    let mut dir = Dir::Up;

    loop {
        let next = pos + dir;
        if !grid.in_bounds(next) {
            break;
        }
        if grid[next] == '#' {
            dir = dir.clockwise();
            continue;
        }
        pos = next;
        grid[pos] = 'X';
    }

    grid[start] = '_';

    let mut candidates = grid.find_all('X').to_vec();
    let mut visited = grid.map(|_| [false; 4]);

    let result = candidates
        .iter()
        .filter(|&&blocked| {
            grid[blocked] = '#';

            visited.fill([false; 4]);
            pos = start;
            dir = Dir::Up;
            visited[start][dir.num()] = true;

            let cycles = loop {
                let next = pos + dir;
                if !grid.in_bounds(next) {
                    break false;
                }
                if grid[next] == '#' {
                    dir = dir.clockwise();
                    continue;
                }
                pos = next;
                if visited[pos][dir.num()] {
                    break true;
                }
                visited[pos][dir.num()] = true;
            };

            grid[blocked] = 'X';
            cycles
        })
        .count();
    result!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/06.txt");

    let mut grid = char_grid(input);

    let start = grid.find('^').unwrap();
    grid[start] = 'X';

    let mut pos = start;
    let mut dir = Dir::Up;

    loop {
        let next = pos + dir;
        if !grid.in_bounds(next) {
            break;
        }
        if grid[next] == '#' {
            dir = dir.clockwise();
            continue;
        }
        pos = next;
        grid[pos] = 'X';
    }

    let result = grid.count_with(|&c| c == 'X');
    result!(result);
}
