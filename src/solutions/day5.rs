
#[aoc_generator(day5)]
fn input_generator(input: &str) -> String {
    input.into()
}

#[aoc(day5, part1)]
fn part_one(_input: &str) -> i32 {
    0
}

#[aoc(day5, part2)]
fn part_two(_input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1() {
        let input = "";
        let result = part_one(&input_generator(input));
        assert_eq!(result, 0);
    }

    #[test]
    fn part2_1() {
        let input = "";
        let result = part_two(&input_generator(input));
        assert_eq!(result, 0);
    }
}
