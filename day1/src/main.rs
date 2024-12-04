use std::io::stdin;

fn main() {
    let mut left = vec![];
    let mut right = vec![];

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        let mut nums = line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let l = nums.next().unwrap();
        let r = nums.next().unwrap();
        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    // Part 1
    let zip = left.iter().zip(right.iter());
    let res1 = zip.fold(0, |acc, (l, r)| acc + l.abs_diff(*r));
    println!("{}", res1);

    // Part 2
    let res2 = left.into_iter().fold(0, |acc, l| {
        let low = right.partition_point(|r| r < &l);
        let high = right.partition_point(|r| r <= &l);
        acc + l * (high - low)
    });
    println!("{}", res2);
}
