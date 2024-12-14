use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");
    // let input = "";

    let w = 101;
    let h = 103;

    let mut quadrants = [0; 4];

    let (mut p, v): (Vec<Point>, Vec<PointI>) = input
        .lines()
        .map(|l| sscanf!(l, "p={usize},{usize} v={isize},{isize}").unwrap())
        .map(|(px, py, vx, vy)| (p2(px, py), p2(vx, vy)))
        .unzip();

    let mut grid = Grid::new_clone(p2(w as usize, h as usize), false);
    let neigh = grid.moore();

    for i in 1.. {
        grid.fill(false);
        p.iter_mut().zip(&v).for_each(|(p, v)| {
            p.x = ((p.x as isize + w + v.x) % w) as usize;
            p.y = ((p.y as isize + h + v.y) % h) as usize;

            grid[*p] = true;
        });

        let has_neighbour = p
            .iter()
            .filter(|&&p| neigh.get_all_neighbors(p).filter(|n| grid[n]).count() >= 2)
            .count();

        if has_neighbour < 300 {
            continue;
        }

        grid.print_hashtag();
        result!(i);
        break;
    }
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/14.txt");
    // let input = "";

    let w = 101;
    let h = 103;

    let mut quadrants = [0; 4];

    for (mut p, v) in input
        .lines()
        .map(|l| sscanf!(l, "p={isize},{isize} v={isize},{isize}").unwrap())
        .map(|(px, py, vx, vy)| (p2(px, py), p2(vx, vy)))
    {
        p += 100 * v;
        p.x = (p.x % w + w) % w;
        p.y = (p.y % h + h) % h;
        if p.x < w / 2 {
            if p.y < h / 2 {
                quadrants[0] += 1;
            } else if p.y > h / 2 {
                quadrants[1] += 1;
            }
        } else if p.x > w / 2 {
            if p.y < h / 2 {
                quadrants[2] += 1;
            } else if p.y > h / 2 {
                quadrants[3] += 1;
            }
        }
    }

    let result = quadrants.iter().copied().product::<isize>();
    result!(result);
}
