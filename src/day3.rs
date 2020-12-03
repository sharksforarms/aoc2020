/*!
Part 1:
Starting from the top-left, count how many trees are hit if going down the slope
of 3 right, 1 down

Part 2:
Same as part 1 but with other slope values
*/

use std::{
    convert::From,
    io::{self, Read},
};

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Open,
    Tree,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Open,
            '#' => Tile::Tree,
            _ => panic!("Unknown tile"),
        }
    }
}

fn tree_count(map: &[Vec<Tile>], slope_x: usize, slope_y: usize) -> usize {
    let height = map.len();
    let width = map[0].len();

    let mut tree_count = 0;
    let mut y = 0;
    let mut x = 0;

    while y + slope_y < height {
        y += slope_y;
        x += slope_x;
        let tile = &map[y][x % width];
        if *tile == Tile::Tree {
            tree_count += 1;
        }
    }

    tree_count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let map = {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;

        buffer
            .lines()
            .map(|lines| lines.chars().map(|t| t.into()).collect::<Vec<Tile>>())
            .collect::<Vec<Vec<Tile>>>()
    };

    println!("Part 1: # trees {}", tree_count(&map, 3, 1));

    let slope1 = tree_count(&map, 1, 1);
    let slope2 = tree_count(&map, 3, 1);
    let slope3 = tree_count(&map, 5, 1);
    let slope4 = tree_count(&map, 7, 1);
    let slope5 = tree_count(&map, 1, 2);
    println!(
        "Part 2: # trees {}",
        slope1 * slope2 * slope3 * slope4 * slope5
    );

    Ok(())
}
