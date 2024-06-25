enum Direction {
	Up,
	Down
}

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<Direction> {
	input
		.chars()
		.map(|c| match c {
			'(' => Direction::Up,
			')' => Direction::Down,
			_ => panic!("Should only input parathesis")
		})
		.collect()
}

#[aoc(day1, part1)]
fn part1(input: &[Direction]) -> i32 {
    input
		.into_iter()
		.map(|d| match d {
			Direction::Up => 1,
			Direction::Down => -1
		})
		.sum()
}

#[aoc(day1, part2)]
fn part2(input: &[Direction]) -> usize {
    let mut sum: u32 = 0;

    for (i, d) in input.into_iter().enumerate() {
        match d {
            Direction::Up => sum += 1,
            Direction::Down => {
				if let Some(s) = sum.checked_sub(1) {
                	sum = s;
            	} else {
                	return i + 1;
            	}
			}
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // (()) and ()() both result in floor 0.
    #[test]
    fn sample1() {
        assert_eq!(part1(&input_generator("(())")), 0);
        assert_eq!(part1(&input_generator("()()")), 0);
    }

    // ((( and (()(()( both result in floor 3.
    #[test]
    fn sample2() {
        assert_eq!(part1(&input_generator("(((")), 3);
        assert_eq!(part1(&input_generator("(()(()(")), 3);
    }

    // ))((((( also results in floor 3.
    #[test]
    fn sample3() {
        assert_eq!(part1(&input_generator("))(((((")), 3);
    }

    // ()) and ))( both result in floor -1 (the first basement level).
    #[test]
    fn sample4() {
        assert_eq!(part1(&input_generator("())")), -1);
        assert_eq!(part1(&input_generator("))(")), -1);
    }

    // ))) and )())()) both result in floor -3.
    #[test]
    fn sample5() {
        assert_eq!(part1(&input_generator(")))")), -3);
        assert_eq!(part1(&input_generator(")())())")), -3);
    }

    // ) causes him to enter the basement at character position 1.
    #[test]
    fn sample6() {
        assert_eq!(part2(&input_generator(")")), 1);
    }

    // ()()) causes him to enter the basement at character position 5.
    #[test]
    fn sample7() {
        assert_eq!(part2(&input_generator("()())")), 5);
    }
}