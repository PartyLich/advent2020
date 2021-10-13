//! Solutions to 2020 day 11 part 2
//! --- Day 11: Seating System ---
use super::{Map, Seating};

// check the first seat visibe from the provided (r, c) and return the count of full seats in each
// of eight directions
fn count_neighbors(map: &[Vec<Seating>], r: usize, c: usize) -> usize {
    macro_rules! apply_rule {
        ($value: ident, $cell: ident) => {
            match $cell {
                Seating::Full => {
                    // full seat visible
                    $value = $cell.value();
                    break;
                }
                Seating::Open => break,
                _ => {}
            }
        };
    }
    let map_height = map.len();
    let map_width = map[0].len();

    // count left
    let mut left = 0;
    for cell in map[r].iter().take(c).rev() {
        apply_rule!(left, cell);
    }

    // count right
    let mut right = 0;
    for cell in map[r].iter().take(map_height).skip(c + 1) {
        apply_rule!(right, cell);
    }

    // count up
    let mut up = 0;
    for row in map.iter().take(r).rev() {
        let cell = &row[c];
        apply_rule!(up, cell);
    }

    // count down
    let mut down = 0;
    for row in map.iter().take(map_height).skip(r + 1) {
        let cell = &row[c];
        apply_rule!(down, cell);
    }

    // diagonals
    // top left
    let mut up_left = 0;
    for i in 1..map_height {
        let col = c.checked_sub(i);
        let row = r.checked_sub(i);

        if let (Some(row), Some(col)) = (row, col) {
            if (0..map_height).contains(&row) && (0..map_width).contains(&col) {
                let cell = &map[row][col];
                apply_rule!(up_left, cell);
            } else {
                break;
            }
        }
    }

    // top right
    let mut up_right = 0;
    for i in 1..map_height {
        let col = c.checked_add(i);
        let row = r.checked_sub(i);

        if let (Some(row), Some(col)) = (row, col) {
            if (0..map_height).contains(&row) && (0..map_width).contains(&col) {
                let cell = &map[row][col];
                apply_rule!(up_right, cell);
            } else {
                break;
            }
        }
    }

    // bottom right
    let mut down_right = 0;
    for i in 1..map_height {
        let col = c.checked_add(i);
        let row = r.checked_add(i);

        if let (Some(row), Some(col)) = (row, col) {
            if (0..map_height).contains(&row) && (0..map_width).contains(&col) {
                let cell = &map[row][col];
                apply_rule!(down_right, cell);
            } else {
                break;
            }
        }
    }

    // bottom left
    let mut down_left = 0;
    for i in 1..map_height {
        let col = c.checked_sub(i);
        let row = r.checked_add(i);

        if let (Some(row), Some(col)) = (row, col) {
            if (0..map_height).contains(&row) && (0..map_width).contains(&col) {
                let cell = &map[row][col];
                apply_rule!(down_left, cell);
            } else {
                break;
            }
        }
    }

    left + right + down + up + down_left + down_right + up_left + up_right
}

/// return the count of occupied seats once the system has stagnated
pub fn two(file_path: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn counts_neighbors() {
        let map: Map<Seating> = r#"#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##"#
            .lines()
            .map(|line| line.chars().map(From::from).collect::<Vec<_>>())
            .collect();

        let expected = vec![
            vec![3, 5, 5, 5, 5, 5, 5, 5, 5, 3],
            vec![4, 7, 7, 7, 7, 7, 7, 7, 6, 5],
            vec![5, 8, 8, 8, 8, 8, 8, 6, 7, 5],
            vec![5, 8, 8, 8, 8, 8, 8, 8, 7, 5],
            vec![5, 8, 7, 8, 8, 8, 8, 8, 8, 5],
            vec![5, 7, 8, 8, 8, 8, 8, 8, 7, 5],
            vec![5, 7, 7, 8, 7, 7, 7, 7, 7, 5],
            vec![5, 6, 8, 8, 8, 8, 8, 8, 7, 4],
            vec![4, 7, 7, 8, 8, 8, 7, 7, 7, 5],
            vec![3, 5, 5, 5, 5, 5, 5, 5, 5, 3],
        ];
        let actual: Map<usize> = map
            .iter()
            .enumerate()
            .map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .map(|(c, _)| count_neighbors(&map, r, c))
                    .collect()
            })
            .collect();

        for r in 0..10 {
            for c in 0..10 {
                let msg = format!("disagreement at row {}, col {}", r, c);
                assert_eq!(actual[r][c], expected[r][c], "{}", msg);
            }
        }
    }

    #[test]
    fn part_two() {
        let msg = "should return the count of occupied seats once the system has stagnated";
        let expected = 26;
        let actual = two("input/11-t.txt");
        assert_eq!(actual, expected, "{}", msg);
    }
}
