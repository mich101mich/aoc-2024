use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");
    // let input = "";

    let mut iter = input.lines();
    let towels = iter
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.as_bytes())
        .to_vec();
    iter.next().unwrap(); // skip empty line

    let mut towel_starts = std::array::from_fn::<_, 256, _>(|_| vec![]);
    for t in towels {
        let (start, rest) = t.split_first().unwrap();
        towel_starts[*start as usize].push(rest);
    }

    fn recurse<'a>(
        towel_starts: &'a [Vec<&'a [u8]>; 256],
        target: &'a [u8],
        cache: &mut HashMap<&'a [u8], usize>,
    ) -> usize {
        if let Some(&v) = cache.get(target) {
            return v;
        }

        let Some((s, rest)) = target.split_first() else {
            return 1;
        };

        let mut ret = 0;
        for next in towel_starts[*s as usize].iter() {
            if let Some(rest) = rest.strip_prefix(*next) {
                ret += recurse(towel_starts, rest, cache);
            }
        }

        cache.insert(target, ret);

        ret
    }

    let mut result = 0;
    let mut cache = HashMap::new();
    for l in iter {
        result += recurse(&towel_starts, l.as_bytes(), &mut cache);
    }
    result!(result);
    // 3968216450757050614 high
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/19.txt");

    let mut iter = input.lines();
    let towels = iter.next().unwrap().split(", ").to_vec();
    iter.next().unwrap(); // skip empty line

    let mut re = String::from("^(?:");
    for t in towels {
        re += t;
        re.push('|');
    }
    re.pop();
    re += ")*$";
    let re = regex::Regex::new(&re).unwrap();

    let result = iter.filter(|l| re.is_match(l)).count();
    result!(result);
}
