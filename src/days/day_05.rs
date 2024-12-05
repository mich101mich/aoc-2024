use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");

    let mut iter = input.lines();
    let mut rules = HashMap::<usize, Vec<usize>>::new();
    for line in iter.by_ref().take_while(|l| !l.is_empty()) {
        let (a, b) = sscanf!(line, "{usize}|{usize}").unwrap();
        rules.entry(a).or_default().push(b);
    }

    let result = iter
        .filter_map(|l| {
            let pages = l.split(',').map(parse_u).to_vec();
            let mut seen = HashSet::new();
            for &page in &pages {
                if let Some(req) = rules.get(&page) {
                    for &r in req {
                        if seen.contains(&r) {
                            return Some(pages);
                        }
                    }
                }
                seen.insert(page);
            }
            None
        })
        .map(|mut pages| {
            let mut relevant = pages.iter().copied().to_set();
            let mut in_edges = HashMap::<usize, HashSet<usize>>::new();
            for &page in &pages {
                in_edges.entry(page).or_default();
                let Some(req) = rules.get(&page) else {
                    continue;
                };
                for &out in req {
                    if relevant.contains(&out) {
                        in_edges.entry(out).or_default().insert(page);
                    }
                }
            }

            let mut queue = pages
                .iter()
                .copied()
                .filter(|p| in_edges[p].is_empty())
                .to_queue();

            pages.clear();
            while let Some(v) = queue.pop_front() {
                pages.push(v);
                let Some(req) = rules.get(&v) else {
                    continue;
                };
                for &next in req {
                    let Some(mut edges) = in_edges.get_mut(&next) else {
                        continue;
                    };
                    edges.remove(&v);
                    if edges.is_empty() {
                        queue.push_back(next);
                    }
                }
            }

            pages[pages.len() / 2]
        })
        .sum::<usize>();

    pv!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/05.txt");

    let mut iter = input.lines();
    let mut rules = HashMap::<usize, Vec<usize>>::new();
    for line in iter.by_ref().take_while(|l| !l.is_empty()) {
        let (a, b) = sscanf!(line, "{usize}|{usize}").unwrap();
        rules.entry(a).or_default().push(b);
    }

    let result = iter
        .filter_map(|l| {
            let pages = l.split(',').map(parse_u).to_vec();
            let mut seen = HashSet::new();
            for &page in &pages {
                if let Some(req) = rules.get(&page) {
                    for &r in req {
                        if seen.contains(&r) {
                            return None;
                        }
                    }
                }
                seen.insert(page);
            }
            Some(pages[pages.len() / 2])
        })
        .sum::<usize>();

    pv!(result);
}
