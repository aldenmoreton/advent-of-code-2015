
#[aoc_generator(day8)]
fn input_generator(input: &str) -> String {
    input.into()
}

#[aoc(day8, part1)]
fn part_one(_input: &str) -> i32 {
    0
}

#[aoc(day8, part2)]
fn part_two(input: &str) -> usize {
    let mut org_code: usize = 0;
    let mut new_code: usize = 0;

    for line in input.lines() {
        org_code += line.len();
        new_code += line.len();
        let mut chars = line.chars();

        while let Some(mut char) = chars.next() {
            if char == '\\' {
                char = chars.next().expect("Should not finish a line with a special character");
                if char == 'x' {
                    chars.next(); chars.next();
                    new_code += 1;
                } else if char == '"' {
                    new_code += 2;
                } else if char == '\\' {
                    new_code += 2;
                }
            }
        }
        new_code += 4;
    }

    new_code - org_code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1() {
        let input = "";
        let result = part_one(&input_generator(input));
        assert_eq!(result, 2);
    }

    #[test]
    fn part2_1() {
        let input = "\"\"";
        println!("'{input}'");
        let result = part_two(&input_generator(input));
        assert_eq!(result, 6);
    }
}
