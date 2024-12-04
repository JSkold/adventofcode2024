use std::{io::stdin, iter::Peekable};

fn test_report<'a>(mut it: Peekable<impl Iterator<Item = &'a isize>>) -> bool {
    let init = it.next().unwrap();
    let sign = (*it.peek().unwrap() - init).signum();
    it.try_fold(init, |prev, cur| {
        let diff = cur - prev;
        if diff != 0 && diff.signum() == sign && diff.abs() <= 3 {
            Some(cur)
        } else {
            None
        }
    })
    .is_some()
}

fn main() {
    let mut res1 = 0usize;
    let mut res2 = 0usize;

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        // Collect
        let reports: Vec<_> = line
            .split_whitespace()
            .map(|s| s.parse::<isize>().unwrap())
            .collect();

        // Part 1
        if test_report(reports.iter().peekable()) {
            res1 += 1;
        }

        // Part 2
        for idx_rm in 0..reports.len() {
            if test_report(
                reports
                    .iter()
                    .enumerate()
                    .filter_map(|(i, it)| if idx_rm != i { Some(it) } else { None })
                    .peekable(),
            ) {
                res2 += 1;
                break;
            }
        }
    }

    println!("{}", res1);
    println!("{}", res2);
}
