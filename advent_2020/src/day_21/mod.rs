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
fn parse(input: &str) -> (HashMap<&str, Ingredient>, HashMap<&str, usize>) {
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
        .fold(
            (HashMap::new(), HashMap::new()),
            |(mut acc, mut maxes), next| {
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

                        // update maxes
                        let max = maxes.entry(a).or_default();
                        if *count > *max {
                            *max = *count;
                        }
                    }
                }

                (acc, maxes)
            },
        )
}

/// retain only allergens equal to the max for each ingredient
///
/// eg if foo is associated with Dairy twice and Fish once, keep only Dairy
fn retain_maxes(map: &mut HashMap<&str, Ingredient>, maxes: &HashMap<&str, usize>) {
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
    let (mut map, maxes) = parse(&input);
    retain_maxes(&mut map, &maxes);
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

/// returns list of allergen containing ingredients sorted by allergen
pub fn two(file_path: &str) -> String {
    let input = read_file(file_path);
    let (mut map, maxes) = parse(&input);
    retain_maxes(&mut map, &maxes);
    find_unique_allergens(&mut map);

    let mut dangerous_ingredients = map
        .into_iter()
        .filter_map(|(name, ingredient)| {
            if ingredient.allergens.is_empty() {
                None
            } else {
                let (allergen, _) = ingredient.allergens.into_iter().next().unwrap();
                Some((name, allergen))
            }
        })
        .collect::<Vec<_>>();
    dangerous_ingredients.sort_unstable_by(|a, b| a.1.cmp(b.1));

    dangerous_ingredients
        .into_iter()
        .map(|(ingredient, _)| ingredient)
        .collect::<Vec<_>>()
        .join(",")
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

    #[test]
    fn part_two() {
        let msg = "should print alpha ordered list of allergen containing ingredients";
        let expected = "mxmxvkd,sqjhc,fvjkl";
        let actual = two("input/21-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
