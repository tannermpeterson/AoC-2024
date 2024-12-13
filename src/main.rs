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
