use crate::utils::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Element<T> {
    cost: Cost,
    payload: T,
}
impl<T> Element<T> {
    fn new(cost: Cost, payload: T) -> Self {
        Element { cost, payload }
    }
}
impl<Id: Eq> PartialOrd for Element<Id> {
    fn partial_cmp(&self, rhs: &Element<Id>) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}
impl<Id: Eq> Ord for Element<Id> {
    fn cmp(&self, rhs: &Element<Id>) -> Ordering {
        rhs.cost.cmp(&self.cost) // reverse order for max-heap
    }
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");
    // let input = "";

    let mut grid = char_grid(input);

    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();
    grid[start] = '.';
    grid[end] = '.';
    let start_dir = Right;

    let mut min_cost_to = grid.map(|_| [usize::MAX; 4]);
    min_cost_to[start][start_dir.num()] = 0;

    let mut queue = BinaryHeap::new();
    queue.push(Element::new(0, (start, start_dir)));

    let mut candidates = vec![];

    let mut result = 0;
    while let Some(Element {
        cost,
        payload: (pos, dir),
    }) = queue.pop()
    {
        if min_cost_to[pos][dir.num()] < cost {
            continue; // found a better path since this was added
        }

        if pos == end {
            result = cost;
            break;
        }

        candidates.clear();
        if grid.get(pos + dir) == Some(&'.') {
            candidates.push(Element::new(cost + 1, (pos + dir, dir)));
        }
        candidates.push(Element::new(cost + 1000, (pos, dir.clockwise())));
        candidates.push(Element::new(cost + 1000, (pos, dir.counter_clockwise())));

        for next in &candidates {
            if next.cost < min_cost_to[next.payload.0][next.payload.1.num()] {
                min_cost_to[next.payload.0][next.payload.1.num()] = next.cost;
                queue.push(*next);
            }
        }
    }

    let mut part_of_best = grid.map(|_| false);
    let mut processed = grid.map(|_| [false; 4]);
    let mut candidates = vec![];

    let mut queue = Dir::all().map(|dir| (end, dir, result)).to_vec();
    part_of_best[end] = true;
    processed[end] = [true; 4];

    while let Some((pos, dir, cost)) = queue.pop() {
        if grid.get(pos - dir) == Some(&'.') {
            candidates.push((pos - dir, dir, cost - 1));
        }
        if cost >= 1000 {
            candidates.push((pos, dir.clockwise(), cost - 1000));
            candidates.push((pos, dir.counter_clockwise(), cost - 1000));
        }

        for candidate in &candidates {
            if processed[candidate.0][candidate.1.num()] {
                continue;
            }

            if min_cost_to[candidate.0][candidate.1.num()] == candidate.2 {
                part_of_best[candidate.0] = true;
                processed[candidate.0][candidate.1.num()] = true;
                if candidate.2 > 0 {
                    queue.push(*candidate);
                }
            }
        }
    }

    result!(part_of_best.count());
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/16.txt");
    // let input = "";

    let mut grid = char_grid(input);

    let start = grid.find('S').unwrap();
    let end = grid.find('E').unwrap();
    grid[start] = '.';
    grid[end] = '.';
    let start_dir = Right;

    let mut min_cost_to = grid.map(|_| [usize::MAX; 4]);
    min_cost_to[start][start_dir.num()] = 0;

    let mut queue = BinaryHeap::new();
    queue.push(Element::new(0, (start, start_dir)));

    let mut candidates = vec![];

    let mut result = 0;
    while let Some(Element {
        cost,
        payload: (pos, dir),
    }) = queue.pop()
    {
        if min_cost_to[pos][dir.num()] < cost {
            continue; // found a better path since this was added
        }

        if pos == end {
            result = cost;
            break;
        }

        candidates.clear();
        if grid.get(pos + dir) == Some(&'.') {
            candidates.push(Element::new(cost + 1, (pos + dir, dir)));
        }
        candidates.push(Element::new(cost + 1000, (pos, dir.clockwise())));
        candidates.push(Element::new(cost + 1000, (pos, dir.counter_clockwise())));

        for next in &candidates {
            if next.cost < min_cost_to[next.payload.0][next.payload.1.num()] {
                min_cost_to[next.payload.0][next.payload.1.num()] = next.cost;
                queue.push(*next);
            }
        }
    }

    result!(result);
}
