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
            conn.entry(a).or_insert_with(HashSet::new).insert(b);
            conn.entry(b).or_insert_with(HashSet::new).insert(a);
        });

    fn recurse<'a>(
        current: &mut HashSet<&'a str>,
        conn: &HashMap<&'a str, HashSet<&'a str>>,
        out: &mut String,
        visited: &mut HashSet<String>,
    ) {
        let mut name = current.iter().copied().to_vec();
        name.sort();
        let name = name.join(",");
        if !visited.insert(name.clone()) {
            return;
        }

        let candidates = conn
            .iter()
            .filter(|(k, v)| !current.contains(*k) && current.is_subset(v))
            .map(|(k, _)| *k)
            .to_vec();

        if candidates.is_empty() {
            if name.len() > out.len() {
                *out = name;
            }
            return;
        }

        for c in candidates {
            current.insert(c);
            recurse(current, conn, out, visited);
            current.remove(c);
        }
    }

    let mut result = String::new();
    let mut visited = HashSet::new();
    let mut start = HashSet::new();
    recurse(&mut start, &conn, &mut result, &mut visited);
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
