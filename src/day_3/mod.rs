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

}
