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
        println!("res: {res}");
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
        println!("score: {score}");
    }
}
