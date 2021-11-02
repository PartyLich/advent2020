//! Solutions to 2020 day 20 part 2
//! --- Day 20: Jurassic Jigsaw ---
use std::collections::HashMap;
use std::str::FromStr;

use regex::Regex;

use crate::day_1::read_file;

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

/// return true if two lists match
fn compare(a: &[char], b: &[char]) -> bool {
    a.iter().zip(b.iter()).all(|(a, b)| a.eq(b))
}

/// return true if two lists are mirrors of each other
fn compare_flipped(a: &[char], b: &[char]) -> bool {
    a.iter().zip(b.iter().rev()).all(|(a, b)| a.eq(b))
}

/// return top left TileId and a map of Tiles with their neighbor list filled
fn find_neighbors(mut tiles: Vec<Tile>) -> (Option<TileId>, HashMap<TileId, Tile>) {
    let is_even = |n| n % 2 == 0;
    let mut result = HashMap::new();
    let mut top_left = None;
    for _ in 0..tiles.len() {
        let mut tile: Tile = tiles.pop().unwrap();
        // 4 sides
        for (idx, border) in tile.borders.iter().enumerate() {
            if tile.neighbors[idx].is_some() {
                continue;
            }

            let target_side = (idx + 2) % 4;

            // n - 1 other tiles
            for mut other in tiles.iter_mut() {
                // 4 sides
                for (other_idx, other_border) in other.borders.iter().enumerate() {
                    let mut horz = false;
                    let mut vert = false;
                    let rotation = (target_side + 4 - other_idx) % 4;
                    match rotation {
                        1 => {
                            if compare(border, other_border) {
                                if is_even(other_idx) {
                                    horz = true;
                                } else {
                                    vert = true;
                                }

                                tile.neighbors[idx] = Some((other.id, (vert, horz, rotation)));
                                other.neighbors[other_idx] =
                                    Some((tile.id, (vert, horz, rotation)));
                                break;
                            }
                            if compare_flipped(border, other_border) {
                                tile.neighbors[idx] = Some((other.id, (vert, horz, rotation)));
                                other.neighbors[other_idx] =
                                    Some((tile.id, (!vert, !horz, rotation)));
                                break;
                            }
                        }
                        3 => {
                            if compare(border, other_border) {
                                if is_even(other_idx) {
                                    horz = true;
                                } else {
                                    vert = true;
                                }

                                tile.neighbors[idx] = Some((other.id, (!vert, !horz, 1)));
                                other.neighbors[other_idx] = Some((tile.id, (horz, vert, 1)));
                                break;
                            }
                            if compare_flipped(border, other_border) {
                                tile.neighbors[idx] = Some((other.id, (!vert, !horz, 1)));
                                other.neighbors[other_idx] = Some((tile.id, (horz, vert, 1)));
                                break;
                            }
                        }
                        2 => {
                            if compare(border, other_border) {
                                if is_even(other_idx) {
                                    horz = true;
                                } else {
                                    vert = true;
                                }

                                tile.neighbors[idx] = Some((other.id, (!vert, !horz, 0)));
                                other.neighbors[other_idx] = Some((tile.id, (!vert, !horz, 0)));
                                break;
                            }
                            if compare_flipped(border, other_border) {
                                tile.neighbors[idx] = Some((other.id, (!vert, !horz, 0)));
                                other.neighbors[other_idx] = Some((tile.id, (!vert, !horz, 0)));
                                break;
                            }
                        }
                        // no rotation
                        _ => {
                            if compare(border, other_border) {
                                if is_even(other_idx) {
                                    horz = true;
                                } else {
                                    vert = true;
                                }

                                tile.neighbors[idx] = Some((other.id, (vert, horz, rotation)));
                                other.neighbors[other_idx] =
                                    Some((tile.id, (vert, horz, rotation)));
                                break;
                            }
                            if compare_flipped(border, other_border) {
                                tile.neighbors[idx] = Some((other.id, (vert, horz, rotation)));
                                other.neighbors[other_idx] =
                                    Some((tile.id, (vert, horz, rotation)));
                                break;
                            }
                        }
                    }
                }
            }
        }

        if tile.neighbors[0].is_none() && tile.neighbors[3].is_none() {
            // top left corner
            top_left = Some(tile.id);
        }

        result.insert(tile.id, tile);
    }

    (top_left, result)
}

/// return the oriented tile on the provided side with the current tile oriented
fn get_side(
    neighbors: &[Option<OrientedTile>; 4],
    vert: bool,
    horz: bool,
    rotation: usize,
    side: usize,
) -> Option<OrientedTile> {
    let flipped = (side + 2) % 4;

    let is_even = rotation % 2 == 0;
    let is_even_side = side % 2 == 0;
    let is_flipped = if is_even_side {
        is_even && vert || !is_even && horz
    } else {
        is_even && horz || !is_even && vert
    };
    let side = if is_flipped { flipped } else { side };

    let mut neigh = *neighbors;
    neigh.rotate_right(rotation);

    neigh[side]
}

/// rotate a grid (2d vec) clockwise `units` times
fn rotate_grid<T: Clone>(grid: Vec<Vec<T>>, units: usize) -> Vec<Vec<T>> {
    match units % 4 {
        1 => {
            let mut result = vec![];
            for c in 0..grid[0].len() {
                let mut row = vec![];
                for line in grid.iter().rev() {
                    let cell = line[c].clone();
                    row.push(cell);
                }
                result.push(row);
            }

            result
        }
        2 => grid
            .into_iter()
            .rev()
            .map(|row| row.into_iter().rev().collect())
            .collect(),
        3 => {
            let mut result = vec![];
            for c in (0..grid[0].len()).rev() {
                let mut row = vec![];
                for line in grid.iter() {
                    let cell = line[c].clone();
                    row.push(cell);
                }
                result.push(row);
            }

            result
        }
        _ => grid,
    }
}

/// flip a grid (2d vec) on its vertical and/or horizontal axes
pub fn flip_grid<I, T: Clone>(grid: I, vertical: bool, horizontal: bool) -> Vec<Vec<T>>
where
    I: IntoIterator<Item = Vec<T>>,
    <I as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    if vertical {
        grid.into_iter()
            .rev()
            .map(|row| {
                if horizontal {
                    row.into_iter().rev().collect()
                } else {
                    row
                }
            })
            .collect()
    } else {
        grid.into_iter()
            .map(|row| {
                if horizontal {
                    row.into_iter().rev().collect()
                } else {
                    row
                }
            })
            .collect()
    }
}

fn next_orient(
    (mut glob_vert, mut glob_horz, global_rotation): Orientation,
    (mut vert, mut horz, rot): Orientation,
) -> Orientation {
    let is_even = |n| n % 2 == 0;

    // swap global pair
    if !is_even(rot) {
        std::mem::swap(&mut glob_vert, &mut glob_horz);
    }

    // apply flips
    horz ^= glob_horz;
    vert ^= glob_vert;

    // rotate global
    let next_rotation = (global_rotation + rot) % 4;

    (vert, horz, next_rotation)
}

/// return a complete row of tiles starting from the leftmost oriented tile
fn get_row(
    tile_map: &mut HashMap<TileId, Tile>,
    top_left: OrientedTile,
) -> (Option<OrientedTile>, Image) {
    let Tile {
        mut neighbors,
        body,
        ..
    } = tile_map.remove(&top_left.0).unwrap();

    // NOTE: square tiles only
    let height = body.len();
    let mut glob_vert = top_left.1 .0;
    let mut glob_horz = top_left.1 .1;
    let mut absolute_rotation = top_left.1 .2;

    // save id of bottom neighbor
    // need the next row to be oriented properly (relative to previous row). same sort of
    // combination that happens below
    let next_row = get_side(&neighbors, glob_vert, glob_horz, absolute_rotation, 2).map(
        |(next_id, next_orientation)| {
            let orientation = next_orient(top_left.1, next_orientation);
            (next_id, orientation)
        },
    );

    // orient body
    let mut row = rotate_grid(body, absolute_rotation);

    while let Some((next_id, next_o)) =
        get_side(&neighbors, glob_vert, glob_horz, absolute_rotation, 1)
    {
        let next_tile = tile_map.remove(&next_id).unwrap();

        let (v, h, r) = next_orient((glob_vert, glob_horz, absolute_rotation), next_o);
        absolute_rotation = r;
        glob_vert = v;
        glob_horz = h;

        let next_body = rotate_grid(next_tile.body, absolute_rotation);

        for (idx, line) in row.iter_mut().enumerate() {
            let mut r = idx;
            // vert flip
            if glob_vert {
                r = (height - 1) - idx;
            }

            // horz flip
            if glob_horz {
                line.extend(next_body[r].iter().rev());
            } else {
                line.extend(next_body[r].iter());
            }
        }

        neighbors = next_tile.neighbors;
    }

    (next_row, row)
}

/// assemble tiles into an image
fn assemble(tile_map: &mut HashMap<TileId, Tile>, top_left: OrientedTile) -> Image {
    let mut result = vec![];

    let (mut next, row) = get_row(tile_map, top_left);
    result.extend(row);

    while let Some(top_left) = next {
        let (next_left, row) = get_row(tile_map, top_left);
        next = next_left;
        result.extend(row);
    }

    result
}

fn monster_indexes(row_offset: usize, col_offset: usize) -> Vec<(usize, usize)> {
    r#"                  #
#    ##    ##    ###
 #  #  #  #  #  #"#
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.char_indices()
                .filter_map(|(col, ch)| {
                    if ch == '#' {
                        Some((row + row_offset, col + col_offset))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}

/// returns an image oriented with monsters located, and a list of all indexes that contain part of
/// a monster
fn find_monsters(image: &[Vec<char>]) -> (String, Vec<(usize, usize)>) {
    const MONSTER_WIDTH: usize = 20;
    let monster_re: Regex =
        Regex::new("(.{18}#).\n(#.{4}(?:#{2}.{4}){2}#{3})\n(.#(?:.{2}#){5})").unwrap();

    fn halp(monster_re: &Regex, image: &[Vec<char>]) -> Option<(String, Vec<(usize, usize)>)> {
        let mut result = vec![];
        let len = image.len();

        // check every flip variation
        for v_flip in 0..=1 {
            let v_flip = v_flip != 0;
            for h_flip in 0..=1 {
                let h_flip = h_flip != 0;

                // check every 3 x MONSTER_WIDTH block
                for row in 0..(len - 2) {
                    let mut col = 0;
                    while col < (len - MONSTER_WIDTH) {
                        let mut f;
                        let mut r;
                        let image_iter: &mut dyn Iterator<Item = &Vec<char>> = if v_flip {
                            r = image[row..(row + 3)].iter().rev();
                            &mut r
                        } else {
                            f = image[row..(row + 3)].iter();
                            &mut f
                        };

                        let img_str = image_iter
                            .map(|line| {
                                if h_flip {
                                    line.iter().rev().skip(col).take(20).collect()
                                } else {
                                    line.iter().skip(col).take(20).collect()
                                }
                            })
                            .collect::<Vec<String>>()
                            .join("\n");

                        if monster_re.is_match(&img_str) {
                            result.extend(monster_indexes(row, col));
                            col += 20;
                            continue;
                        }

                        col += 1;
                    }
                }

                if !result.is_empty() {
                    let mut f;
                    let mut r;
                    let image_iter: &mut dyn Iterator<Item = &Vec<char>> = if v_flip {
                        r = image.iter().rev();
                        &mut r
                    } else {
                        f = image.iter();
                        &mut f
                    };
                    let img_str = image_iter
                        .map(|line| {
                            if h_flip {
                                line.iter().rev().collect()
                            } else {
                                line.iter().collect()
                            }
                        })
                        .collect::<Vec<String>>()
                        .join("\n");
                    return Some((img_str, result));
                }
            }
        }

        None
    }

    halp(&monster_re, image).unwrap_or_else(|| {
        // rotate
        let img_rot = rotate_grid(image.to_vec(), 1);
        halp(&monster_re, &img_rot).unwrap()
    })
}

/// returns count of '#' chars that are not part of a sea monster
pub fn two(file_path: &str) -> usize {
    let input = read_file(file_path);
    let tiles: Vec<_> = input
        .trim()
        .split("\n\n")
        .map(Tile::from_str)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let (top_left, mut tiles) = find_neighbors(tiles);
    let image = assemble(&mut tiles, (top_left.unwrap(), Default::default()));
    let (oriented_image, monster_indices) = find_monsters(&image);

    oriented_image
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.char_indices()
                .filter_map(|(col, ch)| match ch {
                    '#' if !monster_indices.contains(&(row, col)) => Some(()),
                    _ => None,
                })
                .count()
        })
        .sum()
}

#[cfg(test)]
mod test {
    use std::iter::FromIterator;

    use super::*;

    #[test]
    fn rotates_grid() {
        let grid = vec![vec!['a', 'b'], vec!['c', 'd']];
        let expected = vec![vec!['c', 'a'], vec!['d', 'b']];
        let actual = rotate_grid(grid, 1);
        assert_eq!(actual, expected);

        let grid = vec![vec!['a', 'b'], vec!['c', 'd']];
        let expected = vec![vec!['d', 'c'], vec!['b', 'a']];
        let actual = rotate_grid(grid, 2);
        assert_eq!(actual, expected);

        let grid = vec![vec!['a', 'b'], vec!['c', 'd']];
        let expected = vec![vec!['b', 'd'], vec!['a', 'c']];
        let actual = rotate_grid(grid, 3);
        assert_eq!(actual, expected);

        let grid = vec![vec!['a', 'b'], vec!['c', 'd']];
        let expected = vec![vec!['a', 'b'], vec!['c', 'd']];
        let actual = rotate_grid(grid, 0);
        assert_eq!(actual, expected);

        let grid = vec![vec!['a', 'b'], vec!['c', 'd']];
        let expected = vec![vec!['c', 'a'], vec!['d', 'b']];
        let actual = rotate_grid(grid, 5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn orients() {
        let msg = "should return next orientation";
        let expected = (false, false, 0);
        let actual = next_orient((false, false, 0), (false, false, 0));
        assert_eq!(actual, expected, "{}", msg);

        // 3769 -> 3931
        let expected = (true, false, 1);
        let actual = next_orient((false, false, 0), (true, false, 1));
        assert_eq!(actual, expected, "{}", msg);

        // test data -> 2473
        let expected = (false, false, 1);
        let actual = next_orient((false, false, 0), (false, false, 1));
        assert_eq!(actual, expected, "{}", msg);

        // 1567 -> 1511
        let expected = (false, false, 2);
        let actual = next_orient((false, false, 1), (false, false, 1));
        assert_eq!(actual, expected, "{}", msg);

        // 2473 -> 2557
        let expected = (true, true, 2);
        let actual = next_orient((false, true, 1), (false, true, 1));
        assert_eq!(actual, expected, "{}", msg);

        // 1123 -> 1489
        let expected = (false, true, 2);
        let actual = next_orient((false, true, 1), (true, true, 1));
        assert_eq!(actual, expected, "{}", msg);

        // 3907 -> 3167
        let expected = (true, true, 2);
        let actual = next_orient((false, true, 1), (false, true, 1));
        assert_eq!(actual, expected, "{}", msg);

        // 2039 -> 1709
        let expected = (false, true, 2);
        let actual = next_orient((false, true, 1), (true, true, 1));
        assert_eq!(actual, expected, "{}", msg);

        // 1709 -> 3779
        let expected = (true, true, 2);
        let actual = next_orient((false, true, 2), (true, false, 0));
        assert_eq!(actual, expected, "{}", msg);

        // 3697 -> 2311
        let expected = (true, false, 1);
        let actual = next_orient((true, false, 0), (true, true, 1));
        assert_eq!(actual, expected, "{}", msg);

        // 2473 -> 2557
        let expected = (false, true, 1);
        let actual = next_orient((true, false, 1), (true, true, 0));
        assert_eq!(actual, expected, "{}", msg);

        // 3907 -> 3167
        let expected = (false, false, 0);
        let actual = next_orient((true, false, 3), (false, true, 1));
        assert_eq!(actual, expected, "{}", msg);

        // 3037 -> 2957
        let expected = (false, true, 1);
        let actual = next_orient((true, false, 0), (false, false, 1));
        assert_eq!(actual, expected, "{}", msg);

        // 2179 -> 2473
        let expected = (true, false, 1);
        let actual = next_orient((true, false, 1), (false, false, 0));
        assert_eq!(actual, expected, "{}", msg);

        // 2953 -> 1069
        let expected = (false, false, 1);
        let actual = next_orient((true, true, 0), (true, true, 1));
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn assembly() {
        let msg = "should assemble the image";

        let input = read_file("input/20-t.txt");
        let tiles: Vec<_> = input
            .trim()
            .split("\n\n")
            .map(Tile::from_str)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let (top_left, mut tiles) = find_neighbors(tiles);
        let actual = assemble(&mut tiles, (top_left.unwrap(), Default::default()))
            .into_iter()
            .rev()
            .map(String::from_iter)
            .collect::<Vec<_>>()
            .join("\n");

        let expected = r#".#.#..#.##...#.##..#####
###....#.#....#..#......
##.##.###.#.#..######...
###.#####...#.#####.#..#
##.#....#.##.####...#.##
...########.#....#####.#
....#..#...##..#.#.###..
.####...#..#.....#......
#..#.##..#..###.#.##....
#.####..#.####.#.#.###..
###.#.#...#.######.#..##
#.####....##..########.#
##..##.#...#...#.#.#.#..
...#..#..#.#.##..###.###
.#.#....#.##.#...###.##.
###.#...#..#.##.######..
.#.#.###.##.##.#..#.##..
.####.###.#...###.#..#.#
..#.#..#..#.#.#.####.###
#..####...#.#.#.###.###.
#####..#####...###....##
#.##..#..#...#..####...#
.#.###..##..##..####.##.
...###...##...#...#..###"#;
        assert_eq!(actual, expected, "{}", msg);
    }

    #[test]
    fn part_two() {
        let msg = "should return count of '#' chars that are not part of a sea monster";
        let expected = 273;
        let actual = two("input/20-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
