fn main() {}

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
    use std::borrow::Cow;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

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
        let mut tiles: Vec<Cow<Vec<(TileState, u8)>>> =
            tiles.iter().map(|row| Cow::Borrowed(row)).collect();

        tiles[pos.0].to_mut()[pos.1].0 = TileState::OBSTACLE;

        let mut dir = DIRS[dir_idx];
        let mut dir_mask = 1 << dir_idx;

        while pos.0 < tiles.len() && pos.1 < tiles[0].len() {
            let tile = &mut tiles[pos.0].to_mut()[pos.1];
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

mod day7 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn load_inputs() -> Vec<(Vec<u64>, u64)> {
        let file = File::open("inputs/day7.txt").unwrap();
        let buf_reader = BufReader::new(file);
        let in_out = buf_reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let spl: Vec<&str> = line.split(": ").collect();
                let ins: Vec<u64> = spl[1]
                    .split(" ")
                    .map(|nstr| nstr.parse().unwrap())
                    .collect();
                let out: u64 = spl[0].parse().unwrap();
                (ins, out)
            })
            .collect();

        in_out
    }

    fn check_test(ins: &Vec<u64>, out: u64, concat: bool) -> bool {
        _check_test(ins, out, concat, 0)
    }

    fn _check_test(ins: &[u64], out: u64, concat: bool, total: u64) -> bool {
        if total > out {
            false
        } else if ins.len() == 0 {
            total == out
        } else {
            _check_test(&ins[1..], out, concat, total + ins[0])
                || _check_test(&ins[1..], out, concat, total * ins[0])
                || (concat && _check_test(&ins[1..], out, concat, _concat(total, ins[0])))
        }
    }

    fn _concat(total: u64, next: u64) -> u64 {
        let mut spacer = 1;
        while next >= spacer {
            spacer *= 10;
        }
        total * spacer + next
    }

    #[test]
    fn part1() {
        let in_out = load_inputs();

        let res: u64 = in_out
            .iter()
            .map(|(ins, out)| {
                if check_test(ins, *out, false) {
                    *out
                } else {
                    0
                }
            })
            .sum();

        println!("D7P1: {res}");
    }

    #[test]
    fn part2() {
        let in_out = load_inputs();

        let res: u64 = in_out
            .iter()
            .map(
                |(ins, out)| {
                    if check_test(ins, *out, true) {
                        *out
                    } else {
                        0
                    }
                },
            )
            .sum();

        println!("D7P2: {res}");
    }
}

mod day8 {
    use std::collections::{HashMap, HashSet};
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn load_inputs() -> (HashMap<char, Vec<(usize, usize)>>, (usize, usize)) {
        let file = File::open("inputs/day8.txt").unwrap();
        let buf_reader = BufReader::new(file);

        let mut freq_to_locs: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

        let mut rows = 0;
        let mut cols = 0;

        for (r, line) in buf_reader.lines().enumerate() {
            rows = r + 1;
            for (c, ch) in line.unwrap().chars().enumerate() {
                cols = c + 1;
                if ch != '.' {
                    freq_to_locs.entry(ch).or_default().push((r, c));
                }
            }
        }

        (freq_to_locs, (rows, cols))
    }

    #[test]
    fn part1() {
        let (freq_to_locs, bounds) = load_inputs();

        let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

        for (_, locs) in freq_to_locs {
            for (first_idx, pos1) in locs.iter().enumerate() {
                for pos2 in &locs[first_idx + 1..] {
                    let diff = (
                        pos2.0 as isize - pos1.0 as isize,
                        pos2.1 as isize - pos1.1 as isize,
                    );
                    let antinode_loc = (
                        pos1.0.wrapping_add_signed(-diff.0),
                        pos1.1.wrapping_add_signed(-diff.1),
                    );
                    if antinode_loc.0 < bounds.0 && antinode_loc.1 < bounds.1 {
                        antinodes.insert(antinode_loc);
                    }
                    let antinode_loc = (
                        pos2.0.wrapping_add_signed(diff.0),
                        pos2.1.wrapping_add_signed(diff.1),
                    );
                    if antinode_loc.0 < bounds.0 && antinode_loc.1 < bounds.1 {
                        antinodes.insert(antinode_loc);
                    }
                }
            }
        }

        let res = antinodes.len();

        println!("D8P1: {res}");
    }

    #[test]
    fn part2() {
        let (freq_to_locs, bounds) = load_inputs();

        let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

        for (_, locs) in freq_to_locs {
            for (first_idx, pos1) in locs.iter().enumerate() {
                for pos2 in &locs[first_idx + 1..] {
                    let diff = (
                        pos2.0 as isize - pos1.0 as isize,
                        pos2.1 as isize - pos1.1 as isize,
                    );
                    let mut antinode_loc = *pos1;
                    while antinode_loc.0 < bounds.0 && antinode_loc.1 < bounds.1 {
                        antinodes.insert(antinode_loc);
                        antinode_loc = (
                            antinode_loc.0.wrapping_add_signed(-diff.0),
                            antinode_loc.1.wrapping_add_signed(-diff.1),
                        );
                    }
                    let mut antinode_loc = *pos2;
                    while antinode_loc.0 < bounds.0 && antinode_loc.1 < bounds.1 {
                        antinodes.insert(antinode_loc);
                        antinode_loc = (
                            antinode_loc.0.wrapping_add_signed(diff.0),
                            antinode_loc.1.wrapping_add_signed(diff.1),
                        );
                    }
                }
            }
        }

        let res = antinodes.len();

        println!("D8P2: {res}");
    }
}

mod day9 {
    fn load_inputs() -> Vec<u32> {
        let input = include_str!("../inputs/day9.txt");
        input.chars().filter_map(|ch| ch.to_digit(10)).collect()
    }

    #[test]
    fn part1() {
        let input = load_inputs();
        let input: Vec<Option<u32>> = input
            .iter()
            .enumerate()
            .flat_map(|(idx, n)| {
                let n = *n as usize;
                if idx % 2 == 0 {
                    vec![Some(idx as u32 / 2); n]
                } else {
                    vec![None; n]
                }
            })
            .collect();

        let mut input = input.iter();
        let mut front_next = input.next();
        let mut back_next = input.next_back();

        let mut compacted: Vec<u32> = Vec::new();
        while front_next.is_some() && back_next.is_some() {
            let front = front_next.unwrap();
            let back = back_next.unwrap();
            match front {
                Some(front) => {
                    compacted.push(*front);
                    front_next = input.next();
                }
                None => {
                    if let Some(back) = back {
                        compacted.push(*back);
                        front_next = input.next();
                    }
                    back_next = input.next_back();
                }
            };
        }
        if let Some(Some(front)) = front_next {
            compacted.push(*front);
        } else if let Some(Some(back)) = back_next {
            compacted.push(*back);
        }

        let res: u64 = compacted
            .iter()
            .enumerate()
            .map(|(idx, n)| (idx as u32 * n) as u64)
            .sum();

        println!("D9P1: {res}");
    }

    #[test]
    fn part2() {
        let input = load_inputs();

        let mut index = 0;
        let mut file_chunks: Vec<(u32, u32, u32)> = Vec::new();
        let mut empty_chunks: Vec<(u32, u32)> = Vec::new();
        for (idx, size) in input.iter().enumerate() {
            let size = *size as u32;
            if idx % 2 == 0 {
                let id = idx as u32 / 2;
                file_chunks.push((index, size, id));
            } else {
                empty_chunks.push((index, size));
            }
            index += size;
        }

        for file_chunk in file_chunks.iter_mut().rev() {
            for empty_chunk in &mut empty_chunks {
                if empty_chunk.0 >= file_chunk.0 {
                    break;
                } else if empty_chunk.1 >= file_chunk.1 {
                    file_chunk.0 = empty_chunk.0;
                    empty_chunk.0 += file_chunk.1;
                    empty_chunk.1 -= file_chunk.1;
                    break;
                }
            }
        }

        let res: u64 = file_chunks
            .iter()
            .map(|(index, size, id)| {
                let mut res = 0;
                for inc in 0..*size {
                    res += (index + inc) * id;
                }
                res as u64
            })
            .sum();

        println!("D9P2: {res}");
    }
}

mod day10 {
    use rtrb::RingBuffer;
    use std::{
        collections::{HashMap, HashSet},
        fs::File,
        io::{BufRead, BufReader},
    };

    const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    fn load_inputs() -> Vec<Vec<u32>> {
        let file = File::open("inputs/day10.txt").unwrap();
        let buf_reader = BufReader::new(file);
        buf_reader
            .lines()
            .map(|line| {
                line.unwrap()
                    .chars()
                    .map(|ch| ch.to_digit(10).unwrap())
                    .collect()
            })
            .collect()
    }

    fn find_score(topo: &Vec<Vec<u32>>, r_start: usize, c_start: usize) -> u64 {
        let max_r = topo.len();
        let max_c = topo[0].len();

        let mut ends: HashSet<(usize, usize, u32)> = HashSet::new();
        let mut checked_tiles: HashSet<(usize, usize, u32)> = HashSet::new();

        let (mut producer, mut consumer) = RingBuffer::new(300);
        producer
            .push((r_start, c_start, 0))
            .expect("failed to push to rb");

        while !consumer.is_empty() {
            let curr = consumer.pop().unwrap();
            if checked_tiles.contains(&curr) {
                continue;
            }
            checked_tiles.insert(curr);
            if curr.2 == 9 {
                ends.insert(curr);
                continue;
            } else {
                for dir in DIRS {
                    let r_next = curr.0.wrapping_add_signed(dir.0);
                    let c_next = curr.1.wrapping_add_signed(dir.1);
                    if r_next < max_r && c_next < max_c {
                        let val = topo[r_next][c_next];
                        if val == curr.2 + 1 {
                            producer
                                .push((r_next, c_next, val))
                                .expect("failed to push to rb");
                        }
                    }
                }
            }
        }

        ends.len() as u64
    }

    #[test]
    fn part1() {
        let topo = load_inputs();

        let res: u64 = topo
            .iter()
            .enumerate()
            .map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .map(|(c, n)| if *n == 0 { find_score(&topo, r, c) } else { 0 })
                    .sum::<u64>()
            })
            .sum();

        println!("D10P1: {res}");
    }

    fn find_rating(topo: &Vec<Vec<u32>>, r_start: usize, c_start: usize) -> u64 {
        let max_r = topo.len();
        let max_c = topo[0].len();

        let mut ends: HashMap<(usize, usize, u32), u64> = HashMap::new();
        // TODO could try adding checked_tiles, need to count the number of paths to that tiles
        // correctly though

        let (mut producer, mut consumer) = RingBuffer::new(300);
        producer
            .push((r_start, c_start, 0))
            .expect("failed to push to rb");

        while !consumer.is_empty() {
            let curr = consumer.pop().unwrap();
            if curr.2 == 9 {
                *ends.entry(curr).or_insert(0) += 1;
                continue;
            } else {
                for dir in DIRS {
                    let r_next = curr.0.wrapping_add_signed(dir.0);
                    let c_next = curr.1.wrapping_add_signed(dir.1);
                    if r_next < max_r && c_next < max_c {
                        let val = topo[r_next][c_next];
                        if val == curr.2 + 1 {
                            producer
                                .push((r_next, c_next, val))
                                .expect("failed to push to rb");
                        }
                    }
                }
            }
        }

        ends.values().sum()
    }

    #[test]
    fn part2() {
        let topo = load_inputs();

        let res: u64 = topo
            .iter()
            .enumerate()
            .map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .map(|(c, n)| if *n == 0 { find_rating(&topo, r, c) } else { 0 })
                    .sum::<u64>()
            })
            .sum();

        println!("D10P2: {res}");
    }
}

mod day11 {
    use std::collections::HashMap;

    fn load_inputs() -> Vec<u64> {
        include_str!("../inputs/day11.txt")
            .trim()
            .split(" ")
            .filter_map(|s| s.parse().ok())
            .collect()
    }

    fn blink(n: u64, remaining: u64, lut: &mut HashMap<(u64, u64), u64>) -> u64 {
        if remaining <= 0 {
            return 1;
        } else if let Some(res) = lut.get(&(n, remaining)) {
            *res
        } else {
            let res = if n == 0 {
                blink(1, remaining - 1, lut)
            } else {
                let n_str = format!("{}", n);
                if n_str.len() % 2 == 0 {
                    let (nl, nr) = n_str.split_at(n_str.len() / 2);
                    blink(nl.parse().unwrap(), remaining - 1, lut)
                        + blink(nr.parse().unwrap(), remaining - 1, lut)
                } else {
                    blink(n * 2024, remaining - 1, lut)
                }
            };
            lut.insert((n, remaining), res);
            res
        }
    }

    #[test]
    fn part1() {
        let inputs = load_inputs();

        let mut lut = HashMap::new();
        let res: u64 = inputs.iter().map(|n| blink(*n, 25, &mut lut)).sum();

        println!("D11P1: {res}");
    }

    #[test]
    fn part2() {
        let inputs = load_inputs();

        let mut lut = HashMap::new();

        let res: u64 = inputs.iter().map(|n| blink(*n, 75, &mut lut)).sum();

        println!("D11P2: {res}");
    }
}

mod day12 {
    use rtrb::RingBuffer;
    use std::{
        collections::{HashMap, HashSet},
        fs::File,
        io::{BufRead, BufReader},
        ops::RangeFrom,
    };

    const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    fn load_inputs() -> Vec<Vec<(char, bool, Vec<u32>)>> {
        let file = File::open("inputs/day12.txt").unwrap();
        let buf_reader = BufReader::new(file);
        buf_reader
            .lines()
            .map(|line| {
                line.unwrap()
                    .chars()
                    .map(|ch| (ch, false, Vec::new()))
                    .collect()
            })
            .collect()
    }

    fn check_area_by_perimeter(
        tiles: &mut Vec<Vec<(char, bool, Vec<u32>)>>,
        r_start: usize,
        c_start: usize,
    ) -> u32 {
        if tiles[r_start][c_start].1 {
            return 0;
        }

        let r_len = tiles.len();
        let c_len = tiles[0].len();

        let (mut producer, mut consumer) = RingBuffer::new(300);
        producer
            .push((r_start, c_start))
            .expect("failed to push to rb");

        let mut area = 0;
        let mut perimeter = 0;

        while !consumer.is_empty() {
            let (r, c) = consumer.pop().unwrap();
            let tile_id = {
                let tile = &mut tiles[r][c];
                if tile.1 {
                    continue;
                }
                tile.1 = true;
                tile.0
            };
            area += 1;
            for dir in DIRS {
                let r_test = r.wrapping_add_signed(dir.0);
                let c_test = c.wrapping_add_signed(dir.1);
                if r_test >= r_len || c_test >= c_len || tiles[r_test][c_test].0 != tile_id {
                    perimeter += 1;
                } else {
                    producer
                        .push((r_test, c_test))
                        .expect("failed to push to rb");
                }
            }
        }

        area * perimeter
    }

    #[test]
    fn part1() {
        let mut tiles = load_inputs();

        let mut price = 0;
        for r in 0..tiles.len() {
            for c in 0..tiles[0].len() {
                price += check_area_by_perimeter(&mut tiles, r, c);
            }
        }

        println!("D12P1: {price}");
    }

    fn check_area_by_num_sides(
        tiles: &mut Vec<Vec<(char, bool, Vec<u32>)>>,
        r_start: usize,
        c_start: usize,
    ) -> u32 {
        if tiles[r_start][c_start].1 {
            return 0;
        }

        let r_len = tiles.len();
        let c_len = tiles[0].len();

        let (mut producer, mut consumer) = RingBuffer::new(300);
        producer
            .push((r_start, c_start))
            .expect("failed to push to rb");

        let mut area = 0;
        let mut side_ids: HashSet<u32> = HashSet::new();

        while !consumer.is_empty() {
            let (r, c) = consumer.pop().unwrap();
            let tile_id = {
                let tile = &mut tiles[r][c];
                if tile.1 {
                    continue;
                }
                tile.1 = true;
                side_ids.extend(&tile.2);
                tile.0
            };
            area += 1;
            for dir in DIRS {
                let r_test = r.wrapping_add_signed(dir.0);
                let c_test = c.wrapping_add_signed(dir.1);
                if r_test < r_len && c_test < c_len && tiles[r_test][c_test].0 == tile_id {
                    producer
                        .push((r_test, c_test))
                        .expect("failed to push to rb");
                }
            }
        }

        let num_sides = side_ids.len() as u32;

        area * num_sides
    }

    fn update_tiles(
        coord: (usize, usize),
        coord_test: (usize, usize),
        tiles: &mut Vec<Vec<(char, bool, Vec<u32>)>>,
        side_id: &mut Option<u32>,
        side_ids: &mut RangeFrom<u32>,
    ) {
        if coord_test.0 >= tiles.len()
            || coord_test.1 >= tiles[0].len()
            || tiles[coord_test.0][coord_test.1].0 != tiles[coord.0][coord.1].0
        {
            if side_id.is_none() {
                *side_id = Some(side_ids.next().unwrap());
            }
            tiles[coord.0][coord.1].2.push(side_id.unwrap());
        } else {
            *side_id = None;
        }
    }

    #[test]
    fn part2() {
        let mut tiles = load_inputs();

        let r_len = tiles.len();
        let c_len = tiles[0].len();

        let mut prev_id = '.';

        let mut side_ids = 0..;
        for r in 0..r_len {
            let r_u = r.wrapping_add_signed(-1);
            let r_d = r.wrapping_add_signed(1);
            let mut u_id: Option<u32> = None;
            let mut d_id: Option<u32> = None;
            for c in 0..c_len {
                if tiles[r][c].0 != prev_id {
                    u_id = None;
                    d_id = None;
                }
                update_tiles((r, c), (r_u, c), &mut tiles, &mut u_id, &mut side_ids);
                update_tiles((r, c), (r_d, c), &mut tiles, &mut d_id, &mut side_ids);
                prev_id = tiles[r][c].0;
            }
        }
        prev_id = '.';
        for c in 0..c_len {
            let c_l = c.wrapping_add_signed(-1);
            let c_r = c.wrapping_add_signed(1);
            let mut l_id: Option<u32> = None;
            let mut r_id: Option<u32> = None;
            for r in 0..r_len {
                if tiles[r][c].0 != prev_id {
                    l_id = None;
                    r_id = None;
                }
                update_tiles((r, c), (r, c_l), &mut tiles, &mut r_id, &mut side_ids);
                update_tiles((r, c), (r, c_r), &mut tiles, &mut l_id, &mut side_ids);
                prev_id = tiles[r][c].0;
            }
        }

        let mut price = 0;
        for r in 0..tiles.len() {
            for c in 0..tiles[0].len() {
                price += check_area_by_num_sides(&mut tiles, r, c);
            }
        }

        println!("D12P2: {price}");
    }
}

mod day13 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        ops::Sub,
    };

    #[derive(PartialEq, PartialOrd, Clone, Copy)]
    struct Point {
        x: i64,
        y: i64,
    }

    impl Point {
        fn new(x: i64, y: i64) -> Self {
            Self { x, y }
        }
    }

    impl Sub for Point {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    fn min(n1: u64, n2: u64) -> u64 {
        if n1 < n2 {
            n1
        } else {
            n2
        }
    }

    fn load_inputs() -> Vec<(Point, Point, Point)> {
        let file = File::open("inputs/day13.txt").unwrap();
        let buf_reader = BufReader::new(file);
        let mut buf_reader_lines = buf_reader.lines().peekable();

        let mut machines = Vec::new();

        while buf_reader_lines.peek().is_some() {
            machines.push((
                get_point(&buf_reader_lines.next().unwrap().unwrap()),
                get_point(&buf_reader_lines.next().unwrap().unwrap()),
                get_point(&buf_reader_lines.next().unwrap().unwrap()),
            ));
            buf_reader_lines.next();
        }

        machines
    }

    fn get_point(s: &str) -> Point {
        let s = s.rsplit_once(": ").unwrap().1.trim();
        let splitter = if s.contains("+") { "+" } else { "=" };
        let (xs, ys) = s.rsplit_once(", ").unwrap();
        let x = xs.rsplit_once(splitter).unwrap().1.parse().unwrap();
        let y = ys.rsplit_once(splitter).unwrap().1.parse().unwrap();
        Point::new(x, y)
    }

    fn check_machine1(a: Point, b: Point, target: Point) -> Option<u64> {
        let mut a_count = 0;
        let mut test = target;
        while test > Point::new(0, 0) {
            if test.x % b.x == 0 && test.y % b.y == 0 {
                let b_count = test.x / b.x;
                let b_count_check = test.y / b.y;
                if b_count == b_count_check {
                    let test_min = (a_count * 3 + b_count) as u64;
                    return Some(test_min);
                }
            }
            a_count += 1;
            test = test - a;
        }

        None
    }

    #[test]
    fn part1() {
        let inputs = load_inputs();

        let res: u64 = inputs
            .iter()
            .map(|(a, b, target)| check_machine1(*a, *b, *target).unwrap_or(0))
            .sum();

        println!("D13P1: {res}");
    }

    fn check_machine2(a: Point, b: Point, target: Point) -> Option<u64> {
        let target = Point::new(target.x + 10000000000000, target.y + 10000000000000);

        let a_x = a.x as f64;
        let a_y = a.y as f64;
        let b_x = b.x as f64;
        let b_y = b.y as f64;
        let t_x = target.x as f64;
        let t_y = target.y as f64;

        let b_count = ((t_y - a_y * t_x / a_x) / (b_y - a_y * b_x / a_x)).round() as i64;
        let a_count = ((t_x - b_x * b_count as f64) / a_x).round() as i64;

        if (a.x * a_count + b.x * b_count == target.x)
            && (a.y * a_count + b.y * b_count == target.y)
            && a_count > 0
            && b_count > 0
        {
            Some((a_count * 3 + b_count) as u64)
        } else {
            None
        }
    }

    #[test]
    fn part2() {
        let inputs = load_inputs();

        let res: u64 = inputs
            .iter()
            .map(|(a, b, target)| check_machine2(*a, *b, *target).unwrap_or(0))
            .sum();

        println!("D13P2: {res}");
    }
}

mod day14 {
    use std::{
        fs::File,
        io::{BufRead, BufReader},
        ops::{Add, Mul},
        thread::sleep,
        time::Duration,
    };

    const X: u64 = 101;
    const Y: u64 = 103;

    #[derive(Copy, Clone, Debug)]
    struct Point {
        x: i64,
        y: i64,
    }

    impl Point {
        fn new(x: i64, y: i64) -> Self {
            Self { x, y }
        }
    }

    impl Add<Velocity> for Point {
        type Output = Self;
        fn add(self, rhs: Velocity) -> Self::Output {
            let x = X as i64;
            let y = Y as i64;
            let added = Self::new(
                (self.x + rhs.x).rem_euclid(x),
                (self.y + rhs.y).rem_euclid(y),
            );
            added
        }
    }

    #[derive(PartialEq, PartialOrd, Clone, Copy)]
    struct Velocity {
        x: i64,
        y: i64,
    }

    impl Velocity {
        fn new(x: i64, y: i64) -> Self {
            Self { x, y }
        }
    }

    impl Mul<i64> for Velocity {
        type Output = Self;
        fn mul(self, rhs: i64) -> Self::Output {
            Self {
                x: self.x * rhs,
                y: self.y * rhs,
            }
        }
    }

    fn load_inputs() -> Vec<(Point, Velocity)> {
        let file = File::open("inputs/day14.txt").unwrap();
        let buf_reader = BufReader::new(file);

        buf_reader
            .lines()
            .map(|s| {
                let s = s.unwrap();
                let (sp, sv) = s.trim().rsplit_once(" ").unwrap();
                let (px, py) = sp.rsplit_once("=").unwrap().1.rsplit_once(",").unwrap();
                let (vx, vy) = sv.rsplit_once("=").unwrap().1.rsplit_once(",").unwrap();
                (
                    Point::new(px.parse().unwrap(), py.parse().unwrap()),
                    Velocity::new(vx.parse().unwrap(), vy.parse().unwrap()),
                )
            })
            .collect()
    }

    fn safety_factor(final_positions: &Vec<Point>, min_x: u64, min_y: u64) -> u64 {
        let max_x = min_x + (X / 2) - 1;
        let max_y = min_y + (Y / 2) - 1;
        final_positions
            .iter()
            .filter(|p| {
                let x = p.x as u64;
                let y = p.y as u64;
                min_x <= x && x <= max_x && min_y <= y && y <= max_y
            })
            .count() as u64
    }

    #[test]
    fn part1() {
        let pvs = load_inputs();

        let final_positions: Vec<Point> = pvs
            .iter()
            .map(|(p, v)| {
                let final_pos = *p + (*v * 100);
                final_pos
            })
            .collect();

        let res = safety_factor(&final_positions, 0, 0)
            * safety_factor(&final_positions, X / 2 + 1, 0)
            * safety_factor(&final_positions, 0, Y / 2 + 1)
            * safety_factor(&final_positions, X / 2 + 1, Y / 2 + 1);

        println!("D14P1: {res}");
    }

    fn display(pvs: &Vec<(Point, Velocity)>) {
        let mut tiles: Vec<Vec<u32>> = (0..X).map(|_| vec![0; Y as usize]).collect();
        for pv in pvs {
            tiles[pv.0.x as usize][pv.0.y as usize] += 1;
        }

        for row in tiles {
            let display_row: String = row.iter().map(|n| if *n > 0 { '*' } else { ' ' }).collect();
            println!("{:?}", display_row);
        }
    }

    #[test]
    fn part2() {
        let mut pvs = load_inputs();

        let run = false;
        if !run {
            println!("D14P2: -");
            return;
        }

        let mut count = 1;
        loop {
            pvs = pvs.iter().map(|(p, v)| (*p + *v, *v)).collect();
            let poss = pvs.iter().map(|e| e.0).collect();
            let res = safety_factor(&poss, 0, 0)
                * safety_factor(&poss, X / 2 + 1, 0)
                * safety_factor(&poss, 0, Y / 2 + 1)
                * safety_factor(&poss, X / 2 + 1, Y / 2 + 1);
            if res < 100_000_000 {
                println!("-----------------  {count}  -----------------");
                display(&pvs);
                sleep(Duration::from_millis(100));
            }
            count += 1;
        }
    }
}

mod day15 {
    use std::{
        cmp::Ordering,
        collections::HashMap,
        fs::File,
        io::{BufRead, BufReader},
    };

    fn load_inputs() -> (Vec<Vec<char>>, String) {
        let file = File::open("inputs/day15.txt").unwrap();
        let buf_reader = BufReader::new(file);

        let mut buf_reader_lines = buf_reader.lines();
        let tiles = buf_reader_lines
            .by_ref()
            .map_while(|s| {
                let s = s.unwrap();
                let s = s.trim();
                if s.len() == 0 {
                    None
                } else {
                    Some(s.chars().collect())
                }
            })
            .collect();
        let moves: String = buf_reader_lines
            .flat_map(|s| {
                let s = s.unwrap();
                s.chars().collect::<Vec<char>>()
            })
            .collect();

        (tiles, moves)
    }

    fn display_tiles(tiles: &Vec<Vec<char>>) {
        for row in tiles {
            let row: String = row.iter().collect();
            println!("{}", row);
        }
    }

    fn get_starting_pos(tiles: &Vec<Vec<char>>) -> (usize, usize) {
        for r in 0..tiles.len() {
            for c in 0..tiles[0].len() {
                if tiles[r][c] == '@' {
                    return (r, c);
                }
            }
        }
        panic!("@ not found");
    }

    #[test]
    fn part1() {
        let (mut tiles, moves) = load_inputs();

        let mut pos = get_starting_pos(&tiles);

        'outer: for (move_num, m) in moves.chars().enumerate() {
            // display_tiles(&tiles);
            // println!("---------------- {move_num} {m} ----------------");
            let dir = match m {
                '<' => (0, -1),
                '>' => (0, 1),
                '^' => (-1, 0),
                'v' => (1, 0),
                m => panic!("invalid move {m}"),
            };
            let mut target_pos = (
                pos.0.wrapping_add_signed(dir.0),
                pos.1.wrapping_add_signed(dir.1),
            );
            let new_robot_pos = target_pos;
            let mut new_box_pos: Option<(usize, usize)> = None;

            // TODO try while let here
            loop {
                match tiles[target_pos.0][target_pos.1] {
                    '#' => continue 'outer,
                    '.' => break,
                    'O' => {
                        target_pos = (
                            target_pos.0.wrapping_add_signed(dir.0),
                            target_pos.1.wrapping_add_signed(dir.1),
                        );
                        new_box_pos = Some(target_pos);
                    }
                    ch => panic!("invalid char {ch} at position {target_pos:?}"),
                }
            }
            tiles[pos.0][pos.1] = '.';
            tiles[new_robot_pos.0][new_robot_pos.1] = '@';
            if let Some(new_box_pos) = new_box_pos {
                tiles[new_box_pos.0][new_box_pos.1] = 'O';
            }
            pos = new_robot_pos;
        }

        // display_tiles(&tiles);

        let res: usize = tiles
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(c, ch)| if *ch == 'O' { r * 100 + c } else { 0 })
            })
            .sum();

        println!("D15P1: {res}");
    }

    fn widen(tiles: Vec<Vec<char>>) -> Vec<Vec<char>> {
        tiles
            .iter()
            .map(|row| {
                row.iter()
                    .flat_map(|ch| {
                        match ch {
                            '@' => "@.".to_string(),
                            'O' => "[]".to_string(),
                            ch => format!("{ch}{ch}"),
                        }
                        .chars()
                        .collect::<Vec<char>>()
                    })
                    .collect()
            })
            .collect()
    }

    fn check_vertical_move(
        tiles: &Vec<Vec<char>>,
        curr_pos: (usize, usize),
        dir_v: isize,
    ) -> Result<Option<HashMap<(usize, usize), (usize, usize)>>, ()> {
        let target_pos_1 = (curr_pos.0.wrapping_add_signed(dir_v), curr_pos.1);
        let target_pos_2 = match tiles[curr_pos.0][curr_pos.1] {
            '.' => return Ok(None),
            '#' => return Err(()),
            '@' => None,
            '[' => Some((target_pos_1.0, target_pos_1.1 + 1)),
            ']' => Some((target_pos_1.0, target_pos_1.1 - 1)),
            ch => panic!("invalid char {ch} at position {curr_pos:?}"),
        };
        let mut hm = check_vertical_move(tiles, target_pos_1, dir_v)?.unwrap_or_default();
        hm.insert(curr_pos, target_pos_1);
        if let Some(target_pos_2) = target_pos_2 {
            if let Some(hm_r) = check_vertical_move(tiles, target_pos_2, dir_v)? {
                hm.extend(hm_r);
            }
            hm.insert((curr_pos.0, target_pos_2.1), target_pos_2);
        }
        Ok(Some(hm))
    }

    fn check_horizontal_move(
        tiles: &Vec<Vec<char>>,
        curr_pos: (usize, usize),
        dir_h: isize,
    ) -> Result<usize, ()> {
        let mut final_idx = curr_pos.1.wrapping_add_signed(dir_h);
        loop {
            match tiles[curr_pos.0][final_idx] {
                '#' => return Err(()),
                '.' => return Ok(final_idx),
                '[' | ']' => final_idx = final_idx.wrapping_add_signed(dir_h),
                ch => {
                    let invalid_pos = (final_idx, curr_pos.1);
                    panic!("invalid char {ch} at position {invalid_pos:?}")
                }
            }
        }
    }

    #[test]
    fn part2() {
        let (tiles, moves) = load_inputs();
        let mut tiles = widen(tiles);

        let mut curr_pos = get_starting_pos(&tiles);

        for (move_num, m) in moves.chars().enumerate() {
            // display_tiles(&tiles);
            // println!("---------------- {move_num} {m} ----------------");
            let dir = match m {
                '<' => (0, -1),
                '>' => (0, 1),
                '^' => (-1, 0),
                'v' => (1, 0),
                m => panic!("invalid move {m}"),
            };
            // TODO could probably clean this up
            if dir.0 == 0 {
                if let Ok(mut final_idx) = check_horizontal_move(&tiles, curr_pos, dir.1) {
                    while final_idx != curr_pos.1 {
                        let prev_idx = final_idx.wrapping_add_signed(-dir.1);
                        tiles[curr_pos.0][final_idx] = tiles[curr_pos.0][prev_idx];
                        final_idx = prev_idx;
                    }
                    tiles[curr_pos.0][curr_pos.1] = '.';
                    curr_pos = (
                        curr_pos.0.wrapping_add_signed(dir.0),
                        curr_pos.1.wrapping_add_signed(dir.1),
                    );
                }
            } else if let Ok(Some(pos_updates)) = check_vertical_move(&tiles, curr_pos, dir.0) {
                let mut keys: Vec<&(usize, usize)> = pos_updates.keys().collect();
                keys.sort_by(|pos1, pos2| {
                    let (pos1, pos2) = if dir.0 > 0 {
                        (pos2, pos1)
                    } else {
                        (pos1, pos2)
                    };
                    match pos1.0.cmp(&pos2.0) {
                        Ordering::Equal => pos1.1.cmp(&pos2.1),
                        o => o,
                    }
                });
                for old_pos in keys {
                    let new_pos = pos_updates[old_pos];
                    tiles[new_pos.0][new_pos.1] = tiles[old_pos.0][old_pos.1];
                    tiles[old_pos.0][old_pos.1] = '.';
                }
                curr_pos = (
                    curr_pos.0.wrapping_add_signed(dir.0),
                    curr_pos.1.wrapping_add_signed(dir.1),
                );
            }
        }

        // display_tiles(&tiles);

        let res: usize = tiles
            .iter()
            .enumerate()
            .flat_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(c, ch)| if *ch == '[' { r * 100 + c } else { 0 })
            })
            .sum();

        println!("D15P2: {res}");
    }
}

mod day16 {
    use std::{
        collections::{HashMap, HashSet},
        fs::File,
        io::{BufRead, BufReader},
    };

    use rtrb::RingBuffer;

    const DIRS: [(isize, isize); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

    #[derive(Debug)]
    struct Node {
        pos: (usize, usize),
        score: u32,
        dir_idx: usize,
    }

    impl Node {
        fn new(pos: (usize, usize), score: u32, dir_idx: usize) -> Self {
            Self {
                pos,
                score,
                dir_idx,
            }
        }
    }

    fn load_inputs() -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
        let file = File::open("inputs/day16.txt").unwrap();
        let buf_reader = BufReader::new(file);

        let buf_reader_lines = buf_reader.lines();
        let tiles: Vec<Vec<char>> = buf_reader_lines
            .map(|s| s.unwrap().chars().collect())
            .collect();

        let mut start = (0, 0);
        let mut end = (0, 0);
        for r in 0..tiles.len() {
            for c in 0..tiles[0].len() {
                match tiles[r][c] {
                    'S' => start = (r, c),
                    'E' => end = (r, c),
                    _ => (),
                }
            }
        }

        (tiles, start, end)
    }

    fn walk_maze(
        tiles: Vec<Vec<char>>,
        start: (usize, usize),
    ) -> HashMap<(usize, usize), HashMap<usize, u32>> {
        let (mut producer, mut consumer) = RingBuffer::new(1000);
        producer
            .push(Node::new(start, 0u32, 0usize))
            .expect("failed to push to rb");

        let mut checked_tiles: HashMap<(usize, usize), HashMap<usize, u32>> = HashMap::new();
        while !consumer.is_empty() {
            let curr = consumer.pop().unwrap();

            let tile_scores = checked_tiles.entry(curr.pos).or_default();
            let score_for_dir = tile_scores.entry(curr.dir_idx).or_insert(u32::MAX);
            if *score_for_dir <= curr.score {
                continue;
            }

            if tiles[curr.pos.0][curr.pos.1] != 'E' {
                for idx_inc in -1..=1 {
                    let dir_idx =
                        (curr.dir_idx as isize + idx_inc).rem_euclid(DIRS.len() as isize) as usize;

                    let dir = DIRS[dir_idx];
                    let next_pos = (
                        curr.pos.0.wrapping_add_signed(dir.0),
                        curr.pos.1.wrapping_add_signed(dir.1),
                    );
                    if tiles[next_pos.0][next_pos.1] == '#' {
                        continue;
                    }
                    let cost = 1 + (idx_inc as u32 % 2) * 1000;
                    producer
                        .push(Node::new(next_pos, curr.score + cost, dir_idx))
                        .expect("failed to push to rb");
                }
            }

            tile_scores.insert(curr.dir_idx, curr.score);
        }

        checked_tiles
    }

    fn get_min_score(
        checked_tiles: &HashMap<(usize, usize), HashMap<usize, u32>>,
        end: (usize, usize),
    ) -> u32 {
        let dir_to_score = checked_tiles.get(&end).expect("end not reached");
        let min_score = dir_to_score
            .values()
            .min()
            .expect("end reached without score");
        *min_score
    }

    #[test]
    fn part1() {
        let (tiles, start, end) = load_inputs();

        let checked_tiles = walk_maze(tiles, start);
        let min_score = get_min_score(&checked_tiles, end);

        println!("D16P1: {min_score}");
    }

    fn get_num_best_path_tiles(
        checked_tiles: HashMap<(usize, usize), HashMap<usize, u32>>,
        end: (usize, usize),
    ) -> u32 {
        let mut best_path_tiles: HashSet<(usize, usize)> = HashSet::new();

        let min_score = get_min_score(&checked_tiles, end);

        let (mut producer, mut consumer) = RingBuffer::new(100);
        let dir_to_score = checked_tiles.get(&end).expect("node not found");
        for (dir_idx, score) in dir_to_score {
            if *score == min_score {
                producer
                    .push(Node::new(end, min_score + 1, *dir_idx))
                    .expect("failed to push to rb");
            }
        }

        while !consumer.is_empty() {
            let node = consumer.pop().unwrap();
            let dir_to_score = checked_tiles.get(&node.pos).expect("node not found");
            for (dir_idx, dir_score) in dir_to_score {
                let dir = DIRS[*dir_idx];

                let offset = if *dir_idx == node.dir_idx { 1 } else { 1001 };
                if *dir_score != node.score - offset {
                    continue;
                }

                best_path_tiles.insert(node.pos);
                let prev_pos = (
                    node.pos.0.wrapping_add_signed(-dir.0),
                    node.pos.1.wrapping_add_signed(-dir.1),
                );
                if *dir_score > 0 {
                    producer
                        .push(Node::new(prev_pos, *dir_score, *dir_idx))
                        .expect("failed to push to rb");
                }
            }
        }

        best_path_tiles.len() as u32
    }

    #[test]
    fn part2() {
        let (tiles, start, end) = load_inputs();

        let checked_tiles = walk_maze(tiles, start);
        let num_best_path_tiles = get_num_best_path_tiles(checked_tiles, end);

        println!("D16P2: {num_best_path_tiles}");
    }
}

mod day17 {
    use std::{
        collections::{HashMap, HashSet},
        fs::File,
        io::{BufRead, BufReader},
    };

    fn load_inputs() -> Cpu {
        let file = File::open("inputs/day17.txt").unwrap();
        let buf_reader = BufReader::new(file);

        let mut buf_reader_lines = buf_reader.lines();
        let a = get_split_line(buf_reader_lines.next().unwrap().unwrap())
            .parse()
            .unwrap();
        let b = get_split_line(buf_reader_lines.next().unwrap().unwrap())
            .parse()
            .unwrap();
        let c = get_split_line(buf_reader_lines.next().unwrap().unwrap())
            .parse()
            .unwrap();

        buf_reader_lines.next();

        let program = get_split_line(buf_reader_lines.next().unwrap().unwrap())
            .split(",")
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        Cpu::new(a, b, c, program)
    }

    fn get_split_line(l: String) -> String {
        l.rsplit_once(":").unwrap().1.trim().to_string()
    }

    struct Cpu {
        rega: u64,
        regb: u64,
        regc: u64,
        iptr: usize,
        program: Vec<u64>,
    }

    impl Cpu {
        fn new(rega: u64, regb: u64, regc: u64, program: Vec<u64>) -> Self {
            Self {
                rega,
                regb,
                regc,
                iptr: 0,
                program,
            }
        }

        fn combo(&self, operand: u64) -> u64 {
            match operand {
                n if n <= 3 => n,
                4 => self.rega,
                5 => self.regb,
                6 => self.regc,
                invalid => panic!("invalid operand: {invalid}"),
            }
        }

        fn cycle(&mut self) -> (usize, Option<u64>) {
            let opcode = self.program[self.iptr];
            let operand = self.program[self.iptr + 1];
            let mut next_iptr = self.iptr + 2;

            let mut output: Option<u64> = None;

            match opcode {
                0 => {
                    // adv
                    self.rega /= 2u64.pow(self.combo(operand) as u32);
                }
                1 => {
                    // bxl
                    self.regb ^= operand;
                }
                2 => {
                    //bst
                    self.regb = self.combo(operand) % 8;
                }
                3 => {
                    // jnz
                    if self.rega != 0 {
                        next_iptr = operand as usize;
                    }
                }
                4 => {
                    // bxc
                    self.regb ^= self.regc;
                }
                5 => {
                    // out
                    output = Some(self.combo(operand) % 8);
                }
                6 => {
                    // bdv
                    self.regb = self.rega / 2u64.pow(self.combo(operand) as u32);
                }
                7 => {
                    // cdv
                    self.regc = self.rega / 2u64.pow(self.combo(operand) as u32);
                }
                invalid => panic!("invalid opcode: {invalid}"),
            }

            (next_iptr, output)
        }

        fn run(mut self) -> Vec<u64> {
            let mut output: Vec<u64> = Vec::new();

            while self.iptr < self.program.len() {
                let (next_iptr, out) = self.cycle();
                self.iptr = next_iptr;
                if let Some(out) = out {
                    output.push(out);
                }
            }

            output
        }

        fn optimize(mut self) -> u64 {
            let mut test: HashSet<u64> = HashSet::new();

            let bases: Vec<u64> = vec![
                0b1011011010110111010111101,
                0b1011011010110111110111101,
                0b0111011010110111010111101,
                0b0111011010110111110111101,
                0b1111011010110111010111101,
                0b1111011010110111110111101,
                0b0110100100111100110110101,
            ];

            let len: u64 = 3 * 8 + 1;

            for a in 0..8 {
                // for base in &bases {
                let rega = a; // = (a << len) + base;
                self.rega = rega;
                self.regb = 0;
                self.regc = 0;

                let mut output_idx = 0;
                self.iptr = 0;

                while self.iptr < self.program.len() {
                    let (next_iptr, out) = self.cycle();
                    self.iptr = next_iptr;
                    match out {
                        Some(out) => {
                            println!("!!! {a} {out}");
                            // if out != self.program[output_idx] {
                            //     break;
                            // }
                            // output_idx += 1;
                            // if output_idx >= 1 {
                            //     let mask = rega & 0b111_111_111_111_111_111_111_111_111_111;
                            //     if !test.contains(&mask) {
                            //         println!("0b{:b}", mask);
                            //         test.insert(mask);
                            //     }
                            // }
                            // if output_idx == self.program.len() {
                            //     return rega;
                            // }
                        }
                        None => (),
                    }
                }
                // }
            }

            panic!("!!!");
        }
    }

    #[test]
    fn part1() {
        let cpu = load_inputs();
        let output = cpu.run();
        println!("D17P1: {}", format!("{:?}", output).replace(" ", ""));
    }

    #[test]
    fn part2() {
        let run = false;
        let res = if run {
            let cpu = load_inputs();
            cpu.optimize().to_string()
        } else {
            "-".to_string()
        };

        println!("D17P2: {res}");
    }
}

mod day18 {
    use std::collections::HashSet;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use rtrb::RingBuffer;

    fn load_inputs() -> Vec<(usize, usize)> {
        let file = File::open("inputs/day18.txt").unwrap();
        let buf_reader = BufReader::new(file);
        buf_reader
            .lines()
            .map(|line| {
                let line = line.unwrap();
                let (r, c) = line.rsplit_once(",").unwrap();
                (r.parse().unwrap(), c.parse().unwrap())
            })
            .collect()
    }

    const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    fn find_exit(tiles: &Vec<Vec<bool>>) -> Result<u64, ()> {
        let len = tiles.len();

        let mut checked_tiles: HashSet<(usize, usize)> = HashSet::new();

        let (mut producer, mut consumer) = RingBuffer::new(300);
        producer.push(((0, 0), 0)).expect("failed to push to rb");

        while !consumer.is_empty() {
            let (pos, count) = consumer.pop().unwrap();
            if checked_tiles.contains(&pos) {
                continue;
            }
            checked_tiles.insert(pos);
            if pos == (len - 1, len - 1) {
                return Ok(count);
            }
            for dir in DIRS {
                let r_next = pos.0.wrapping_add_signed(dir.0);
                let c_next = pos.1.wrapping_add_signed(dir.1);
                if r_next < len && c_next < len && !tiles[r_next][c_next] {
                    producer
                        .push(((r_next, c_next), count + 1))
                        .expect("failed to push to rb");
                }
            }
        }

        Err(())
    }

    #[test]
    fn part1() {
        let coords = load_inputs();

        let mut tiles: Vec<Vec<bool>> = vec![vec![false; 71]; 71];

        let sim = 1024;

        for idx in 0..sim {
            let coord = coords[idx];
            tiles[coord.0][coord.1] = true;
        }

        let count = find_exit(&tiles).unwrap();

        println!("D18P1: {count}");
    }

    #[test]
    fn part2() {
        let coords = load_inputs();

        let base_tiles: Vec<Vec<bool>> = vec![vec![false; 71]; 71];

        let mut upper_byte_idx = coords.len() - 1;
        let mut lower_byte_idx = 0;

        let res = loop {
            let byte_idx = (upper_byte_idx + lower_byte_idx) / 2;

            let mut tiles = base_tiles.clone();
            for idx in 0..=byte_idx {
                let coord = coords[idx];
                tiles[coord.0][coord.1] = true;
            }

            if find_exit(&tiles).is_ok() {
                lower_byte_idx = byte_idx;
            } else {
                upper_byte_idx = byte_idx;
            }
            if lower_byte_idx >= upper_byte_idx - 1 {
                break coords[upper_byte_idx];
            }
        };

        println!("D18P2: {},{}", res.0, res.1);
    }
}

mod day19 {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    fn load_inputs() -> (Vec<String>, Vec<String>) {
        let file = File::open("inputs/day19.txt").unwrap();
        let buf_reader = BufReader::new(file);
        let mut buf_reader_lines = buf_reader.lines();

        let patterns = buf_reader_lines
            .next()
            .unwrap()
            .unwrap()
            .trim()
            .split(", ")
            .map(|s| s.to_string())
            .collect();

        buf_reader_lines.next();

        let designs = buf_reader_lines.map(|s| s.unwrap()).collect();

        (patterns, designs)
    }

    fn is_valid(design: &str, patterns: &Vec<String>) -> bool {
        if design.len() == 0 {
            return true;
        }

        for p in patterns {
            match design.split_once(p) {
                Some((d1, d2)) if d1.len() == 0 => {
                    if is_valid(d2, &patterns) {
                        return true;
                    }
                }
                _ => continue,
            }
        }

        false
    }

    #[test]
    fn part1() {
        let (patterns, designs) = load_inputs();
        let res: u32 = designs.iter().map(|d| is_valid(d, &patterns) as u32).sum();
        println!("D19P1: {res}");
    }

    fn num_arrangements<'a>(
        design: &'a str,
        patterns: &Vec<&String>,
        memo: &mut HashMap<&'a str, u64>,
    ) -> u64 {
        if design.len() == 0 {
            return 1;
        }

        if let Some(count) = memo.get(design) {
            return *count;
        }

        let mut count = 0;

        for p in patterns {
            match design.split_once(*p) {
                Some(("", d2)) => {
                    count += num_arrangements(d2, &patterns, memo);
                }
                _ => continue,
            }
        }

        memo.insert(design, count);

        count
    }

    #[test]
    fn part2() {
        let (patterns, designs) = load_inputs();
        let res: u64 = designs
            .iter()
            .map(|d| {
                let filtered_patterns: Vec<&String> =
                    patterns.iter().filter(|p| d.contains(*p)).collect();
                num_arrangements(d, &filtered_patterns, &mut HashMap::new()) as u64
            })
            .sum();
        println!("D19P2: {res}");
    }
}

mod day20 {
    use std::{
        collections::HashMap,
        fs::File,
        io::{BufRead, BufReader},
    };

    const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    fn load_inputs() -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
        let file = File::open("inputs/day20.txt").unwrap();
        let buf_reader = BufReader::new(file);

        let buf_reader_lines = buf_reader.lines();
        let tiles: Vec<Vec<char>> = buf_reader_lines
            .map(|s| s.unwrap().chars().collect())
            .collect();

        let mut start = (0, 0);
        let mut end = (0, 0);
        for r in 0..tiles.len() {
            for c in 0..tiles[0].len() {
                match tiles[r][c] {
                    'S' => start = (r, c),
                    'E' => end = (r, c),
                    _ => (),
                }
            }
        }

        (tiles, start, end)
    }

    fn index_path(
        tiles: Vec<Vec<char>>,
        start: (usize, usize),
        end: (usize, usize),
    ) -> HashMap<(usize, usize), u32> {
        let mut path_idxs: HashMap<(usize, usize), u32> = HashMap::new();

        let mut idx = 0;

        let mut pos = start;
        path_idxs.insert(pos, idx);
        'outer: while pos != end {
            for dir in DIRS {
                let new_pos = (
                    pos.0.wrapping_add_signed(dir.0),
                    pos.1.wrapping_add_signed(dir.1),
                );
                if new_pos.0 < tiles.len()
                    && new_pos.1 < tiles[0].len()
                    && tiles[new_pos.0][new_pos.1] != '#'
                    && !path_idxs.contains_key(&new_pos)
                {
                    pos = new_pos;
                    idx += 1;
                    path_idxs.insert(pos, idx);
                    continue 'outer;
                }
            }
            panic!("next pos not found");
        }

        path_idxs
    }

    #[test]
    fn part1() {
        let (tiles, start, end) = load_inputs();

        let indexed_tiles = index_path(tiles, start, end);

        let res: u32 = indexed_tiles
            .iter()
            .map(|(pos, idx)| {
                DIRS.iter()
                    .map(|dir| {
                        let cheat_dur = 2;
                        let new_pos = (
                            pos.0.wrapping_add_signed(dir.0 * cheat_dur),
                            pos.1.wrapping_add_signed(dir.1 * cheat_dur),
                        );
                        if let Some(new_idx) = indexed_tiles.get(&new_pos) {
                            let time_save = new_idx.saturating_sub(*idx + cheat_dur as u32);
                            (time_save >= 100) as u32
                        } else {
                            0
                        }
                    })
                    .sum::<u32>()
            })
            .sum();

        println!("D20P1: {res}");
    }

    #[test]
    fn part2() {
        let (tiles, start, end) = load_inputs();

        let indexed_tiles = index_path(tiles, start, end);

        // TODO how to improve this?
        let res: u32 = indexed_tiles
            .iter()
            .map(|(pos, idx)| {
                indexed_tiles
                    .iter()
                    .map(|(new_pos, new_idx)| {
                        let r_diff = ((new_pos.0 as i32) - (pos.0 as i32)).abs();
                        let c_diff = ((new_pos.1 as i32) - (pos.1 as i32)).abs();
                        let cheat_dur = c_diff + r_diff;
                        if cheat_dur <= 20 {
                            let time_save = new_idx.saturating_sub(*idx + cheat_dur as u32);
                            (time_save >= 100) as u32
                        } else {
                            0
                        }
                    })
                    .sum::<u32>()
            })
            .sum();

        println!("D20P2: {res}");
    }
}

mod day21 {
    use std::{
        collections::HashMap,
        fs::File,
        io::{BufRead, BufReader},
        iter,
    };

    fn load_inputs() -> Vec<String> {
        let file = File::open("inputs/day21.txt").unwrap();
        let buf_reader = BufReader::new(file);
        buf_reader.lines().map(|s| s.unwrap()).collect()
    }

    const KEYPAD: [[char; 3]; 4] = [
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        ['_', '0', 'A'],
    ];

    const CONTROL_PAD: [[char; 3]; 2] = [['_', '^', 'A'], ['<', 'v', '>']];

    fn get_coords() -> (HashMap<char, (usize, usize)>, HashMap<char, (usize, usize)>) {
        let keypad_coords: HashMap<char, (usize, usize)> = KEYPAD
            .iter()
            .enumerate()
            .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, ch)| (*ch, (r, c))))
            .collect();

        let control_pad_coords: HashMap<char, (usize, usize)> = CONTROL_PAD
            .iter()
            .enumerate()
            .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, ch)| (*ch, (r, c))))
            .collect();

        (keypad_coords, control_pad_coords)
    }

    fn get_controls(
        curr_ch: char,
        next_ch: char,
        btn_coords: &HashMap<char, (usize, usize)>,
    ) -> (Vec<char>, Vec<char>) {
        let mut h_btns: Vec<char> = Vec::new();
        let mut v_btns: Vec<char> = Vec::new();

        let empty_coords = btn_coords[&'_'];

        let coords = btn_coords[&curr_ch];
        let next_coords = btn_coords[&next_ch];
        let v_dist = next_coords.0 as isize - coords.0 as isize;
        let h_dist = next_coords.1 as isize - coords.1 as isize;
        let v_ch = match v_dist {
            n if n > 0 => 'v',
            _ => '^',
        };
        let h_ch = match h_dist {
            n if n > 0 => '>',
            _ => '<',
        };
        if coords.1 == empty_coords.1 && coords.0.wrapping_add_signed(v_dist) == empty_coords.0 {
            v_btns.extend(iter::repeat(h_ch).take(h_dist.abs() as usize));
            v_btns.extend(iter::repeat(v_ch).take(v_dist.abs() as usize));
        } else {
            v_btns.extend(iter::repeat(v_ch).take(v_dist.abs() as usize));
            v_btns.extend(iter::repeat(h_ch).take(h_dist.abs() as usize));
        }
        v_btns.push('A');

        if coords.0 == empty_coords.0 && coords.1.wrapping_add_signed(h_dist) == empty_coords.1 {
            h_btns.extend(iter::repeat(v_ch).take(v_dist.abs() as usize));
            h_btns.extend(iter::repeat(h_ch).take(h_dist.abs() as usize));
        } else {
            h_btns.extend(iter::repeat(h_ch).take(h_dist.abs() as usize));
            h_btns.extend(iter::repeat(v_ch).take(v_dist.abs() as usize));
        }
        h_btns.push('A');

        (h_btns, v_btns)
    }

    fn find_shortest_seq_len_keypad(
        code: Vec<char>,
        keypad_coords: &HashMap<char, (usize, usize)>,
        control_pad_coords: &HashMap<char, (usize, usize)>,
        depth: u64,
    ) -> u64 {
        let mut curr_ch = 'A';
        let mut len = 0;

        let mut memo: HashMap<(char, char, u64), u64> = HashMap::new();

        for next_ch in code {
            let (h_btns, v_btns) = get_controls(curr_ch, next_ch, keypad_coords);

            let memo_key = (curr_ch, next_ch, depth);
            let shortest = if let Some(cached) = memo.get(&memo_key) {
                *cached
            } else {
                let h_res =
                    find_shortest_seq_len_control_pad(h_btns, control_pad_coords, depth, &mut memo);
                let v_res =
                    find_shortest_seq_len_control_pad(v_btns, control_pad_coords, depth, &mut memo);

                let shorter = if v_res < h_res { v_res } else { h_res };
                memo.insert(memo_key, shorter);
                shorter
            };
            len += shortest;

            curr_ch = next_ch;
        }

        len
    }

    fn find_shortest_seq_len_control_pad(
        code: Vec<char>,
        control_pad_coords: &HashMap<char, (usize, usize)>,
        remn: u64,
        memo: &mut HashMap<(char, char, u64), u64>,
    ) -> u64 {
        if remn == 0 {
            return code.len() as u64;
        }

        let mut curr_ch = 'A';
        let mut len = 0;
        for next_ch in code {
            let (h_btns, v_btns) = get_controls(curr_ch, next_ch, control_pad_coords);

            let memo_key = (curr_ch, next_ch, remn - 1);

            let shortest = if let Some(cached) = memo.get(&memo_key) {
                *cached
            } else {
                let h_res =
                    find_shortest_seq_len_control_pad(h_btns, control_pad_coords, remn - 1, memo);
                let v_res =
                    find_shortest_seq_len_control_pad(v_btns, control_pad_coords, remn - 1, memo);
                let shorter = if v_res < h_res { v_res } else { h_res };
                memo.insert(memo_key, shorter);
                shorter
            };
            len += shortest;

            curr_ch = next_ch;
        }

        len
    }

    #[test]
    fn part1() {
        let codes = load_inputs();

        let (keypad_coords, control_pad_coords) = get_coords();

        let res: u64 = codes
            .iter()
            .map(|s| {
                let shortest = find_shortest_seq_len_keypad(
                    s.chars().collect(),
                    &keypad_coords,
                    &control_pad_coords,
                    2,
                );
                let code_parsed: u64 = s[..3].parse().unwrap();
                shortest * code_parsed
            })
            .sum();

        println!("D21P1: {res}");
    }

    #[test]
    fn part2() {
        let codes = load_inputs();

        let (keypad_coords, control_pad_coords) = get_coords();

        let res: u64 = codes
            .iter()
            .map(|s| {
                let shortest = find_shortest_seq_len_keypad(
                    s.chars().collect(),
                    &keypad_coords,
                    &control_pad_coords,
                    25,
                );
                let code_parsed: u64 = s[..3].parse().unwrap();
                shortest * code_parsed
            })
            .sum();

        println!("D21P2: {res}");
    }
}
