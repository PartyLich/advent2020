//! Solutions to 2020 day 21 problems
//! --- Day 21: Allergen Assessment ---
use std::collections::HashMap;

use crate::day_1::read_file;

/// a food ingredient
#[derive(Debug, Default)]
struct Ingredient<'a> {
    /// number of foods this ingredient appears in
    appearances: usize,
    /// max times an allergen is associated with this ingredient
    max: (&'a str, usize),
    /// map of allergens associated with this ingredient
    allergens: HashMap<&'a str, usize>,
}

/// parse str input into an ingredient/allergen map
fn parse(input: &str) -> HashMap<&str, Ingredient> {
    input
        .lines()
        .map(|line| {
            let (ingredients, allergens) = line
                .split_once("(contains ")
                .expect("Failed to parse ingredient list");
            let allergens: Vec<_> = allergens
                .split_once(")")
                .map(|(allergens, _)| allergens.split(", ").collect())
                .expect("Failed to parse allergens");
            let ingredients: Vec<_> = ingredients
                .split_whitespace()
                .map(|ingredient| (ingredient, allergens.clone()))
                .collect();

            ingredients
        })
        .fold(HashMap::new(), |mut acc, next| {
            let ingredients = next;

            for (ingredient_name, allergens) in ingredients {
                let ingredient = acc.entry(ingredient_name).or_default();
                ingredient.appearances += 1;

                // update entry
                for a in allergens {
                    let count = ingredient.allergens.entry(a).or_default();
                    *count += 1;

                    if *count > ingredient.max.1 {
                        ingredient.max = (a, *count);
                    }
                }
            }

            acc
        })
}

/// retain only allergens equal to the max for each ingredient
///
/// eg if foo is associated with Dairy twice and Fish once, keep only Dairy
fn retain_maxes(map: &mut HashMap<&str, Ingredient>) {
    let mut maxes = HashMap::new();

    for ingredient in map.values_mut() {
        let (max_name, max) = ingredient.max;
        ingredient.allergens.iter().for_each(|(min_name, min)| {
            // update global max for allergen
            if *min_name == max_name {
                let global_max = maxes.entry(max_name).or_insert(max);
                if max > *global_max {
                    *global_max = max;
                }

                return;
            }

            let global_max = maxes.entry(min_name).or_insert(*min);
            if *min > *global_max {
                *global_max = *min;
            }
        })
    }

    // remove any allergen associated less than max
    for ingredient in map.values_mut() {
        ingredient.allergens.retain(|name, count| {
            let max = maxes.get(name).unwrap_or(&0);
            *count >= *max
        });
    }
}

/// return count of allergen free ingredient appearances
pub fn one(file_path: &str) -> usize {
    let input = read_file(file_path);
    let mut map = parse(&input);
    retain_maxes(&mut map);

    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one() {
        let msg = "should count of allergen free ingredient appearances";
        let expected = 5;
        let actual = one("input/21-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
