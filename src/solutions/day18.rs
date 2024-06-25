
#[aoc_generator(day18)]
fn input_generator(input: &str) ->  [[bool; 100]; 100] {
    let mut new_grid = [[false; 100]; 100];
    for (i, line) in input.lines().enumerate() {
        for (j, character) in line.chars().enumerate() {
            let next_light = match character {
                '.' => false,
                '#' => true,
                _ => panic!("This character should not be in the input")
            };
            new_grid[i][j] = next_light
        }
    }
    new_grid
}

fn on_next_cycle(x: usize, y: usize, grid: &[[bool; 100]; 100]) -> bool {
    let current_state = grid[x][y];
    let mut neighbor_count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if i == 0 && j == 0 { continue }
            let check_x: i32 = x as i32 + i;
            let check_y: i32 = y as i32 + j;

            if check_x < 0 || check_y < 0 || check_x > 99 || check_y > 99 { continue }
            if grid[check_x as usize][check_y as usize] { neighbor_count += 1}
        }
    }
    if current_state {
        neighbor_count == 2 || neighbor_count == 3
    } else {
        neighbor_count == 3
    }
}

fn next_grid_p1(input: [[bool; 100]; 100]) -> [[bool; 100]; 100] {
    let mut new_grid = [[false; 100]; 100];
    for i in 0..100 {
        for j in 0..100 {
            new_grid[i][j] = on_next_cycle(i, j, &input)
        }
    }
    new_grid
}

fn next_grid_p2(input: [[bool; 100]; 100]) -> [[bool; 100]; 100] {
    let mut new_grid = [[false; 100]; 100];
    for i in 0..100 {
        for j in 0..100 {
            if matches!((i, j), (0, 0) | (0, 99) | (99, 0) | (99, 99)) {
                new_grid[i][j] = true;
                continue
            }
            new_grid[i][j] = on_next_cycle(i, j, &input)
        }
    }
    new_grid
}

#[aoc(day18, part1)]
fn part_one(input: &[[bool; 100]; 100]) -> u16 {
    let mut curr_grid = input.to_owned();
    for _ in 0..100 {
        curr_grid = next_grid_p1(curr_grid)
    }
    curr_grid
        .into_iter()
        .flatten()
        .map(|e| e as u16)
        .sum()
}

#[aoc(day18, part2)]
fn part_two(input: &[[bool; 100]; 100]) -> u16 {
    let mut curr_grid = input.to_owned();
    curr_grid[0][0] = true;
    curr_grid[0][99] = true;
    curr_grid[99][0] = true;
    curr_grid[99][99] = true;
    for _ in 0..100 {
        curr_grid = next_grid_p2(curr_grid)
    }
    curr_grid
        .into_iter()
        .flatten()
        .map(|e| e as u16)
        .sum()
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
