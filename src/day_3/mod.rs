//! Solutions to 2020 day 3 problems
//! Toboggan Trajectory
use crate::day_1::read_file;

/// Map square types
#[derive(Debug, PartialEq)]
enum Terrain {
    /// a grid square containing a tree
    Tree,
    /// a grid square containing open space
    Open,
}

impl From<char> for Terrain {
    fn from(character: char) -> Self {
        match character {
            '.' => Self::Open,
            '#' => Self::Tree,
            _ => panic!("Invalid character in map data: '{}'", character),
        }
    }
}

type Map = Vec<Vec<Terrain>>;

/// deserializes a 2d vec of [`Terrain`] from the specified file path
fn load_terrain(file_path: &str) -> Map {
    read_file(file_path)
        .lines()
        .map(|line| line.chars().map(From::from).collect::<Vec<_>>())
        .collect()
}

/// A line slope on a 2 dimensional coordinate system
#[derive(Debug, Clone, Copy)]
struct Slope(usize, usize);

impl Slope {
    pub fn new(x: usize, y: usize) -> Self {
        // reduce very naively. fine for the specs in the problem, but still deserves better
        if x > 0 && y % x == 0 {
            return Self(x / x, y / x);
        }
        Self(x, y)
    }
}

/// Count the number of [`Tree`](Terrain::Tree) squares encountered on this map by following the
/// provided slope, starting from the top left (0, 0) square
fn count_trees(map: &Map, slope: Slope) -> usize {
    if slope.1 < 1 {
        panic!(
            "Supplied slope {:?} cannot take you down the mountain",
            slope
        )
    }

    let mut x = 0;
    let mut y = 0;
    let mut count = 0;
    let map_height = map.len();
    let map_width = map[0].len();

    while y < map_height {
        count += match map[y][x] {
            Terrain::Tree => 1,
            Terrain::Open => 0,
        };
        x = (x + slope.0) % map_width;
        y += slope.1;
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn loads_terrain() {
        let msg = "should deserialize a map string";
        let expected = vec![
            vec![Terrain::Open, Terrain::Open, Terrain::Tree, Terrain::Tree],
            vec![Terrain::Tree, Terrain::Open, Terrain::Open, Terrain::Open],
        ];
        let actual = load_terrain("input/3-t_terrain.txt");
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn counts_trees() {
        let msg = "should count the number of trees for a given slope";
        let map = vec![
            vec![Terrain::Open, Terrain::Open, Terrain::Tree, Terrain::Tree],
            vec![Terrain::Tree, Terrain::Open, Terrain::Open, Terrain::Open],
        ];
        let expected = 0;
        let actual = count_trees(&map, Slope::new(1, 1));
        assert_eq!(actual, expected, "{}", msg);

        let expected = 1;
        let actual = count_trees(&map, Slope::new(0, 1));
        assert_eq!(actual, expected, "{}", msg);

        let expected = 1;
        let actual = count_trees(&map, Slope::new(4, 1));
        assert_eq!(actual, expected, "{}", msg);
    }
}
