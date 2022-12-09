use std::collections::HashMap;

mod file_reader;

fn main() {
    day_two()
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
    let entries = file_reader::get_lines_separated("./2022/day_1_input.txt");

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
    let entries = file_reader::get_lines_separated("./2022/day_1_input.txt");
    let mut best = 0;
    for entry in entries {
        let ints: Vec<i32> = entry.iter().map(|x| x.parse().unwrap()).collect();
        best = std::cmp::max(best, ints.iter().sum())
    }
    println!("{}", best)
}
