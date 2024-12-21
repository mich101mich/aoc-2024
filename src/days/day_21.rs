use rayon::vec;

use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");

    let d_pad = byte_grid(" ^A\n<v>");
    let d_pad_symbols = [b'^', b'>', b'v', b'<', b'A'];
    let d_pad_symbol_index = {
        let mut map = [0; 256];
        for (i, &symbol) in d_pad_symbols.iter().enumerate() {
            map[symbol as usize] = i as u8;
        }
        map
    };
    const A: u8 = 4;

    const D_PADS: usize = 25;

    #[rustfmt::skip]
    let raw_d_pad_paths = [
        //from ^                 >                 v                <            A
        [vec![""        ], vec!["^<", "<^"], vec!["^"      ], vec![">^" ], vec!["<"      ]], // to ^
        [vec![">v", "v>"], vec![""        ], vec![">"      ], vec![">>" ], vec!["v"      ]], // to >
        [vec!["v"       ], vec!["<"       ], vec![""       ], vec![">"  ], vec!["<v","v<"]], // to v
        [vec!["v<"      ], vec!["<<"      ], vec!["<"      ], vec![""   ], vec!["v<<"    ]], // to <
        [vec![">"       ], vec!["^"       ], vec![">^","^>"], vec![">>^"], vec![""       ]], // to A
    ];

    // d_pad depth -> sequence -> steps
    let mut cache: [HashMap<Vec<u8>, usize>; D_PADS] = std::array::from_fn(|_| HashMap::new());

    // from -> to -> [possible shortest paths]
    let d_pad_paths: [[Vec<Vec<u8>>; 5]; 5] = std::array::from_fn(|row| {
        std::array::from_fn(|col| {
            raw_d_pad_paths[col][row]
                .iter()
                .map(|&s| s.bytes().map(|b| d_pad_symbol_index[b as usize]).to_vec())
                .to_vec()
        })
    });

    fn cost_of(
        sequence: &[u8],
        remaining_bots: usize,
        cache: &mut [HashMap<Vec<u8>, usize>; D_PADS],
        d_pad_paths: &[[Vec<Vec<u8>>; 5]; 5],
    ) -> usize {
        if remaining_bots == 0 {
            return sequence.len(); // human input: buttons can be pressed immediately
        } else if let Some(&cost) = cache[remaining_bots - 1].get(sequence) {
            return cost;
        }

        let mut cost = 0;
        let mut pos = A;
        let mut next_sequence = Vec::new();
        for &b in sequence {
            let mut best = usize::MAX;
            for possible in &d_pad_paths[pos as usize][b as usize] {
                next_sequence.clear();
                next_sequence.extend_from_slice(possible);
                next_sequence.push(A);
                best = best.min(cost_of(
                    &next_sequence,
                    remaining_bots - 1,
                    cache,
                    d_pad_paths,
                ));
            }
            cost += best;
            pos = b;
        }
        assert_eq!(pos, A);

        cache[remaining_bots - 1].insert(sequence.to_vec(), cost);
        cost
    }

    let num_pad = byte_grid("789\n456\n123\n 0A");
    let num_pad_start = num_pad.find(b'A').unwrap();

    let mut frontier = vec![];
    let mut next_frontier = vec![];
    let mut target_paths = vec![];

    let result = input
        .lines()
        .map(|l| {
            let mut pos = num_pad_start;
            let mut cost = 0;
            for target in l.bytes() {
                let target_pos = num_pad.find(target).unwrap();
                frontier.clear();
                frontier.push((pos, vec![]));
                next_frontier.clear();
                target_paths.clear();
                while target_paths.is_empty() {
                    for (p, mut path) in frontier.drain(..) {
                        if p == target_pos {
                            path.push(A);
                            target_paths.push(path);
                            continue;
                        }
                        for dir in Dir::all() {
                            let Some(next) = dir.bounded_add(p, num_pad.bounds()) else {
                                continue;
                            };
                            if num_pad[next] == b' ' {
                                continue;
                            }
                            let mut next_path = path.clone();
                            next_path.push(d_pad_symbol_index[dir.to_char_arrow() as usize]);
                            next_frontier.push((next, next_path));
                        }
                    }
                    std::mem::swap(&mut frontier, &mut next_frontier);
                }

                let best = target_paths
                    .iter()
                    .map(|path| cost_of(path, D_PADS, &mut cache, &d_pad_paths))
                    .min()
                    .unwrap();

                cost += best;
                pos = target_pos;
            }
            cost * sscanf!(l, "{usize}A").unwrap()
        })
        .sum::<usize>();

    result!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/21.txt");

    let num_pad = char_grid("789\n456\n123\n 0A");
    let num_pad_start = num_pad.find('A').unwrap();
    let d_pad = char_grid(" ^A\n<v>");
    let d_pad_start = d_pad.find('A').unwrap();

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct State {
        first: Point,
        second: Point,
        third: Point,
        typed_correctly: usize,
    }

    const GOAL: State = State {
        first: p2(0, 0),
        second: p2(0, 0),
        third: p2(0, 0),
        typed_correctly: usize::MAX,
    }; // dummy value

    let start = State {
        first: num_pad_start,
        second: d_pad_start,
        third: d_pad_start,
        typed_correctly: 0,
    };

    let result = input
        .lines()
        .map(|l| {
            let chars = l.chars().to_vec();
            let get_neighbors = |state: State, out: &mut Vec<State>| {
                // pressing one of the directions
                for step in Dir::all() {
                    let Some(third) = step.bounded_add(state.third, d_pad.bounds()) else {
                        continue;
                    };
                    if d_pad[third] == ' ' {
                        continue;
                    }
                    out.push(State { third, ..state });
                }
                // pressing A
                let third_action = d_pad[state.third];
                if third_action != 'A' {
                    if let Some(second) = Dir::from_char(third_action)
                        .unwrap()
                        .bounded_add(state.second, d_pad.bounds())
                    {
                        if d_pad[second] != ' ' {
                            out.push(State { second, ..state });
                        }
                    }
                    return;
                }

                let second_action = d_pad[state.second];
                if second_action != 'A' {
                    if let Some(first) = Dir::from_char(second_action)
                        .unwrap()
                        .bounded_add(state.first, num_pad.bounds())
                    {
                        if num_pad[first] != ' ' {
                            out.push(State { first, ..state });
                        }
                    }
                    return;
                }

                if num_pad[state.first] == chars[state.typed_correctly] {
                    let typed_correctly = state.typed_correctly + 1;
                    if typed_correctly == chars.len() {
                        out.push(GOAL);
                    } else {
                        out.push(State {
                            typed_correctly,
                            ..state
                        });
                    }
                }
            };
            let path = a_star_search(get_neighbors, start, GOAL, |_| 0)
                .unwrap()
                .path
                .len()
                - 1; // -1 because the start state is included in the path
            path * sscanf!(l, "{usize}A").unwrap()
        })
        .sum::<usize>();

    result!(result);
}
