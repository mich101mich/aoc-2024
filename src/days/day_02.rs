use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");

    fn is_diff_valid(diff: isize) -> bool {
        diff >= 1 && diff <= 3
    }
    fn is_valid(values: &[isize]) -> bool {
        let diffs = values.windows(2).map(|w| w[1] - w[0]).to_vec();
        let len = diffs.len();
        let mut start = 0;
        while start < len && is_diff_valid(diffs[start]) {
            start += 1;
        }
        if start >= len - 1 {
            return true; // either all valid or only last element invalid
        }
        let mut end = len - 1;
        while is_diff_valid(diffs[end]) {
            // no bounds check needed since there is at least one invalid element
            end -= 1;
        }
        if end == 0 {
            return true; // only first element invalid
        }

        if end == start {
            (start > 0 && is_diff_valid(diffs[start - 1] + diffs[start]))
                || (start < len - 1 && is_diff_valid(diffs[start] + diffs[start + 1]))
        } else {
            end == start + 1 && is_diff_valid(diffs[start] + diffs[end])
        }
    }

    let parsed = input
        .lines()
        .filter(|l| {
            let mut values = l.split_ascii_whitespace().map(parse).to_vec();
            if is_valid(&values) {
                return true;
            }
            values.iter_mut().for_each(|v| *v *= -1);
            is_valid(&values)
        })
        .count();

    pv!(parsed);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/02.txt");

    let parsed = input
        .lines()
        .filter(|l| {
            let values = l.split_ascii_whitespace().map(parse).to_vec();
            let mut iter = values.windows(2);
            let Some([a, b]) = iter.next() else {
                return false;
            };
            if a == b || a.abs_diff(*b) > 3 {
                return false;
            }
            if a < b {
                iter.all(|w| w[0] < w[1] && w[1] - w[0] <= 3)
            } else {
                iter.all(|w| w[0] > w[1] && w[0] - w[1] <= 3)
            }
        })
        .count();

    pv!(parsed);
}
