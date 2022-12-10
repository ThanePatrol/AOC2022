use std::collections::{HashMap, HashSet};

mod file_reader;

fn main() {
    day_four_part_two()
}

fn day_four_part_two() {
    let entries = file_reader::get_lines_as_vec("./2022/day4");
    let mut count = 0;

    for entry in entries {
        let pairs = entry.split(",").collect::<Vec<&str>>();
        let first = pairs[0].split("-").collect::<Vec<&str>>();
        let secon = pairs[1].split("-").collect::<Vec<&str>>();

        let min_first = first[0].parse::<i32>().unwrap();
        let max_first = first[1].parse::<i32>().unwrap();
        let min_second = secon[0].parse::<i32>().unwrap();
        let max_second = secon[1].parse::<i32>().unwrap();

        let first_range = min_first..max_first;
        let second_range = min_second..max_second;

        if first_range.contains(&min_second) ||
            first_range.contains(&max_second) ||
            min_first == min_second || max_first == max_second {
            count += 1;
        } else if second_range.contains(&min_first) || second_range.contains(&max_first) {
            count += 1;
        }
    }
    println!("{}", count);
}

fn day_four_part_one() {
    let entries = file_reader::get_lines_as_vec("./2022/day4");
    let mut count = 0;

    for entry in entries {
        let pairs = entry.split(",").collect::<Vec<&str>>();
        let first = pairs[0].split("-").collect::<Vec<&str>>();
        let secon = pairs[1].split("-").collect::<Vec<&str>>();

        let min_first = first[0].parse::<i32>().unwrap();
        let max_first = first[1].parse::<i32>().unwrap();
        let min_second = secon[0].parse::<i32>().unwrap();
        let max_second = secon[1].parse::<i32>().unwrap();

        if min_first <= min_second && max_first >= max_second {
            count += 1;
        } else if min_second <= min_first && max_second >= max_first {
            count += 1;
        }
    }
    println!("{}", count);
}

fn day_three_part_two() {
    let entries = file_reader::get_lines_as_vec("./2022/day3");
    let mut sum = 0;
    let mut mappings = HashMap::new();

    let lower = "abcdefghijklmnopqrstuvwxyz";
    for c in lower.chars() {
        mappings.insert(c, c as usize - 96);
    }
    let upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for c in upper.chars() {
        mappings.insert(c, c as usize - 38);
    }

    let mut i = 1;
    let mut groups = Vec::new();
    let mut group = Vec::new();
    for entry in entries {
        group.push(entry);
        if i % 3 == 0 {
            groups.push(group.clone());
            group.clear();
        }
        i += 1;
    }
    println!("{:?}", groups);


    for g in groups {
        let mut map = HashMap::new();
        let sack1 = g[0].chars().collect::<Vec<char>>();
        for c in sack1 {
            map.insert(c, 0);
        }

        let sack2 = g[1].chars().collect::<Vec<char>>();
        for c in sack2 {
            if map.contains_key(&c) {
                map.insert(c, 2);
            }
        }

        let sack3 = g[2].chars().collect::<Vec<char>>();
        for c in sack3 {
            if map.contains_key(&c) && map.get(&c).unwrap() == &2 {
                println!("{}", c);
                sum += *mappings.get(&c).unwrap() as i32;
                break;
            }
        }

    }
    println!("{}", sum);
}

fn day_three_part_one() {
    let entries = file_reader::get_lines_as_vec("./2022/day3");
    let mut sum = 0;

    let mut mappings = HashMap::new();

    let lower = "abcdefghijklmnopqrstuvwxyz";
    for c in lower.chars() {
        mappings.insert(c, c as usize - 96);
    }
    let upper = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    for c in upper.chars() {
        mappings.insert(c, c as usize - 38);
    }
    println!("{:?}", mappings);

    for entry in entries {
        let mut first_comp = HashSet::new();
        let mut secn_comp = HashSet::new();

        let chars = entry.chars().collect::<Vec<char>>();
        let mid = chars.len() / 2;

        for i in 0..mid {
            first_comp.insert(chars[i]);
        }

        for i in mid..chars.len() {
            let c = chars[i];
            if first_comp.contains(&c) {
                println!("{}", c);
                sum += *mappings.get(&c).unwrap() as i32;
                first_comp.remove(&chars[i]);
            }
            secn_comp.insert(chars[i]);
        }
    }
    println!("{:?}", sum);
}

fn day_two_part_two() {
    let entries = file_reader::get_lines_as_vec("./2022/day2");

}

fn day_two() {
    let entries = file_reader::get_lines_as_vec("./2022/day2");
    let mut score = 0;

    for entry in entries {
        let chars = entry.chars().collect::<Vec<char>>();
        let elf = chars[0];
        let me = chars[2];

        //me == rock
        if me == 'X' {
            score += 1;
            if elf == 'A' {
                score += 3;
            } else if elf == 'C' {
                score += 6;
            }

            //me == paper
        } else if me == 'Y' {
            score += 2;
            if elf == 'A' {
                score += 6;
            } else if elf == 'B' {
                score += 3;
            }

            //me == scissors
        } else if me == 'Z' {
            score += 3;
            if elf == 'B' {
                score += 6;
            } else if elf == 'C' {
                score += 3;
            }
        }
    }

    println!("{}", score);
}

fn _day_one_part_two() {
    let entries = file_reader::get_lines_separated("./2022/day1");

    let mut sums = Vec::new();

    for entry in entries {
        println!("{:?}", entry);
        let ints = entry.iter().map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
        let s = ints.iter().sum::<i32>();
        sums.push(s);
    }
    sums.sort();
    let n = sums.len();
    println!("{:?}", sums);
    let top_three = sums[n-1] + sums[n-2] + sums[n-3];
    println!("{}", top_three);
}

fn _day_one() {
    let entries = file_reader::get_lines_separated("./2022/day1");
    let mut best = 0;
    for entry in entries {
        let ints: Vec<i32> = entry.iter().map(|x| x.parse().unwrap()).collect();
        best = std::cmp::max(best, ints.iter().sum())
    }
    println!("{}", best)
}
