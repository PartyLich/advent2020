//! Solutions to 2020 day 20
//! --- Day 20: Jurassic Jigsaw ---
use crate::day_1::read_file;

type TileId = usize;
// borders
type Tile = [Vec<char>; 4];

/// parse a tile id and tile borders from a string
fn parse_tile(tile_str: &str) -> Result<(TileId, Tile), &str> {
    let mut lines = tile_str.lines();
    let head = lines.next().ok_or("Empty input")?;
    let grid: Vec<_> = lines.map(|line| line.chars().collect::<Vec<_>>()).collect();

    let (_, head) = head.split_once(" ").ok_or("Failed to parse header")?;
    let (head, _) = head.split_once(":").ok_or("Failed to parse header")?;
    let head = head.parse().map_err(|_e| "Failed to parse tile ID")?;

    // get borders, clockwise
    let back = grid.len() - 1;
    let cols = grid[0].len();
    let top = grid[0].to_owned();
    let bottom = grid[back].iter().copied().rev().collect::<Vec<_>>();
    let mut left = vec![];
    let mut right = vec![];
    for r in 0..grid.len() {
        right.push(grid[r][cols - 1]);
        left.push(grid[back - r][0]);
    }

    Ok((head, [top, right, bottom, left]))
}

/// return true if two lists match or are mirrors of each other
fn compare(a: &[char], b: &[char]) -> bool {
    // match
    a.iter().zip(b.iter()).all(|(a, b)| a.eq(b)) ||
        // match flipped
        a.iter().zip(b.iter().rev()).all(|(a, b)| a.eq(b))
}

/// return Some(TileId) if the tile has a neighbor count in the supplied range, None otherwise
fn count_neighbors(
    tiles: &[(TileId, Tile)],
    min: usize,
    max: usize,
) -> impl Fn(&(TileId, Tile)) -> Option<usize> + '_ {
    move |(id, borders)| {
        let mut count = 0;

        for border in borders {
            // n - 1 other tiles
            for (other_id, other_borders) in tiles {
                if id == other_id {
                    continue;
                }

                // 4 sides
                for other in other_borders {
                    if compare(border, other) {
                        count += 1;
                        // exit early if we've exceeded bounds
                        if count > max {
                            return None;
                        }
                    }
                }
            }
        }

        if count < min {
            None
        } else {
            Some(*id)
        }
    }
}

/// returns the product of the four corner tile ids
pub fn one(file_path: &str) -> usize {
    let input = read_file(file_path);
    let tiles: Vec<_> = input
        .trim()
        .split("\n\n")
        .map(parse_tile)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    tiles
        .iter()
        .filter_map(count_neighbors(&tiles, 2, 2))
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn comparison() {
        let msg = "should return true if lists are equal";

        let a = vec!['.', '.', '.', '#', '#', '#', '.', '#', '.', '.'];
        let b = vec!['.', '.', '#', '.', '#', '#', '#', '.', '.', '.'];
        let actual = compare(&a, &b);
        assert!(actual, "{}", msg);

        let a = vec!['.', '.', '.', '#', '#', '#', '.', '#', '.', '.'];
        let b = vec!['.', '.', '.', '#', '#', '#', '.', '#', '.', '.'];
        let actual = compare(&a, &b);
        assert!(actual, "{}", msg);
    }

    #[test]
    fn part_one() {
        let msg = "should return product of the four corner tile ids";
        let expected = 20899048083289;
        let actual = one("input/20-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
