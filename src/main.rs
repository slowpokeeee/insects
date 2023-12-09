#[derive(Debug, Eq, PartialEq, Clone)]
enum InsectPart {
    Bottom,
    Upper,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Insect {
    Spider,
    Ladybug,
    Bee,
    Cricket,
}

#[derive(Debug, Clone)]
struct Tile {
    north: (Insect, InsectPart),
    east: (Insect, InsectPart),
    south: (Insect, InsectPart),
    west: (Insect, InsectPart),
}

impl Tile {
    fn new(
        north: (Insect, InsectPart),
        east: (Insect, InsectPart),
        south: (Insect, InsectPart),
        west: (Insect, InsectPart),
    ) -> Self {
        Tile {
            north,
            east,
            south,
            west,
        }
    }

    fn rotate(&mut self) {
        (self.north, self.east, self.south, self.west) = (
            self.west.clone(),
            self.north.clone(),
            self.east.clone(),
            self.south.clone(),
        );
    }
}

struct Puzzle {
    board: [[Option<Tile>; 3]; 3],
    tiles: [Tile; 9],
}

impl std::fmt::Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..3 {
            for col in 0..3 {
                let tile = &self.board[row][col];
                write!(f, "{tile:?} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Puzzle {
    fn new(board: [[Option<Tile>; 3]; 3], tiles: [Tile; 9]) -> Self {
        Puzzle { board, tiles }
    }

    fn solve(&mut self) -> bool {
        let mut used_tiles = [false; 9];
        self._solve(0, 0, &mut used_tiles)
    }

    fn _solve(&mut self, row: usize, col: usize, used_tiles: &mut [bool; 9]) -> bool {
        if row == 3 {
            return true;
        }

        for tile_index in 0..self.tiles.len() {
            if !used_tiles[tile_index] {
                used_tiles[tile_index] = true;
                for _rotate in 0..4 {
                    // Try placing the current tile

                    self.board[row][col] = Some(self.tiles[tile_index].clone());

                    // Check if placement is valid
                    if self.check_neighbors(row, col) {
                        // If valid, proceed to next cell
                        if col + 1 < 3 {
                            if self._solve(row, col + 1, used_tiles) {
                                return true;
                            }
                        } else {
                            // Reached the end of the row, move to next row
                            if self._solve(row + 1, 0, used_tiles) {
                                return true;
                            }
                        }
                    }

                    // Try rotating the tile before placing it again
                    self.tiles[tile_index].rotate();
                }
                // Mark title as usable before going to next
                used_tiles[tile_index] = false;
            }
            // Backtrack if placement is invalid or doesn't lead to a solution
            self.board[row][col] = None;
        }

        // No solution found with tiles available
        false
    }

    fn check_neighbors(&self, row: usize, col: usize) -> bool {
        let Some(tile) = &self.board[row][col] else {
            return false;
        };

        if row > 0 {
            if let Some(top_neighbor) = &self.board[row - 1][col] {
                if top_neighbor.south.1 == tile.north.1 || top_neighbor.south.0 != tile.north.0 {
                    return false;
                }
            };
        }

        if row < 2 {
            if let Some(bottom_neighbor) = &self.board[row + 1][col] {
                if bottom_neighbor.north.1 == tile.south.1
                    || bottom_neighbor.north.0 != tile.south.0
                {
                    return false;
                }
            };
        }

        if col > 0 {
            if let Some(left_neighbor) = &self.board[row][col - 1] {
                if left_neighbor.east.1 == tile.west.1 || left_neighbor.east.0 != tile.west.0 {
                    return false;
                }
            };
        }

        if col < 2 {
            if let Some(right_neighbor) = &self.board[row][col + 1] {
                if right_neighbor.west.1 == tile.east.1 || right_neighbor.west.0 != tile.east.0 {
                    return false;
                }
            };
        }

        true
    }
}

fn main() {
    let board = [[None, None, None], [None, None, None], [None, None, None]];
    let tiles = [
        Tile::new(
            (Insect::Spider, InsectPart::Bottom),
            (Insect::Bee, InsectPart::Upper),
            (Insect::Spider, InsectPart::Upper),
            (Insect::Cricket, InsectPart::Bottom),
        ),
        Tile::new(
            (Insect::Ladybug, InsectPart::Bottom),
            (Insect::Cricket, InsectPart::Bottom),
            (Insect::Spider, InsectPart::Bottom),
            (Insect::Bee, InsectPart::Bottom),
        ),
        Tile::new(
            (Insect::Ladybug, InsectPart::Upper),
            (Insect::Spider, InsectPart::Bottom),
            (Insect::Bee, InsectPart::Bottom),
            (Insect::Cricket, InsectPart::Upper),
        ),
        Tile::new(
            (Insect::Spider, InsectPart::Upper),
            (Insect::Ladybug, InsectPart::Upper),
            (Insect::Ladybug, InsectPart::Bottom),
            (Insect::Cricket, InsectPart::Upper),
        ),
        Tile::new(
            (Insect::Ladybug, InsectPart::Upper),
            (Insect::Cricket, InsectPart::Bottom),
            (Insect::Bee, InsectPart::Upper),
            (Insect::Spider, InsectPart::Upper),
        ),
        Tile::new(
            (Insect::Bee, InsectPart::Upper),
            (Insect::Spider, InsectPart::Bottom),
            (Insect::Ladybug, InsectPart::Upper),
            (Insect::Cricket, InsectPart::Upper),
        ),
        Tile::new(
            (Insect::Cricket, InsectPart::Bottom),
            (Insect::Ladybug, InsectPart::Upper),
            (Insect::Bee, InsectPart::Upper),
            (Insect::Bee, InsectPart::Bottom),
        ),
        Tile::new(
            (Insect::Ladybug, InsectPart::Bottom),
            (Insect::Spider, InsectPart::Upper),
            (Insect::Bee, InsectPart::Upper),
            (Insect::Cricket, InsectPart::Bottom),
        ),
        Tile::new(
            (Insect::Bee, InsectPart::Upper),
            (Insect::Spider, InsectPart::Upper),
            (Insect::Ladybug, InsectPart::Bottom),
            (Insect::Cricket, InsectPart::Bottom),
        ),
    ];

    let mut puzzle = Puzzle::new(board, tiles);

    puzzle.solve();
    println!("{puzzle}");
}
