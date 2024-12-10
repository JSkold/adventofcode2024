use std::io::stdin;

fn parse_number(it: &mut (impl Iterator<Item = char> + Clone)) -> Option<isize> {
    let str = it
        .clone()
        .take_while(|c| c.is_numeric())
        .collect::<String>();
    let len = str.len();
    let num = match str.parse::<isize>() {
        Ok(num) => num,
        Err(_) => return None,
    };
    // Below should be replaced with advance_by when stabilized.
    it.nth(len - 1);
    Some(num)
}

fn mul(line: &str, idx: usize) -> Option<isize> {
    let mut it = line.chars().skip(idx);
    assert!(it.next().unwrap() == 'm');
    assert!(it.next().unwrap() == 'u');
    assert!(it.next().unwrap() == 'l');

    let Some('(') = it.next() else { return None };
    let first_term = parse_number(&mut it)?;
    let Some(',') = it.next() else { return None };
    let second_term = parse_number(&mut it)?;
    let Some(')') = it.next() else { return None };

    Some(first_term * second_term)
}

fn main() {
    let mut res1 = 0;
    let mut res2 = 0;

    let mut lines = stdin().lines();

    let mut do_last_line = true;
    while let Some(Ok(line)) = lines.next() {
        // Part 1
        let sum = line.match_indices("mul").fold(0, |acc, (idx, _)| {
            if let Some(res) = mul(&line, idx) {
                acc + res
            } else {
                acc
            }
        });

        res1 += sum;

        // Part 2
        let mut do_range: Vec<_> = Vec::new();
        let mut it_do = line.match_indices("do()").map(|(idx, _)| idx);
        let mut it_dont = line.match_indices("don't()").map(|(idx, _)| idx);
        let mut last_idx: usize = 0;
        let mut switch = do_last_line;
        loop {
            if switch {
                let next_dont = loop {
                    if let Some(next_dont) = it_dont.next() {
                        if next_dont < last_idx {
                            continue;
                        } else {
                            break Some(next_dont);
                        }
                    } else {
                        break None;
                    }
                };

                let range_end = match next_dont {
                    Some(idx) => {
                        switch = !switch;
                        idx
                    }
                    None => {
                        do_last_line = switch;
                        line.len()
                    }
                };
                do_range.push(last_idx..range_end);
                last_idx = range_end;
                if line.len() == last_idx {
                    break;
                }
            } else {
                let next_do = loop {
                    if let Some(next_do) = it_do.next() {
                        if next_do < last_idx {
                            continue;
                        } else {
                            break Some(next_do);
                        }
                    } else {
                        break None;
                    }
                };
                if let Some(next_do) = next_do {
                    switch = !switch;
                    last_idx = next_do;
                } else {
                    do_last_line = switch;
                    break;
                }
            }
        }
        
        let sum = line.match_indices("mul").fold(0, |acc, (idx, _)| {
            if let Some(res) = mul(&line, idx) {
                if do_range.iter().any(|r| r.contains(&idx)) {
                    acc + res
                } else {
                    acc
                }
            } else {
                acc
            }
        });

        res2 += sum;
    }

    println!("{res1}");
    println!("{res2}");
}
