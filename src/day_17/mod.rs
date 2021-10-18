//! Solutions to 2020 day 17
//! --- Day 17: Conway Cubes ---
use std::collections::HashSet;

type Triple = (isize, isize, isize);

/// map a line of serialized conway cube states to the set of active coordinates
fn to_active(enumerated_line: (usize, &str)) -> HashSet<Triple> {
    let (y, line) = enumerated_line;
    line.chars()
        .enumerate()
        .fold(HashSet::new(), |mut acc, (x, character)| match character {
            '#' => {
                acc.insert((x as isize, y as isize, 0));
                acc
            }
            _ => acc,
        })
}

/// parse initial conway cube state from string
fn parse_state(serialized: &str) -> HashSet<Triple> {
    serialized
        .lines()
        .enumerate()
        .map(to_active)
        .reduce(|acc, next| &acc | &next)
        .unwrap()
}

/// Count the number of cubes in the active state after the sixth cycle
pub fn one(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_cube_state() {
        let msg =
            "should map a line of serialized conway cube states to the set of active coordinates";
        let expected = vec![(1, 0, 0)].into_iter().collect();
        let actual = to_active((0, ".#."));
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_one() {
        let msg = "should return the number of cubes in the active state after the sixth cycle";
        let expected = 112;
        let actual = one("input/17-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
