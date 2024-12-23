use std::collections::BTreeSet;

use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");
    let mut conn = HashMap::new();

    input
        .lines()
        .map(|l| sscanf!(l, "{str}-{str}").unwrap())
        .for_each(|(a, b)| {
            conn.entry(a).or_insert_with(Vec::new).push(b);
            conn.entry(b).or_insert_with(Vec::new).push(a);
        });

    conn.iter_mut().for_each(|(_, v)| v.sort_unstable());

    fn recurse<'a>(
        current: &mut String,
        conn: &HashMap<&'a str, Vec<&'a str>>,
        candidates: &[&'a str],
        out: &mut String,
    ) {
        if candidates.is_empty() {
            if current.len() > out.len() {
                *out = current.clone();
            }
            return;
        }

        let start_len = current.len();

        let candidates = &mut &candidates[..]; // mut ref to a slice allows "removing" elements from the front
        let mut next_candidates = Vec::with_capacity(candidates.len());
        while let Some((&c, rest)) = candidates.split_first() {
            *candidates = rest;
            // The one part of the C++ std lib that is better than Rust's: Sorted list operations
            let mut iter = conn[c].iter().peekable();
            next_candidates.clear();
            next_candidates.extend(candidates.iter().copied().filter(|x| {
                while let Some(&y) = iter.peek() {
                    match x.cmp(y) {
                        std::cmp::Ordering::Less => return false,
                        std::cmp::Ordering::Equal => {
                            iter.next();
                            return true;
                        }
                        std::cmp::Ordering::Greater => {
                            iter.next();
                        }
                    }
                }
                false
            }));
            current.push(',');
            current.push_str(c);
            recurse(current, conn, &next_candidates, out);
            current.truncate(start_len);
        }
    }

    let mut starts = conn.keys().copied().to_vec();
    starts.sort_unstable();

    let mut result = String::new();
    let mut current = String::with_capacity(conn.len() * 3);
    let mut removed = HashSet::new();
    for a in starts {
        let mut candidates = conn.remove(a).unwrap(); // avoid duplicate checks by removing the node
        candidates.retain(|&x| !removed.contains(x));

        current.clear();
        current.push_str(a);
        recurse(&mut current, &conn, &candidates, &mut result);
        removed.insert(a);
    }

    result!(result);
}
use crate::utils::*;

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/23.txt");

    let mut conn = HashMap::new();

    input
        .lines()
        .map(|l| sscanf!(l, "{str}-{str}").unwrap())
        .for_each(|(a, b)| {
            conn.entry(a).or_insert_with(HashSet::new).insert(b);
            conn.entry(b).or_insert_with(HashSet::new).insert(a);
        });

    let mut seen = HashSet::new();
    for (a, a_list) in conn.iter() {
        if !a.starts_with('t') {
            continue;
        }
        let a_list = a_list.iter().copied().to_vec();
        for (i, b) in a_list.iter().enumerate() {
            for c in &a_list[i + 1..] {
                if conn[b].contains(c) {
                    let mut set = [*a, *b, *c];
                    set.sort();
                    seen.insert(set);
                }
            }
        }
    }
    result!(seen.len());
}
