use rayon::result;

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

    let z_vars = logic
        .keys()
        .filter(|k| k.starts_with('z'))
        .copied()
        .to_ord_set();

    // binary adder:
    // each bit (N > 0) is the combination of its direct bits and the carry from the previous bit
    // - bitN = xN ^ yN
    // - zN = bitN ^ carryP
    // each carry (N > 0) is the combination of the direct overflow from the bits and the added bit+prev_carry
    // - overflowN = xN & yN
    // - add_overflowN = bitN & carryP
    // - carryN = overflowN | add_overflowN

    let mut known_errors = BTreeSet::new();
    known_errors.insert("z00");
    known_errors.insert(*z_vars.last().unwrap());

    let [mut bit_carry_out_vars, mut z_out_vars, mut overflow_out_vars] =
        std::array::from_fn(|_| BTreeSet::new());
    let [mut bit_carry_in_vars, mut overflow_in_vars] = std::array::from_fn(|_| BTreeSet::new());

    for (&output, rule) in logic.iter() {
        if rule.a == "x00" {
            known_errors.insert(output); // structure of first bit is different
            continue;
        }

        match rule.op {
            LogicOp::And => {
                if rule.a.starts_with('x') {
                    assert!(rule.b.starts_with('y'));
                    assert_eq!(rule.a[1..], rule.b[1..]);
                    overflow_out_vars.insert(output); // overflowN
                } else {
                    bit_carry_in_vars.insert(rule.a);
                    bit_carry_in_vars.insert(rule.b);
                    overflow_out_vars.insert(output); // add_overflowN
                }
            }
            LogicOp::Or => {
                overflow_in_vars.insert(rule.a);
                overflow_in_vars.insert(rule.b);
                bit_carry_out_vars.insert(output); // carryN
            }
            LogicOp::Xor => {
                if rule.a.starts_with('x') {
                    assert!(rule.b.starts_with('y'));
                    assert_eq!(rule.a[1..], rule.b[1..]);
                    bit_carry_out_vars.insert(output); // bitN
                } else {
                    bit_carry_in_vars.insert(rule.a);
                    bit_carry_in_vars.insert(rule.b);
                    z_out_vars.insert(output); // zN
                }
            }
        }
    }

    let excluded = known_errors.clone();

    known_errors.extend(z_vars.symmetric_difference(&z_out_vars)); // extra_z_rules
    known_errors.extend(bit_carry_out_vars.symmetric_difference(&bit_carry_in_vars)); // extra_bit_or_carry_rules
    known_errors.extend(overflow_out_vars.symmetric_difference(&overflow_in_vars)); // extra_overflow_rules

    let mut swaps = known_errors.difference(&excluded).copied().to_vec();
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
