use std::env;

#[derive(Debug)]
struct Stats {
    speed: u32,
    duration: u32,
    rest: u32
}

impl Stats {
    fn distance_traveled(&self, time: u32) -> u32 {
        let cycle = self.duration + self.rest;
        let num_of_whole_cycles = time / cycle;
        let last_cycle_position = time % cycle;
        let last_cycle_distance = if last_cycle_position >= self.duration {
            self.speed * self.duration
        } else {
            last_cycle_position * self.speed
        };

        num_of_whole_cycles * (self.speed * self.duration) + last_cycle_distance
    }
}

#[aoc_generator(day14)]
fn input_generator(input: &str) -> Vec<Stats> {
    input
        .lines()
        .map(|line| {
            let mut words = line.trim().split(' ');
            words.next(); words.next(); words.next();
            let speed = words.next().unwrap().parse().unwrap();
            words.next(); words.next();
            let duration = words.next().unwrap().parse().unwrap();
            words.next(); words.next(); words.next(); words.next(); words.next(); words.next();
            let rest = words.next().unwrap().parse().unwrap();
            Stats { speed, duration, rest }
        })
        .collect()
}

fn get_race_distance() -> u32 {
    env::var("AOC_DAY14_DISTANCE")
        .unwrap_or("2503".into())
        .parse()
        .unwrap()
}

#[aoc(day14, part1)]
fn part_one(input: &[Stats]) -> u32 {
    let race_distance = get_race_distance();

    input
        .into_iter()
        .map(|s| {
            s.distance_traveled(race_distance)
        })
        .max()
        .unwrap_or_default()
}

#[aoc(day14, part2)]
fn part_two(input: &[Stats]) -> u32 {
    let race_distance = get_race_distance();

    let mut scoreboard = vec![0; input.len()];
    for t in 1..=race_distance {
        let distances: Vec<u32> = input
            .iter()
            .map(|horse| horse.distance_traveled(t))
            .collect();

        let max_distance = distances.iter().max().unwrap();
        for (i, distance) in distances.iter().enumerate() {
            if distance == max_distance {
                scoreboard[i] += 1;
            }
        }

    }

    scoreboard.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;

    #[test]
    fn part1_1() {
        env::set_var("AOC_DAY14_DISTANCE", "1000");
        let input =
"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        let result = part_one(&input_generator(input));
        assert_eq!(result, 1120);
    }

    #[test]
    fn part2_1() {
        env::set_var("AOC_DAY14_DISTANCE", "1000");
        let input =
"Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        let result = part_two(&input_generator(input));
        assert_eq!(result, 689);
    }
}
