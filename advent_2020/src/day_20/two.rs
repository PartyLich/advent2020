//! Solutions to 2020 day 20 part 2
//! --- Day 20: Jurassic Jigsaw ---
use std::str::FromStr;

type TileId = usize;
type Borders = [Vec<char>; 4];
type Image = Vec<Vec<char>>;

/// vertical flip, horizontal flip, rotation
type Orientation = (bool, bool, usize);
/// id, orientation
type OrientedTile = (TileId, Orientation);

/// complete image tile
#[derive(Debug, Clone)]
struct Tile {
    /// id
    id: TileId,
    /// borders in [top, right, bottom, left] order
    borders: Borders,
    /// image excluding borders
    body: Image,
    /// adjacent tile ids in [top, right, bottom, left] order
    neighbors: [Option<OrientedTile>; 4],
}

impl FromStr for Tile {
    type Err = &'static str;

    fn from_str(tile_str: &str) -> Result<Self, Self::Err> {
        let mut lines = tile_str.lines();
        let head = lines.next().ok_or("Empty input")?;
        let grid: Vec<_> = lines.map(|line| line.chars().collect::<Vec<_>>()).collect();

        let (_, head) = head.split_once(" ").ok_or("Failed to parse header")?;
        let (head, _) = head.split_once(":").ok_or("Failed to parse header")?;
        let id = head.parse().map_err(|_e| "Failed to parse tile ID")?;

        // get borders (clockwise) and body
        let back = grid.len() - 1;
        let cols = grid[0].len();
        let top = grid[0].to_owned();
        let bottom = grid[back].iter().copied().rev().collect::<Vec<_>>();
        let mut left = vec![];
        let mut right = vec![];
        let mut body = vec![];
        for r in 0..grid.len() {
            right.push(grid[r][cols - 1]);
            left.push(grid[back - r][0]);
            if (1..grid.len() - 1).contains(&r) {
                body.push(grid[r][1..(cols - 1)].to_owned());
            }
        }

        Ok(Self {
            id,
            borders: [top, right, bottom, left],
            body,
            neighbors: [None; 4],
        })
    }
}

/// returns count of '#' chars that are not part of a sea monster
pub fn two(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_two() {
        let msg = "should return count of '#' chars that are not part of a sea monster";
        let expected = 273;
        let actual = two("input/20-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
