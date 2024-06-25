// use itertools::Itertools;

// #[aoc_generator(day19)]
// fn input_generator(input: &str) -> (Vec<(String, String)>, String) {
//     let (mappings, sequence) = input.split_once("\n\n").unwrap();
//     let mappings = mappings
//         .lines()
//         .map(|line| line.split_once(" => ").unwrap())
//         .map(|(key, value)| (key.to_owned(), value.to_owned()))
//         .collect_vec();

//     (mappings, sequence.to_owned())
// }

// #[aoc(day19, part1)]
// fn part_one((mappings, sequence): &(Vec<(String, String)>, String)) -> i32 {
//     let mut characters = sequence.chars().peekable();
//     while let Some(character) = characters.next() {
//         let curr_molecule = match character {
//             'e' => "e".to_owned(),
//             c if c.is_upper() && characters.peek().map(|c| c.is_lowercase()) => c.to_string(),
//         };
//     }

//     todo!()
// }

// #[aoc(day19, part2)]
// fn part_two((mappings, sequence): &(Vec<(String, String)>, String)) -> i32 {
//     0
// }

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
