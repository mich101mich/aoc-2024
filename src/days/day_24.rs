use std::{cmp::Reverse, collections::BTreeSet};

use crate::utils::*;

#[derive(Debug, Clone, Copy, FromScanf, PartialEq, Eq, Hash)]
#[sscanf(autogen = "UPPERCASE")]
enum LogicOp {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, Copy, FromScanf)]
#[sscanf("{a} {op} {b} -> {out}")]
struct Gate<'a> {
    a: &'a str,
    b: &'a str,
    op: LogicOp,
    out: &'a str,
}

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct Rule<'a> {
        a: &'a str,
        b: &'a str,
        op: LogicOp,
    }
    impl<'a> Rule<'a> {
        fn new(a: &'a str, b: &'a str, op: LogicOp) -> Self {
            Self {
                a: a.min(b),
                b: a.max(b),
                op,
            }
        }
    }

    let (_initial, logic) = input.split_once("\n\n").unwrap();

    let mut logic = logic
        .lines()
        .map(|l| sscanf!(l, "{Gate}").unwrap())
        .map(|gate| (gate.out, Rule::new(gate.a, gate.b, gate.op)))
        .to_map();

    fn check_result<'a>(
        logic: &HashMap<&'a str, Rule<'a>>,
        x: u64,
        y: u64,
        x_vars: &[&'a str],
        y_vars: &[&'a str],
    ) -> bool {
        let mut values = HashMap::new();
        for i in 0..x_vars.len() {
            values.insert(x_vars[i], (x >> i) & 1 == 1);
            values.insert(y_vars[i], (y >> i) & 1 == 1);
        }

        let mut waiting_on: HashMap<&str, Vec<_>> = HashMap::new();
        let mut queue = logic.keys().copied().to_vec();

        while let Some(output) = queue.pop() {
            let rule = logic[output];
            let a = values.get(rule.a).copied();
            let b = values.get(rule.b).copied();
            match (a, b) {
                (Some(a), Some(b)) => {
                    let result = match rule.op {
                        LogicOp::And => a & b,
                        LogicOp::Or => a | b,
                        LogicOp::Xor => a ^ b,
                    };
                    values.insert(output, result);
                    if let Some(waiting) = waiting_on.remove(&output) {
                        queue.extend(waiting);
                    }
                }
                (Some(a), None) => {
                    waiting_on.entry(rule.b).or_default().push(output);
                }
                _ => {
                    waiting_on.entry(rule.a).or_default().push(output);
                }
            }
        }

        let mut result = 0u64;
        for (k, v) in values {
            if let Ok(i) = sscanf!(k, "z{u32}") {
                result |= (v as u64) << i;
            }
        }
        result == x + y
    }

    fn verify(logic: &HashMap<&str, Rule>, x_vars: &[&str], y_vars: &[&str]) -> bool {
        let input_limit = 1 << x_vars.len();
        let mut rng = rand::thread_rng();

        for _ in 0..10 {
            let x = rng.gen_range(0..input_limit);
            let y = rng.gen_range(0..input_limit);
            if !check_result(logic, x, y, x_vars, y_vars) {
                return false;
            }
        }
        check_result(logic, 0, 0, x_vars, y_vars)
            && check_result(logic, input_limit - 1, input_limit - 1, x_vars, y_vars)
    }

    let all_vars = logic
        .iter()
        .flat_map(|(k, v)| [k, v.a, v.b])
        .collect::<BTreeSet<_>>();
    let x_vars = all_vars
        .iter()
        .filter(|k| k.starts_with('x'))
        .copied()
        .to_vec();
    let y_vars = all_vars
        .iter()
        .filter(|k| k.starts_with('y'))
        .copied()
        .to_vec();
    let z_vars = all_vars
        .iter()
        .filter(|k| k.starts_with('z'))
        .copied()
        .to_vec();

    // binary adder:
    // each bit (N > 0) is the combination of its direct bits and the carry from the previous bit
    // - bitN = xN ^ yN
    // - zN = bitN ^ carryP
    // each carry (N > 0) is the combination of the direct overflow from the bits and the added bit+prev_carry
    // - overflowN = xN & yN
    // - add_overflowN = bitN & carryP
    // - carryN = overflowN | add_overflowN

    // observation from the input: all swaps are with at least one z variable
    let mut is_z_but_shouldnt = vec![];
    let mut should_be_z_but_isnt = vec![];
    let rule_0 = Rule::new("x00", "y00", LogicOp::Xor);
    let last_z = *z_vars.last().unwrap();
    for (&output, rule) in logic.iter() {
        if *rule == rule_0 || output == last_z {
            continue; // first and last rules are different
        }
        let is_z = output.starts_with('z');
        let should_be_z = (rule.op == LogicOp::Xor && !rule.a.starts_with('x'));
        if is_z && !should_be_z {
            is_z_but_shouldnt.push(output);
        } else if !is_z && should_be_z {
            should_be_z_but_isnt.push(output);
        }
    }

    const TARGET_SWAPS: usize = 4;
    assert!(is_z_but_shouldnt.len() <= TARGET_SWAPS);
    assert!(should_be_z_but_isnt.len() <= TARGET_SWAPS);

    let mut swaps = is_z_but_shouldnt
        .iter()
        .copied()
        .chain(should_be_z_but_isnt.iter().copied())
        .to_vec();

    let a = is_z_but_shouldnt[0];
    let b = is_z_but_shouldnt[1];
    let c = is_z_but_shouldnt[2];

    let used = swaps.iter().copied().to_set();

    let d_candidates = if is_z_but_shouldnt.len() == TARGET_SWAPS {
        is_z_but_shouldnt.iter().copied().to_vec()
    } else {
        logic
            .keys()
            .filter(|k| !used.contains(*k))
            .copied()
            .to_vec()
    };
    let d_target_candidates = if should_be_z_but_isnt.len() == TARGET_SWAPS {
        should_be_z_but_isnt.iter().copied().to_vec()
    } else {
        logic
            .keys()
            .filter(|k| !used.contains(*k))
            .copied()
            .to_vec()
    };

    'outer: for &a_target in &should_be_z_but_isnt {
        for &b_target in &should_be_z_but_isnt {
            if b_target == a_target {
                continue;
            }
            for &c_target in &should_be_z_but_isnt {
                if c_target == a_target || c_target == b_target {
                    continue;
                }
                for &d in &d_candidates {
                    for &d_target in &d_target_candidates {
                        if d_target == d {
                            continue;
                        }

                        fn swap<'a>(
                            a: &'a str,
                            b: &'a str,
                            new_logic: &mut HashMap<&'a str, Rule<'_>>,
                        ) {
                            let a_rule = new_logic.remove(a).unwrap();
                            let b_rule = new_logic.remove(b).unwrap();
                            new_logic.insert(a, b_rule);
                            new_logic.insert(b, a_rule);
                        }
                        let mut new_logic = logic.clone();
                        swap(a, a_target, &mut new_logic);
                        swap(b, b_target, &mut new_logic);
                        swap(c, c_target, &mut new_logic);
                        swap(d, d_target, &mut new_logic);

                        if verify(&new_logic, &x_vars, &y_vars) {
                            swaps.push(d);
                            swaps.push(d_target);
                            break 'outer;
                        }
                    }
                }
            }
        }
    }

    swaps.sort_unstable();
    let result = swaps.join(",");
    result!(result);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/24.txt");
    // let input = "";

    let (initial, logic) = input.split_once("\n\n").unwrap();
    let initial = initial
        .lines()
        .map(|l| sscanf!(l, "{str}: {u8}").unwrap())
        .map(|(a, b)| (a, b != 0))
        .to_map();

    let logic = logic
        .lines()
        .map(|l| sscanf!(l, "{Gate}").unwrap())
        .to_vec();

    let mut values = initial;
    let mut waiting_on = HashMap::new();
    let mut queue = (0..logic.len()).to_vec();

    while let Some(i) = queue.pop() {
        let gate = logic[i];
        let a = values.get(gate.a).copied();
        let b = values.get(gate.b).copied();
        match (a, b) {
            (Some(a), Some(b)) => {
                let result = match gate.op {
                    LogicOp::And => a & b,
                    LogicOp::Or => a | b,
                    LogicOp::Xor => a ^ b,
                };
                values.insert(gate.out, result);
                if let Some(waiting) = waiting_on.remove(&gate.out) {
                    queue.extend(waiting);
                }
            }
            (Some(a), None) => {
                waiting_on.entry(gate.b).or_insert_with(Vec::new).push(i);
            }
            _ => {
                waiting_on.entry(gate.a).or_insert_with(Vec::new).push(i);
            }
        }
    }

    let mut result = 0u64;
    for (k, v) in values {
        if let Ok(i) = sscanf!(k, "z{u32}") {
            result |= (v as u64) << i;
        }
    }
    result!(result);
}
