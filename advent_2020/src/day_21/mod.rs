//! Solutions to 2020 day 21 problems
//! --- Day 21: Allergen Assessment ---
use std::collections::{HashMap, HashSet};

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

/// retain only unique allergens
fn find_unique_allergens(map: &mut HashMap<&str, Ingredient>) {
    let mut found = HashMap::new();
    let mut count = 1;
    let mut guard = 0;

    // limit to 1000 in case something goes terribly wrong and we're heading toward an infinite loop
    while count != 0 && guard < 10000 {
        count = 0;

        for (ingredient_name, ingredient) in map.iter_mut() {
            if ingredient.allergens.is_empty() {
                continue;
            }

            if ingredient.allergens.len() == 1 {
                // single item max that's unique
                let (allergen, _) = ingredient.allergens.iter().next().unwrap();
                found.insert(*allergen, *ingredient_name);
            } else {
                count += 1;
                ingredient.allergens.retain(|allergen, _count| {
                    if let Some(name) = found.get(allergen) {
                        name == ingredient_name
                    } else {
                        true
                    }
                });
            }
        }

        guard += 1;
    }
}

/// print logic grid
#[allow(dead_code)]
fn logic_grid(map: &HashMap<&str, Ingredient>, allergens: &HashSet<&str>) {
    println!();
    print!("{:12}", "");
    for a in allergens {
        print!("| {:4} ", a.chars().take(4).collect::<String>());
    }
    println!();
    for (ingredient_name, ingredient) in map.iter() {
        print!(
            "{:<11} ",
            ingredient_name.chars().take(10).collect::<String>()
        );
        for a in allergens {
            let count = ingredient.allergens.get(a).or(Some(&0)).unwrap();
            print!("| {:4} ", count);
        }
        println!();
    }
    println!();
}

/// return count of allergen free ingredient appearances
pub fn one(file_path: &str) -> usize {
    let input = read_file(file_path);
    let mut map = parse(&input);
    retain_maxes(&mut map);
    find_unique_allergens(&mut map);

    map.iter()
        .filter_map(|(_, ingredient)| {
            if ingredient.allergens.is_empty() {
                Some(ingredient.appearances)
            } else {
                None
            }
        })
        .sum()
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
