use std::io::stdin;

enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

fn is_xmas(map: &Vec<Vec<char>>, pos: (usize, usize), dir: Direction) -> bool {
    let step: (isize, isize) = match dir {
        Direction::North => (0, -1),
        Direction::NorthEast => (1, -1),
        Direction::East => (1, 0),
        Direction::SouthEast => (1, 1),
        Direction::South => (0, 1),
        Direction::SouthWest => (-1, 1),
        Direction::West => (-1, 0),
        Direction::NorthWest => (-1, -1),
    };

    let mut word = String::new();
    for i in 0..4 {
        let new_pos = (
            pos.0.checked_add_signed(step.0 * i),
            pos.1.checked_add_signed(step.1 * i),
        );
        if let (Some(x), Some(y)) = new_pos {
            if let Some(c) = map.get(y).and_then(|v| v.get(x)) {
                word.push(*c);
            }
        }
    }

    word == "XMAS"
}

fn count_xmas(map: &Vec<Vec<char>>, pos: (usize, usize)) -> usize {
    use Direction::*;
    [
        North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
    ]
    .into_iter()
    .map(|dir| is_xmas(map, pos, dir))
    .filter(|b| *b)
    .count()
}

fn main() {
    let mut map: Vec<Vec<_>> = Vec::new();

    let mut lines = stdin().lines();
    while let Some(Ok(line)) = lines.next() {
        map.push(line.chars().collect::<Vec<char>>());
    }

    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map.get(y).unwrap().len() {
            if *map.get(y).unwrap().get(x).unwrap() == 'X' {
                let s = count_xmas(&map, (x, y));
                sum += s;
            }
        }
    }

    println!("{sum}");
}
