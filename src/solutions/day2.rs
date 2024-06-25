struct Gift {
    l: u8,
    w: u8,
    h: u8
}

impl Gift {
    fn min_side(&self) -> u8 {
        *vec![self.l, self.w, self.h]
            .iter()
            .min()
            .unwrap()
    }

    fn medium_side(&self) -> u8 {
        let mut sides = vec![self.l.clone(), self.w.clone(), self.h.clone()];
        sides.sort();

        sides[1]
    }

    fn surface_area(&self) -> u32 {
        2 * self.l as u32 * self.w as u32 +
        2 * self.w as u32 * self.h as u32 +
        2 * self.h as u32 * self.l as u32
    }

    fn volume(&self) -> u32 {
        self.l as u32 *
        self.w as u32 *
        self.h as u32
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Gift> {
    input.
        lines()
        .map(|line| {
            let mut dimensions = line.trim().split('x').map(|d| d.parse().unwrap());
            Gift {
                l: dimensions.next().unwrap(),
                w: dimensions.next().unwrap(),
                h: dimensions.next().unwrap(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn part_one(input: &[Gift]) -> u32 {
    input
        .into_iter()
        .map(|gift| gift.surface_area() + gift.min_side() as u32 * gift.medium_side() as u32)
        .sum()
}

#[aoc(day2, part2)]
fn part_two(input: &[Gift]) -> u32 {
    input
        .into_iter()
        .map(|gift| {
            let volume = gift.volume();
            let smallest_perimeter = gift.min_side() as u32 * 2 + gift.medium_side() as u32 * 2;
            volume + smallest_perimeter
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1() {
        let result = part_one(&input_generator("2x3x4"));
        assert_eq!(result, 58);
    }

    #[test]
    fn part1_2() {
        let result = part_one(&input_generator("1x1x10"));
        assert_eq!(result, 43);
    }

    #[test]
    fn part2_1() {
        let result = part_two(&input_generator("2x3x4"));
        assert_eq!(result, 34);
    }

    #[test]
    fn part2_2() {
        let result = part_two(&input_generator("1x1x10"));
        assert_eq!(result, 14);
    }
}
