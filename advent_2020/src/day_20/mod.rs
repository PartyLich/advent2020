//! Solutions to 2020 day 20
//! --- Day 20: Jurassic Jigsaw ---

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

/// returns the product of the four corner tile ids
pub fn one(file_path: &str) -> usize {
    todo!();
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
