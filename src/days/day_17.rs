use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");

    let (wrong_a_start, _, _, prog) = sscanf!(
        input,
        "Register A: {usize}
Register B: {usize}
Register C: {usize}

Program: {str}"
    )
    .unwrap();

    let program = prog.split(',').map(parse_u).to_vec();

    struct State {
        a: usize,
        b: usize,
        c: usize,
    }
    impl State {
        fn combo(&self, num: usize) -> usize {
            match num {
                0..=3 => num,
                4 => self.a,
                5 => self.b,
                6 => self.c,
                _ => unreachable!(),
            }
        }
    }

    const OP_IS_COMBO: [bool; 8] = [true, false, true, false, false, true, true, true]; // 0, 2, 5, 6, 7

    assert!(program.len() % 2 == 0);
    let ops = program.chunks_exact(2).to_vec();
    let mut op_count = [0; 8];
    for op in &ops {
        op_count[op[0]] += 1;
    }

    // observations about the puzzle input:
    // - a is processed in blocks of 3 bits
    //   - assertion: only one operation modified a
    assert_eq!(op_count[0], 1);
    //   - assertion: the operation that modifies a shifts it by 3 bits
    assert!(ops.contains(&[0, 3].as_slice()));
    // - each block produces one output, meaning there are 3 * output_len bits in a
    //   - assertion: only one output operation
    assert_eq!(op_count[5], 1);
    // - program halts when a == 0
    //   - assertion: only one jump operation
    assert_eq!(op_count[3], 1);
    //   - assertion: jump operation is the last operation and jumps to the first operation
    assert_eq!(ops.last(), Some(&[3, 0].as_slice()));
    // - there is one source of chaos in the input, in that some higher bits of a are used in the output.
    //   - => start at the back where there are no higher bits, and work backwards
    // - values of b and c are overwritten in each iteration
    //   - assertion: the first operation that overwrites b is before the first operation that uses b
    let first_op_overwriting_b = ops
        .iter()
        .position(|op| (op[0] == 2 || op[0] == 6) && op[1] != 5);
    let first_op_using_b = ops
        .iter()
        .position(|op| op[0] == 1 || op[0] == 4 || (OP_IS_COMBO[op[0]] && op[1] == 5));
    assert!(
        (first_op_overwriting_b.is_none() && first_op_using_b.is_none())
            || first_op_overwriting_b.unwrap() < first_op_using_b.unwrap()
    );
    //   - assertion: the first operation that overwrites c is before the first operation that uses c
    let first_op_overwriting_c = ops.iter().position(|op| op[0] == 7 && op[1] != 6);
    let first_op_using_c = ops
        .iter()
        .position(|op| op[0] == 4 || (OP_IS_COMBO[op[0]] && op[1] == 6));
    assert!(
        (first_op_overwriting_c.is_none() && first_op_using_c.is_none())
            || first_op_overwriting_c.unwrap() < first_op_using_c.unwrap()
    );

    let mut prefixes = vec![];
    let mut next_prefixes = vec![0usize];
    for expected_start in (0..program.len()).rev() {
        prefixes.clear();
        prefixes.extend(next_prefixes.drain(..).map(|x| x << 3));
        for &prefix in &prefixes {
            for a_start in prefix..(prefix + 8) {
                let mut state = State {
                    a: a_start,
                    b: 0,
                    c: 0,
                };

                let mut expected_i = expected_start;
                let mut ip = 0;
                while ip + 1 < program.len() {
                    let rhs = program[ip + 1];
                    match program[ip] {
                        0 => state.a >>= state.combo(rhs),
                        1 => state.b ^= rhs,
                        2 => state.b = state.combo(rhs) % 8,
                        3 => {
                            if state.a != 0 {
                                ip = rhs;
                                continue;
                            }
                        }
                        4 => state.b ^= state.c,
                        5 => {
                            let val = state.combo(rhs) % 8;
                            if expected_i == program.len() || program[expected_i] != val {
                                expected_i = program.len() + 1;
                                break;
                            }
                            expected_i += 1;
                        }
                        6 => state.b = state.a >> state.combo(rhs),
                        7 => state.c = state.a >> state.combo(rhs),
                        _ => unreachable!(),
                    }
                    ip += 2;
                }
                if expected_i == program.len() {
                    next_prefixes.push(a_start);
                }
            }
        }
        assert!(!next_prefixes.is_empty());
    }
    let result = *next_prefixes.iter().min().unwrap();
    result!(result);

    let mut p1_input = input.replace(&wrong_a_start.to_string(), &result.to_string());
    let remapped = part_one_inner(&p1_input);
    assert_eq!(remapped, prog);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/17.txt");
    let result = part_one_inner(input);
    result!(result);
}

#[allow(unused)]
pub fn part_one_inner(input: &str) -> String {
    let (a_start, b_start, c_start, prog) = sscanf!(
        input,
        "Register A: {usize}
Register B: {usize}
Register C: {usize}

Program: {str}"
    )
    .unwrap();

    struct State {
        a: usize,
        b: usize,
        c: usize,
        ip: usize,
        program: Vec<usize>,
    }
    impl State {
        fn combo(&self, num: usize) -> usize {
            match num {
                0..=3 => num,
                4 => self.a,
                5 => self.b,
                6 => self.c,
                _ => unreachable!(),
            }
        }
    }

    let mut state = State {
        program: prog.split(',').map(parse_u).to_vec(),
        a: a_start,
        b: b_start,
        c: c_start,
        ip: 0,
    };

    let mut result = vec![];
    loop {
        if state.ip + 1 >= state.program.len() {
            break;
        }
        let rhs = state.program[state.ip + 1];
        match state.program[state.ip] {
            0 => state.a >>= state.combo(rhs),
            1 => state.b ^= rhs,
            2 => state.b = state.combo(rhs) % 8,
            3 => {
                if state.a != 0 {
                    state.ip = rhs;
                    continue;
                }
            }
            4 => state.b ^= state.c,
            5 => result.push(state.combo(rhs) % 8),
            6 => state.b = state.a >> state.combo(rhs),
            7 => state.c = state.a >> state.combo(rhs),
            _ => unreachable!(),
        }
        state.ip += 2;
    }
    let mut output = String::new();
    for c in result {
        output += &c.to_string();
        output.push(',');
    }
    output.pop();
    output
}
