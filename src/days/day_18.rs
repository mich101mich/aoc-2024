use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");
    // let input = "";

    let mut points = input
        .lines()
        .map(|l| sscanf!(l, "{usize},{usize}").unwrap())
        .map(|(x, y)| p2(x, y))
        .to_vec();

    let w = 71;
    let goal = p2(w - 1, w - 1);
    let mut grid = Grid::new_clone(p2(w, w), false);

    let result = binary_search(0, points.len(), |taken| {
        for p in &points[..taken] {
            grid[*p] = true;
        }
        for p in &points[taken..] {
            grid[*p] = false;
        }
        grid.a_star(p2(0, 0), goal, |b| !*b, |p| p.manhattan(goal))
            .is_none()
    });

    let p = points[result - 1];
    result!("{},{}", p.x, p.y);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/18.txt");

    let points = input
        .lines()
        .map(|l| sscanf!(l, "{usize},{usize}").unwrap())
        .map(|(x, y)| p2(x, y));

    let w = 71;
    let mut grid = Grid::new_clone(p2(w, w), false);

    for p in points.take(1024) {
        grid[p] = true;
    }

    let goal = p2(w - 1, w - 1);
    let result = grid
        .a_star(p2(0, 0), goal, |b| !*b, |p| p.manhattan(goal))
        .unwrap()
        .cost;
    result!(result);
}
