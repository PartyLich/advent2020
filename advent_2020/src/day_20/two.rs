//! Solutions to 2020 day 20 part 2
//! --- Day 20: Jurassic Jigsaw ---
use std::collections::HashMap;
use std::str::FromStr;

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
    todo!()
}

fn rotate_grid<T: Clone>(grid: Vec<Vec<T>>, units: usize) -> Vec<Vec<T>> {
    todo!()
}

fn next_orient(
    (mut glob_vert, mut glob_horz, absolute_rotation): Orientation,
    (mut vert, mut horz, rot): Orientation,
) -> Orientation {
    todo!()
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

fn find_monsters(image: &[Vec<char>]) -> (String, Vec<(usize, usize)>) {
    todo!()
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
    use super::*;

    #[test]
    fn part_two() {
        let msg = "should return count of '#' chars that are not part of a sea monster";
        let expected = 273;
        let actual = two("input/20-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
