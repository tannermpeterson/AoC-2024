fn main() {
    println!("Hello, world!");
}

mod day1 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn load_inputs() -> (Vec<i32>, Vec<i32>) {
        let file = File::open("inputs/day1.txt").unwrap();
        let buf_reader = BufReader::new(file);
        let mut list1 = Vec::<i32>::new();
        let mut list2 = Vec::<i32>::new();
        for line in buf_reader.lines() {
            if let Ok(line) = line {
                let mut line_split = line.split("   ");
                list1.push(line_split.next().unwrap().parse().unwrap());
                list2.push(line_split.next().unwrap().parse().unwrap());
            }
        }

        list1.sort();
        list2.sort();

        return (list1, list2);
    }

    #[test]
    fn part1() {
        let (list1, list2) = load_inputs();
        let res: i32 = list1
            .iter()
            .zip(list2.iter())
            .map(|(n1, n2)| (n2 - n1).abs())
            .sum();
        println!("D1P1: {res}");
    }

    #[test]
    fn part2() {
        let (list1, list2) = load_inputs();

        let mut score: i32 = 0;

        let mut list1_iter = list1.iter();
        let mut n1o = list1_iter.next();
        let mut list2_iter = list2.iter();
        let mut n2o = list2_iter.next();
        while n1o.is_some() && n2o.is_some() {
            let n1 = n1o.unwrap();
            let n2 = n2o.unwrap();
            if n1 == n2 {
                score += *n1;
                n2o = list2_iter.next();
            } else if n1 < n2 {
                n1o = list1_iter.next();
            } else {
                n2o = list2_iter.next();
            }
        }
        println!("D1P2: {score}");
    }
}

mod day2 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn load_inputs() -> Vec<Vec<u32>> {
        let file = File::open("inputs/day2.txt").unwrap();
        let buf_reader = BufReader::new(file);
        buf_reader
            .lines()
            .map(|line| {
                line.unwrap()
                    .split(" ")
                    .map(|nstr| nstr.parse().unwrap())
                    .collect()
            })
            .collect()
    }

    fn is_report_safe(levels: &Vec<u32>) -> bool {
        let increasing = levels[0] < levels[1];
        let mut levels_iter = levels.iter();
        let mut l = levels_iter.next().unwrap();
        for next in levels_iter {
            let diff = (*next as i32) - (*l as i32);
            if diff == 0 || diff.abs() > 3 || (diff > 0) != increasing {
                return false;
            }
            l = next;
        }
        true
    }

    #[test]
    fn part1() {
        let inputs = load_inputs();
        let num_safe: u32 = inputs
            .iter()
            .fold(0, |count, levels| count + is_report_safe(levels) as u32);
        println!("D2P1: {num_safe}");
    }

    fn is_report_safe_with_dampener(levels: &Vec<u32>) -> bool {
        // TODO find a way to do this that isn't O(n^2), i.e. rewrite is_report_safe
        for idx_to_drop in 0..levels.len() {
            let partial_levels: Vec<u32> = levels
                .iter()
                .enumerate()
                .filter_map(|(idx, level)| {
                    if idx != idx_to_drop {
                        Some(*level)
                    } else {
                        None
                    }
                })
                .collect();
            if is_report_safe(&partial_levels) {
                return true;
            }
        }
        false
    }

    #[test]
    fn part2() {
        let inputs = load_inputs();
        let num_safe: u32 = inputs.iter().fold(0, |count, levels| {
            count + is_report_safe_with_dampener(levels) as u32
        });
        println!("D2P2: {num_safe}");
    }
}

mod day3 {
    use std::fs::File;
    use std::io::Read;

    fn load_inputs() -> String {
        let mut buf = String::new();
        let _ = File::open("inputs/day3.txt")
            .unwrap()
            .read_to_string(&mut buf);
        buf
    }

    fn find_mul(text: &str) -> Option<u32> {
        let open_idx = text.find("(");
        let close_idx = text.find(")");
        if open_idx.is_none() || close_idx.is_none() || open_idx.is_some_and(|i| i != 0) {
            return None;
        }

        let arg_str = &text[open_idx.unwrap() + 1..close_idx.unwrap()];
        let args: Vec<&str> = arg_str.split(",").collect();
        if args.len() != 2 {
            return None;
        }
        let left = args[0];
        let right = args[1];
        if left.len() < 1 || 3 < left.len() || right.len() < 1 || 3 < right.len() {
            return None;
        }

        if let Ok(left) = left.parse::<u32>() {
            if let Ok(right) = right.parse::<u32>() {
                return Some(left * right);
            }
        }
        None
    }

    #[test]
    fn part1() {
        let inputs = load_inputs();
        let res: u32 = inputs.split("mul").filter_map(find_mul).sum();
        println!("D3P1: {res}");
    }

    #[test]
    fn part2() {
        let inputs = load_inputs();
        let res: u32 = inputs
            .split("don't()")
            .enumerate()
            .filter_map(|(idx, s)| {
                if idx == 0 {
                    Some(s)
                } else if let Some((_, s_enabled)) = s.split_once("do()") {
                    Some(s_enabled)
                } else {
                    None
                }
            })
            .flat_map(|s| s.split("mul").filter_map(find_mul))
            .sum();
        println!("D3P2: {res}");
    }
}

mod day4 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn load_inputs() -> Vec<Vec<char>> {
        let file = File::open("inputs/day4.txt").unwrap();
        let buf_reader = BufReader::new(file);
        buf_reader
            .lines()
            .map(|line| line.unwrap().chars().collect())
            .collect()
    }

    const TARGET: [char; 3] = ['M', 'A', 'S'];

    fn search_xmas_from_pos(
        inputs: &Vec<Vec<char>>,
        r_start: usize,
        c_start: usize,
        r_inc: isize,
        c_inc: isize,
    ) -> bool {
        let r_max = inputs.len() - 1;
        let c_max = inputs[0].len() - 1;

        let mut r = r_start;
        let mut c = c_start;
        r = r.wrapping_add_signed(r_inc);
        c = c.wrapping_add_signed(c_inc);
        for ch in TARGET.iter() {
            if r > r_max || c > c_max || inputs[r][c] != *ch {
                return false;
            }
            r = r.wrapping_add_signed(r_inc);
            c = c.wrapping_add_signed(c_inc);
        }
        return true;
    }

    #[test]
    fn part1() {
        let inputs = load_inputs();

        let mut res = 0;

        for r_start in 0..inputs.len() {
            for c_start in 0..inputs[0].len() {
                if inputs[r_start][c_start] != 'X' {
                    continue;
                }
                for r_inc in -1..=1 as isize {
                    for c_inc in -1..=1 as isize {
                        res += search_xmas_from_pos(&inputs, r_start, c_start, r_inc, c_inc) as u32;
                    }
                }
            }
        }

        println!("D4P1: {res}");
    }

    fn search_x_mas_from_pos(inputs: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
        let ch_ul = inputs[r - 1][c - 1];
        let ch_ur = inputs[r - 1][c + 1];
        let ch_bl = inputs[r + 1][c - 1];
        let ch_br = inputs[r + 1][c + 1];
        check_pairs(ch_ul, ch_br) && check_pairs(ch_ur, ch_bl)
    }

    fn check_pairs(ch1: char, ch2: char) -> bool {
        (ch1 == 'M' && ch2 == 'S') || (ch1 == 'S' && ch2 == 'M')
    }

    #[test]
    fn part2() {
        let inputs = load_inputs();

        let mut res = 0;

        for r in 1..inputs.len() - 1 {
            for c in 1..inputs[0].len() - 1 {
                res += (inputs[r][c] == 'A' && search_x_mas_from_pos(&inputs, r, c)) as u32;
            }
        }

        println!("D4P2: {res}");
    }
}

mod day5 {
    use std::collections::{HashMap, HashSet};
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn load_inputs() -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
        let file = File::open("inputs/day5a.txt").unwrap();
        let buf_reader = BufReader::new(file);
        let dependencies = buf_reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let mut pairs = line.split("|");
                (
                    pairs.next().unwrap().parse::<u32>().unwrap(),
                    pairs.next().unwrap().parse::<u32>().unwrap(),
                )
            })
            .collect();

        let file = File::open("inputs/day5b.txt").unwrap();
        let buf_reader = BufReader::new(file);
        let updates = buf_reader
            .lines()
            .map(|line| {
                line.unwrap()
                    .split(",")
                    .map(|n| n.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();

        (dependencies, updates)
    }

    fn create_dependency_mapping(dependency_pairs: &Vec<(u32, u32)>) -> HashMap<u32, HashSet<u32>> {
        let mut dependency_map: HashMap<u32, HashSet<u32>> = HashMap::new();
        for (first, second) in dependency_pairs {
            dependency_map
                .entry(*second)
                .or_insert(HashSet::new())
                .insert(*first);
        }
        dependency_map
    }

    fn is_update_valid(dependency_map: &mut HashMap<u32, HashSet<u32>>, pages: &[u32]) -> bool {
        match pages {
            [] => true,
            [_] => true,
            [curr, rest @ ..] => {
                let dependencies = dependency_map.entry(*curr).or_default();
                for page in rest {
                    if dependencies.contains(page) {
                        return false;
                    }
                }
                is_update_valid(dependency_map, rest)
            }
        }
    }

    #[test]
    fn part1() {
        let (dependency_pairs, updates) = load_inputs();

        let mut dependency_map = create_dependency_mapping(&dependency_pairs);

        let res: u32 = updates
            .iter()
            .map(|update| {
                if is_update_valid(&mut dependency_map, &update) {
                    update[update.len() / 2]
                } else {
                    0
                }
            })
            .sum();

        println!("D5P1: {res}");
    }

    fn fix_update(dependency_map: &mut HashMap<u32, HashSet<u32>>, pages: &mut [u32]) {
        let mut curr_idx = 0;
        while curr_idx < pages.len() - 1 {
            let curr = pages[curr_idx];
            let dependencies = dependency_map.entry(curr).or_default();
            let mut new_idx = 0;
            for (idx, page) in pages[curr_idx..].iter().enumerate() {
                if dependencies.contains(page) {
                    new_idx = idx + curr_idx;
                }
            }
            if new_idx > curr_idx {
                for next_idx in curr_idx + 1..=new_idx {
                    pages.swap(next_idx - 1, next_idx);
                }
            } else {
                curr_idx += 1;
            }
        }
    }

    #[test]
    fn part2() {
        let (dependency_pairs, updates) = load_inputs();

        let mut dependency_map = create_dependency_mapping(&dependency_pairs);

        let res: u32 = updates
            .into_iter()
            .map(|mut update| {
                if is_update_valid(&mut dependency_map, &update) {
                    0
                } else {
                    fix_update(&mut dependency_map, &mut update);
                    update[update.len() / 2]
                }
            })
            .sum();

        println!("D5P2: {res}");
    }
}

mod day6 {
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Write};

    #[derive(Clone)]
    enum TileState {
        UNVISITED,
        VISITED,
        OBSTACLE,
    }

    const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    fn load_inputs() -> (Vec<Vec<TileState>>, (usize, usize)) {
        let file = File::open("inputs/day6.txt").unwrap();
        let buf_reader = BufReader::new(file);

        let mut starting_pos = (0, 0);
        let tiles = buf_reader
            .lines()
            .enumerate()
            .map(|(r, line)| {
                line.unwrap()
                    .chars()
                    .enumerate()
                    .map(|(c, ch)| match ch {
                        '.' => TileState::UNVISITED,
                        '#' => TileState::OBSTACLE,
                        '^' => {
                            starting_pos = (r, c);
                            TileState::UNVISITED
                        }
                        bad_ch => panic!("invalid char: {bad_ch}"),
                    })
                    .collect()
            })
            .collect();

        (tiles, starting_pos)
    }

    #[test]
    fn part1() {
        let (mut tiles, mut pos) = load_inputs();

        let mut dir_idx = 0;
        let mut dir = DIRS[dir_idx];

        let mut count = 0;
        while pos.0 < tiles.len() && pos.1 < tiles[0].len() {
            let tile = &mut tiles[pos.0][pos.1];
            match tile {
                TileState::VISITED => {}
                TileState::UNVISITED => {
                    count += 1;
                    *tile = TileState::VISITED;
                }
                TileState::OBSTACLE => {
                    pos = (
                        pos.0.wrapping_add_signed(-dir.0),
                        pos.1.wrapping_add_signed(-dir.1),
                    );
                    dir_idx = (dir_idx + 1) % DIRS.len();
                    dir = DIRS[dir_idx];
                }
            }
            pos = (
                pos.0.wrapping_add_signed(dir.0),
                pos.1.wrapping_add_signed(dir.1),
            );
        }

        println!("D6P1: {count}");
    }

    fn check_loop(
        tiles: &Vec<Vec<(TileState, u8)>>,
        mut pos: (usize, usize),
        mut dir_idx: usize,
    ) -> bool {
        let mut tiles = tiles.clone();

        tiles[pos.0][pos.1].0 = TileState::OBSTACLE;

        let mut dir = DIRS[dir_idx];
        let mut dir_mask = 1 << dir_idx;

        while pos.0 < tiles.len() && pos.1 < tiles[0].len() {
            let tile = &mut tiles[pos.0][pos.1];
            match tile.0 {
                TileState::VISITED => {
                    if tile.1 & dir_mask > 0 {
                        return true;
                    }
                    tile.1 |= dir_mask;
                }
                TileState::UNVISITED => {
                    tile.0 = TileState::VISITED;
                    tile.1 |= dir_mask;
                }
                TileState::OBSTACLE => {
                    pos = (
                        pos.0.wrapping_add_signed(-dir.0),
                        pos.1.wrapping_add_signed(-dir.1),
                    );
                    dir_idx = (dir_idx + 1) % DIRS.len();
                    dir = DIRS[dir_idx];
                    dir_mask = 1 << dir_idx;
                }
            }
            pos = (
                pos.0.wrapping_add_signed(dir.0),
                pos.1.wrapping_add_signed(dir.1),
            );
        }

        false
    }

    #[test]
    fn part2() {
        let (tiles, starting_pos) = load_inputs();

        let mut pos = starting_pos;

        let mut dir_idx = 0;
        let mut dir = DIRS[dir_idx];

        let mut tiles: Vec<Vec<(TileState, u8)>> = tiles
            .into_iter()
            .map(|row| row.into_iter().map(|tile| (tile, 0)).collect())
            .collect();

        let mut count = 0;
        while pos.0 < tiles.len() && pos.1 < tiles[0].len() {
            match tiles[pos.0][pos.1].0 {
                TileState::VISITED => {
                    tiles[pos.0][pos.1].1 |= 1 << dir_idx;
                }
                TileState::UNVISITED => {
                    if pos != starting_pos {
                        count += check_loop(&tiles, pos, dir_idx) as u32;
                    }
                    tiles[pos.0][pos.1].0 = TileState::VISITED;
                    tiles[pos.0][pos.1].1 |= 1 << dir_idx;
                }
                TileState::OBSTACLE => {
                    pos = (
                        pos.0.wrapping_add_signed(-dir.0),
                        pos.1.wrapping_add_signed(-dir.1),
                    );
                    dir_idx = (dir_idx + 1) % DIRS.len();
                    dir = DIRS[dir_idx];
                }
            }
            pos = (
                pos.0.wrapping_add_signed(dir.0),
                pos.1.wrapping_add_signed(dir.1),
            );
        }

        println!("D6P2: {count}");
    }
}
