//! Solutions to 2020 day 17 part 2
//! --- Day 17: Conway Cubes ---
use std::collections::HashSet;

use crate::day_1::read_file;

use super::parse_state;

type Quad = (isize, isize, isize, isize);

/// map a line of serialized conway cube states to the set of active coordinates
fn to_active(enumerated_line: (usize, &str)) -> HashSet<Quad> {
    let (y, line) = enumerated_line;
    line.chars()
        .enumerate()
        .fold(HashSet::new(), |mut acc, (x, character)| match character {
            '#' => {
                acc.insert((x as isize, y as isize, 0, 0));
                acc
            }
            _ => acc,
        })
}

/// Returns a Some with the count of neighbors, if count is < max. Otherwise returns None.
fn count_neigbors(state: &HashSet<Quad>, cube: &Quad, max: usize) -> Option<usize> {
    let mut count = 0;
    for x in (cube.0 - 1)..=(cube.0 + 1) {
        for y in (cube.1 - 1)..=(cube.1 + 1) {
            for z in (cube.2 - 1)..=(cube.2 + 1) {
                for w in (cube.3 - 1)..=(cube.3 + 1) {
                    let current_cube = (x, y, z, w);

                    // skip the central cube
                    if current_cube == *cube {
                        continue;
                    }

                    if state.contains(&current_cube) {
                        count += 1;
                        // exit early if we've exceeded the neighbor limit
                        if count > max {
                            return None;
                        }
                    }
                }
            }
        }
    }

    Some(count)
}

/// Returns the next set of active conway cubes according to the following rules:
///
/// - If a cube is active and exactly 2 or 3 of its neighbors are also active,
///   the cube remains active.
///
///   Otherwise, the cube becomes inactive.
/// - If a cube is inactive but exactly 3 of its neighbors are active,
///   the cube becomes active.
///
///   Otherwise, the cube remains inactive.
fn next_state(state: &HashSet<Quad>) -> HashSet<Quad> {
    const MAX_NEIGHBORS: usize = 3;
    let mut next = HashSet::new();
    for active in state {
        // check all 81 (3^4, for 3 dimensions) positions
        for x in (active.0 - 1)..=(active.0 + 1) {
            for y in (active.1 - 1)..=(active.1 + 1) {
                for z in (active.2 - 1)..=(active.2 + 1) {
                    for w in (active.3 - 1)..=(active.3 + 1) {
                        let current_cube = (x, y, z, w);

                        if let Some(count) = count_neigbors(state, &current_cube, MAX_NEIGHBORS) {
                            if current_cube == *active && (2..3).contains(&count) {
                                // stays active if exactly 2 or 3 of its neighbors are also active
                                next.insert(*active);
                            } else if count == 3 {
                                // becomes active if exactly 3 of its neighbors are active
                                next.insert(current_cube);
                            }
                        }
                    }
                }
            }
        }
    }

    next
}

/// Count the number of cubes in the active state after the sixth cycle
pub fn two(file_path: &str) -> usize {
    let input = read_file(file_path);
    let mut state = parse_state(to_active, &input);

    for _ in 0..6 {
        state = next_state(&state);
    }

    state.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_cube_state() {
        let msg =
            "should map a line of serialized conway cube states to the set of active coordinates";
        let expected = vec![(1, 0, 0, 0)].into_iter().collect();
        let actual = to_active((0, ".#."));
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn counts_neighbors() {
        let msg = "should return the next set of active cubes";
        let initial = vec![
            (1, 0, 0, 0),
            (0, 2, 0, 0),
            (1, 2, 0, 0),
            (2, 2, 0, 0),
            (2, 1, 0, 0),
        ]
        .into_iter()
        .collect();
        let expected = Some(1);
        let actual = count_neigbors(&initial, &(1, 0, 0, 0), 3);
        assert_eq!(actual, expected, "{}", msg);

        let expected = Some(3);
        let actual = count_neigbors(&initial, &(2, 1, 0, 0), 3);
        assert_eq!(actual, expected, "{}", msg);

        let expected = Some(1);
        let actual = count_neigbors(&initial, &(1, 0, 0, 0), 3);
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    #[ignore]
    fn part_two() {
        let msg = "should return the number of cubes in the active state after the sixth cycle";
        let expected = 848;
        let actual = two("input/17-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
