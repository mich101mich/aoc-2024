use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");
    let mut iter = input.lines();
    let mut result = 0;
    while let Some(line) = iter.next() {
        let (ax, ay) = sscanf!(line, "Button A: X+{isize}, Y+{isize}").unwrap();
        let (bx, by) = sscanf!(iter.next().unwrap(), "Button B: X+{isize}, Y+{isize}").unwrap();
        let (px, py) = sscanf!(iter.next().unwrap(), "Prize: X={isize}, Y={isize}").unwrap();
        let (px, py) = (px + 10000000000000, py + 10000000000000);

        // with loss of generality, but asserting that AoC is nice, assume there is only one solution for a and b, if any

        // px = bx * b + ax * a  ^  py = by * b + ay * a
        // <=> bx * b = px - ax * a  ^  py = by * b + ay * a
        // <=> b = (px - ax * a) / bx  ^  py = by * b + ay * a
        // <=> b = (px - ax * a) / bx  ^  py = by * (px - ax * a) / bx + ay * a
        // <=> b = (px - ax * a) / bx  ^  py = by / bx * px - by / bx * ax * a + ay * a
        // <=> b = (px - ax * a) / bx  ^  py = by / bx * px - (by / bx * ax - ay) * a
        // <=> b = (px - ax * a) / bx  ^  (by / bx * ax - ay) * a = by / bx * px - py
        // <=> b = (px - ax * a) / bx  ^  a = (by / bx * px - py) / (by / bx * ax - ay)
        // <=> b = (px - ax * a) / bx  ^  a = (by * px - py * bx) / (by * ax - ay * bx)

        let a_numerator = by * px - py * bx;
        let a_denominator = by * ax - ay * bx;
        if a_numerator % a_denominator == 0 {
            let a = a_numerator / a_denominator;

            let b_numerator = px - ax * a;
            let b_denominator = bx;
            if b_numerator % b_denominator == 0 {
                let b = b_numerator / b_denominator;
                result += a * 3 + b;
            }
        }

        if iter.next().is_none() {
            break;
        }
    }

    result!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/13.txt");

    let mut iter = input.lines();
    let mut result = 0;
    while let Some(line) = iter.next() {
        let (ax, ay) = sscanf!(line, "Button A: X+{isize}, Y+{isize}").unwrap();
        let (bx, by) = sscanf!(iter.next().unwrap(), "Button B: X+{isize}, Y+{isize}").unwrap();
        let (px, py) = sscanf!(iter.next().unwrap(), "Prize: X={isize}, Y={isize}").unwrap();

        let mut res = isize::MAX;
        for b in 0..100 {
            let x = px - bx * b;
            let y = py - by * b;
            if x < 0 || y < 0 {
                break;
            }
            if x % ax == 0 && x / ax * ay == y {
                res = res.min(x / ax * 3 + b);
            }
        }
        if res != isize::MAX {
            result += res;
        }

        if iter.next().is_none() {
            break;
        }
    }

    result!(result);
}
