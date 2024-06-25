#[aoc_generator(day20)]
fn input_generator(input: &str) -> String {
    input.into()
}

fn get_house_presents(house_number: i32, multiplier: i32) -> i32 {
    let mut num_presents = 0;
    for i in 1..=house_number {
        if house_number % i == 0 {
            num_presents += i
        }
    }

    num_presents * multiplier
}

#[aoc(day20, part1)]
fn part_one(_input: &str) -> i32 {
    let input = 33100000;
    let mut house_number = 1;
    loop {
        if get_house_presents(house_number, 10) >= input {
            break house_number;
        }
        house_number += 1
    }
}

#[aoc(day20, part2)]
fn part_two(_input: &str) -> i32 {
    let input = 33_100_000;
    let len = 776_160;
    let mut house_number = len / 2;
    loop {
        let curr_presents = get_house_presents(house_number, 11);
        match curr_presents.cmp(input) {}
        house_number += 1
    }
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
