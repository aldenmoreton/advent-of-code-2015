use itertools::Itertools;

#[aoc_generator(day17)]
fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn check_nums(target: usize, containers: &[usize]) -> usize {
    let mut sum = 0;
    for (i, container) in containers.iter().enumerate() {
        let Some(next_target) = target.checked_sub(*container) else {
            continue;
        };

        if next_target == 0 {
            sum += 1;
            continue;
        }

        sum += check_nums(next_target, &containers[i + 1..])
    }

    sum
}

fn check_with_length(
    target: usize,
    containers: &[usize],
    curr_combo_len: usize,
    combos: &mut Vec<usize>,
) -> usize {
    let mut sum = 0;
    for (i, container) in containers.iter().enumerate() {
        let Some(next_target) = target.checked_sub(*container) else {
            continue;
        };

        if next_target == 0 {
            sum += 1;
            combos.push(curr_combo_len + 1);
            continue;
        }

        sum += check_with_length(
            next_target,
            &containers[i + 1..],
            curr_combo_len + 1,
            combos,
        )
    }

    sum
}

#[aoc(day17, part1)]
fn part_one(input: &[usize]) -> usize {
    let target = 150;
    let sorted_input = input.iter().sorted_unstable().rev().cloned().collect_vec();

    check_nums(target, &sorted_input)
}

#[aoc(day17, part2)]
fn part_two(input: &[usize]) -> usize {
    let target = 150;
    let sorted_input = input.iter().sorted_unstable().rev().cloned().collect_vec();

    let mut combos = Vec::new();
    check_with_length(target, &sorted_input, 0, &mut combos);

    combos
        .into_iter()
        .into_group_map_by(|a| *a)
        .into_iter()
        .min_by_key(|(len, _)| *len)
        .unwrap()
        .1
        .len()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn part1_1() {
        let input = indoc! {"
            20
            15
            10
            5
            5
        "};

        let input = input_generator(input);
        let target = 25;
        let sorted_input = input.iter().sorted_unstable().rev().cloned().collect_vec();

        let result = check_nums(target, &sorted_input);
        assert_eq!(result, 4)
    }

    #[test]
    fn part2_1() {
        let input = indoc! {"
            20
            15
            10
            5
            5
        "};

        let input = input_generator(input);
        let target = 25;
        let sorted_input = input.iter().sorted_unstable().rev().cloned().collect_vec();

        let mut combos = Vec::new();
        check_with_length(target, &sorted_input, 0, &mut combos);
        println!("{combos:?}");

        let result = combos
            .into_iter()
            .into_group_map_by(|a| *a)
            .into_iter()
            .min_by_key(|(len, _)| *len)
            .unwrap()
            .1
            .len();

        assert_eq!(result, 3);
    }
}
