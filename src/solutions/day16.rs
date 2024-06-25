use std::collections::HashMap;

const ANALYSIS_INPUT: &str =
"children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1";

fn get_kv(input: &str) -> (&str, u32) {
    let mut kv = input.split(": ");
    let key = kv.next().unwrap();
    let value = kv.next().unwrap().parse().unwrap();

    (key, value)
}

fn get_analysis(input: &str) -> HashMap<&str, u32> {
    let mut map: HashMap<&str, u32> = HashMap::new();
    for line in input.lines() {
        let (key, value) = get_kv(line);
        map.insert(
            key,
            value
        );
    }
    map
}

#[aoc(day16, part1)]
fn part_one(input: &str) -> String {
    let analysis_map = get_analysis(ANALYSIS_INPUT);
    'sues: for line in input.lines() {
        let (name_part, compounds_part) = line.split_once(": ").unwrap();
        let sue_number = name_part.split_once(' ').unwrap().1.to_string();
        let compounds = compounds_part.split(", ");
        for compound in compounds {
            let (k, v) = get_kv(compound);
            if analysis_map.get(k).unwrap() != &v {
                continue 'sues
            }
        }
        println!("{name_part}");
        println!("{compounds_part}");

        return sue_number
    }
    unreachable!()
}

#[aoc(day16, part2)]
fn part_two(input: &str) -> String {
    let analysis_map = get_analysis(ANALYSIS_INPUT);
    'sues: for line in input.lines() {
        let (name_part, compounds_part) = line.split_once(": ").unwrap();
        let sue_number = name_part.split_once(' ').unwrap().1.to_string();
        let compounds = compounds_part.split(", ");
        for compound in compounds {
            let (k, v) = get_kv(compound);
            match k {
                "cats" | "trees" => {
                    if analysis_map.get(k).unwrap() >= &v {
                        continue 'sues
                    }
                },
                "pomeranians" | "goldfish" => {
                    if analysis_map.get(k).unwrap() <= &v {
                        continue 'sues
                    }
                },
                _ => {
                    if analysis_map.get(k).unwrap() != &v {
                        continue 'sues
                    }
                }
            }
        }
        println!("{name_part}");
        println!("{compounds_part}");

        return sue_number
    }
    unreachable!()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn part1_1() {
//         let input = "";
//         let result = part_one(&input_generator(input));
//         assert_eq!(result, 0);
//     }

//     #[test]
//     fn part2_1() {
//         let input = "";
//         let result = part_two(&input_generator(input));
//         assert_eq!(result, 0);
//     }
// }
