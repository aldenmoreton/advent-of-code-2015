
#[aoc_generator(day15)]
fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            let line = line.replace(',', "");
            let mut words = line.split(' ');
            words.next(); words.next();
            let capacity: i32 = words.next().unwrap().parse().unwrap();
            words.next();
            let durability: i32 = words.next().unwrap().parse().unwrap();
            words.next();
            let flavor: i32 = words.next().unwrap().parse().unwrap();
            words.next();
            let texture: i32 = words.next().unwrap().parse().unwrap();
            words.next();
            let calories: i32 = words.next().unwrap().parse().unwrap();
            vec![capacity, durability, flavor, texture, calories]
        })
        .collect()
}

#[aoc(day15, part1)]
fn part_one(input: &[Vec<i32>]) -> i32 {
    let num_of_ingredients = input.len();
    let num_of_properties = input.first().unwrap().len();
    let mut ingredient_amounts = vec![0; num_of_ingredients];
    let mut max_score = 0;
    ingredient_amounts[num_of_ingredients - 1] = 100;
    'trials: loop {
        let mut curr_score = 1;
        'property_scoring: for property_index in 0..num_of_properties - 1 {
            let mut property_score = 0;
            for (ingredient, ingredient_amount) in input.iter().zip(ingredient_amounts.iter()) {
                property_score += ingredient[property_index] * ingredient_amount
            }
            if property_score <= 0 {
                curr_score = 0;
                break 'property_scoring
            }
            curr_score *= property_score
        }
        if max_score < curr_score {max_score = curr_score}

        'amount_counting: loop {
            ingredient_amounts[num_of_ingredients - 1] = 0;
            'adding: for i in 0..num_of_ingredients - 1 {
                let new_amount = ingredient_amounts[i] + 1;
                if new_amount > 100 {
                    if i == (num_of_ingredients - 2) {
                        break 'trials
                    } else {
                        ingredient_amounts[i] = 0
                    }
                } else {
                    ingredient_amounts[i] = new_amount;
                    break 'adding
                }
            }
            let amount_sum: i32 = ingredient_amounts.iter().sum();
            if amount_sum <= 100 {
                ingredient_amounts[num_of_ingredients - 1] = 100 - amount_sum;
                break 'amount_counting
            }
        }
    }
    max_score
}

#[aoc(day15, part2)]
fn part_two(input: &[Vec<i32>]) -> i32 {
    let num_of_ingredients = input.len();
    let num_of_properties = input.first().unwrap().len();
    let mut ingredient_amounts = vec![0; num_of_ingredients];
    let mut max_score = 0;
    ingredient_amounts[num_of_ingredients - 1] = 100;
    'trials: loop {
        let mut curr_score = 1;
        'property_scoring: for property_index in 0..num_of_properties - 1 {
            let mut property_score = 0;
            for (ingredient, ingredient_amount) in input.iter().zip(ingredient_amounts.iter()) {
                property_score += ingredient[property_index] * ingredient_amount
            }
            if property_score <= 0 {
                curr_score = 0;
                break 'property_scoring
            }
            curr_score *= property_score
        }
        if max_score < curr_score {max_score = curr_score}

        'amount_counting: loop {
            ingredient_amounts[num_of_ingredients - 1] = 0;
            'adding: for i in 0..num_of_ingredients - 1 {
                let new_amount = ingredient_amounts[i] + 1;
                if new_amount > 100 {
                    if i == (num_of_ingredients - 2) {
                        break 'trials
                    } else {
                        ingredient_amounts[i] = 0
                    }
                } else {
                    ingredient_amounts[i] = new_amount;
                    break 'adding
                }
            }
            let amount_sum: i32 = ingredient_amounts.iter().sum();
            if amount_sum <= 100 {
                ingredient_amounts[num_of_ingredients - 1] = 100 - amount_sum;
            } else {
                continue;
            }
            let calorie_count: i32 = input
                .iter()
                .zip(ingredient_amounts.iter())
                .map(|(ingredient, amount)| {
                    ingredient[num_of_properties - 1] * amount
                })
                .sum();
            if calorie_count == 500 { break 'amount_counting }
        }
    }
    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_1() {
        let input =
"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
println!("Input: {input}");
        let result = part_one(&input_generator(input));
        assert_eq!(result, 62842880);
    }

    #[test]
    fn part2_1() {
        let input =
"Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
        let result = part_two(&input_generator(input));
        assert_eq!(result, 57600000);
    }
}
