use crate::utils::*;

#[allow(unused)]
pub fn run() {
    #[allow(unused_variables)]
    let input = include_str!("../input/09.txt");
    // let input = "";

    struct Digit {
        count: u8,
        original_pos: usize,
        file_id: Option<usize>,
    }

    let mut original_pos = 0;
    let mut line = input
        .bytes()
        .map(|b| b - b'0')
        .enumerate()
        .map(|(i, count)| {
            let ret = Digit {
                count,
                original_pos,
                file_id: if i % 2 == 0 { Some(i / 2) } else { None },
            };
            original_pos += count as usize;
            ret
        })
        .to_queue();

    let mut sum = 0;
    while !line.is_empty() {
        let Digit {
            count: file_count,
            original_pos: file_pos,
            file_id: Some(file_id),
        } = line.pop_back().unwrap()
        else {
            // skip empty
            continue;
        };

        let free = line
            .iter()
            .position(|d| d.file_id.is_none() && d.count >= file_count);
        let Some(free) = free else {
            for p in file_pos..file_pos + file_count as usize {
                sum += p * file_id;
            }
            continue;
        };

        let Digit {
            count: free_count,
            original_pos: free_pos,
            ..
        } = line[free];

        for p in free_pos..free_pos + file_count as usize {
            sum += p * file_id;
        }

        if free_count > file_count {
            line[free] = Digit {
                count: free_count - file_count,
                original_pos: free_pos + file_count as usize,
                file_id: None,
            };
        } else {
            line.remove(free);
        }
    }

    pv!(sum);
}

#[allow(unused)]
pub fn part_one() {
    #[allow(unused_variables)]
    let input = include_str!("../input/09.txt");
    // let input = "";

    enum Digit {
        File { count: u8, id: usize },
        Empty(u8),
    }

    let mut line = input
        .bytes()
        .map(|b| b - b'0')
        .enumerate()
        .map(|(i, b)| {
            if i % 2 == 0 {
                Digit::File {
                    count: b,
                    id: i / 2,
                }
            } else {
                Digit::Empty(b)
            }
        })
        .to_queue();

    let mut sum = 0;
    let mut pos = 0;
    while !line.is_empty() {
        match line.pop_front().unwrap() {
            Digit::File { count, id } => {
                for _ in 0..count {
                    sum += pos * id;
                    pos += 1;
                }
            }
            Digit::Empty(mut n) => {
                while n > 0 {
                    if line.is_empty() {
                        break;
                    }
                    match line.pop_back().unwrap() {
                        Digit::File { count, id } => {
                            let taken = n.min(count);
                            for _ in 0..taken {
                                sum += pos * id;
                                pos += 1;
                            }
                            n -= taken;
                            if count > taken {
                                line.push_back(Digit::File {
                                    count: count - taken,
                                    id,
                                });
                                break;
                            }
                        }
                        Digit::Empty(b) => {
                            // can't fill empty with empty
                        }
                    }
                }
            }
        }
    }

    pv!(sum);
}
