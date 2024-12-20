use crate::utils::*;

fn flood_fill(
    grid: &Grid<char>,
    neighbourhood: ManhattanNeighborhood,
    out: &mut Grid<usize>,
    frontier: &mut Vec<Point>,
    next_frontier: &mut Vec<Point>,
) {
    for dist in 1.. {
        next_frontier.clear();
        for &pos in frontier.iter() {
            for next in neighbourhood.get_all_neighbors(pos) {
                if grid[next] == '#' {
                    continue;
                }
                if out[next] > dist {
                    out[next] = dist;
                    next_frontier.push(next);
                }
            }
        }
        if next_frontier.is_empty() {
            break;
        }
        std::mem::swap(frontier, next_frontier);
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");

    let mut grid = char_grid(input);

    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();
    grid[start] = '.';
    grid[end] = '.';

    let mut neighbourhood = grid.manhattan();

    let mut dist_to_start = grid.map(|_| i32::MAX as usize); // not using usize::MAX because of overflow
    let mut dist_to_end = grid.map(|_| i32::MAX as usize);

    let mut frontier = vec![start];
    let mut next_frontier = vec![];

    dist_to_start[start] = 0;
    flood_fill(
        &grid,
        neighbourhood,
        &mut dist_to_start,
        &mut frontier,
        &mut next_frontier,
    );

    frontier.clear();
    next_frontier.clear();
    frontier.push(end);
    dist_to_end[end] = 0;
    flood_fill(
        &grid,
        neighbourhood,
        &mut dist_to_end,
        &mut frontier,
        &mut next_frontier,
    );

    let total_cost = dist_to_start[end];

    let mut result = 0;

    for start in grid.find_all('.') {
        let start_cost = dist_to_start[start];
        for dist in 1..=20 {
            for end in manhattan_ring_iter(start.cast::<isize>().unwrap(), dist as isize) {
                if grid.get(end) != Some(&'.') {
                    continue;
                }
                let end_cost = dist_to_end[end];
                let cost = start_cost + dist + end_cost;
                if cost >= total_cost {
                    continue;
                }
                let saved = total_cost - cost;
                if saved >= 100 {
                    result += 1;
                }
            }
        }
    }

    result!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/20.txt");

    let mut grid = char_grid(input);

    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();
    grid[start] = '.';
    grid[end] = '.';

    let mut neighbourhood = grid.manhattan();

    let mut visited = Visited::new();
    visited.update(start, 0, start);

    let mut next = BinaryHeap::new();
    next.push(Element::new(start, 0, 0));

    let mut possible_cheats = HashMap::<_, Vec<_>>::new();

    while let Some(current) = next.pop() {
        if current.cost > visited.cost(current.id).unwrap() {
            continue;
        }
        if current.id == end {
            break;
        }

        for other_id in neighbourhood.get_all_neighbors(current.id) {
            let other_cost = current.cost + 1;
            if grid[other_id] == '#' {
                possible_cheats
                    .entry(current.id)
                    .or_default()
                    .push((other_id, other_cost + 1));
                continue;
            }
            if visited.update(other_id, other_cost, current.id) {
                next.push(Element::new(other_id, other_cost, 0));
            }
        }
    }

    let mut result = 0;
    for (p, cheats) in possible_cheats {
        for (cheat_pos, cheat_cost) in cheats {
            for next in neighbourhood.get_all_neighbors(cheat_pos) {
                if grid[next] == '.' && visited.cost(next).unwrap() > cheat_cost {
                    let saved = visited.cost(next).unwrap() - cheat_cost;
                    if saved >= 100 {
                        result += 1;
                    }
                }
            }
        }
    }
    result!(result);
}
