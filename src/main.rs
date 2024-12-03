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
