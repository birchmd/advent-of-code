use std::{
    fmt::Debug,
    ops::{Index, IndexMut},
};

pub type Position = (usize, usize);

pub struct NeighborsCreator {
    n_rows: usize,
    n_cols: usize,
}

impl NeighborsCreator {
    const CARDINAL_DIRECTIONS: [(isize, isize); 4] = [
        (-1, 0), // up
        (0, 1),  // right
        (1, 0),  // down
        (0, -1), // left
    ];

    const ALL_DIRECTIONS: [(isize, isize); 8] = [
        (-1, 0),  // up
        (-1, 1),  // up-right
        (0, 1),   // right
        (1, 1),   // down-right
        (1, 0),   // down
        (1, -1),  // down-left
        (0, -1),  // left
        (-1, -1), // up-left
    ];

    pub fn left(&self, x: Position) -> Option<Position> {
        checked_add(x, (0, -1))
    }

    pub fn right(&self, x: Position) -> Option<Position> {
        let (i, j) = checked_add(x, (0, 1))?;
        if j < self.n_cols {
            Some((i, j))
        } else {
            None
        }
    }

    pub fn up(&self, x: Position) -> Option<Position> {
        checked_add(x, (-1, 0))
    }

    pub fn down(&self, x: Position) -> Option<Position> {
        let (i, j) = checked_add(x, (1, 0))?;
        if i < self.n_rows {
            Some((i, j))
        } else {
            None
        }
    }

    pub fn cardinal_neighbors_of(&self, x: Position) -> impl Iterator<Item = Position> + '_ {
        Self::CARDINAL_DIRECTIONS.into_iter().filter_map(move |dx| {
            let (i, j) = checked_add(x, dx)?;
            if i < self.n_rows && j < self.n_cols {
                Some((i, j))
            } else {
                None
            }
        })
    }

    pub fn all_neighbors_of(&self, x: Position) -> impl Iterator<Item = Position> + '_ {
        Self::ALL_DIRECTIONS.into_iter().filter_map(move |dx| {
            let (i, j) = checked_add(x, dx)?;
            if i < self.n_rows && j < self.n_cols {
                Some((i, j))
            } else {
                None
            }
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub rows: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn n_rows(&self) -> usize {
        self.rows.len()
    }

    pub fn n_cols(&self) -> usize {
        self.rows.first().map(|r| r.len()).unwrap_or_default()
    }

    pub fn neighbor_context(&self) -> NeighborsCreator {
        NeighborsCreator {
            n_cols: self.n_cols(),
            n_rows: self.n_rows(),
        }
    }

    pub fn index_range(&self) -> impl Iterator<Item = Position> {
        let n_rows = self.n_rows();
        let n_cols = self.n_cols();
        (0..n_rows).flat_map(move |i| (0..n_cols).map(move |j| (i, j)))
    }
}

impl<T: Clone> Grid<T> {
    pub fn transposed(&self) -> Self {
        let n_rows = self.n_rows();
        let n_cols = self.n_cols();
        let mut output = vec![Vec::with_capacity(n_rows); n_cols];
        for j in 0..n_cols {
            for i in 0..n_rows {
                output[j].push(self[(i, j)].clone())
            }
        }
        Self { rows: output }
    }
}

impl<T: Clone + Eq> Grid<T> {
    pub fn index_of(&self, el: &T) -> Option<Position> {
        self.index_range().find(|x| &self[*x] == el)
    }
}

impl<T: Clone + Into<char>> Grid<T> {
    pub fn render(&self) -> String {
        let n = self.n_rows();
        let m = self.n_cols();
        let mut output = String::with_capacity(n * (m + 1));
        for row in &self.rows {
            for x in row {
                output.push(x.clone().into());
            }
            output.push('\n');
        }
        output
    }
}

impl<T> Index<Position> for Grid<T> {
    type Output = T;

    fn index(&self, index: Position) -> &Self::Output {
        let (i, j) = index;
        let row = &self.rows[i];
        &row[j]
    }
}

impl<T> IndexMut<Position> for Grid<T> {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        let (i, j) = index;
        let row = &mut self.rows[i];
        &mut row[j]
    }
}

fn checked_add(x: Position, dx: (isize, isize)) -> Option<Position> {
    let (i, j) = x;
    let (di, dj) = dx;

    let new_i = if di < 0 {
        i.checked_sub(di.unsigned_abs())?
    } else {
        i.checked_add(di as usize)?
    };

    let new_j = if dj < 0 {
        j.checked_sub(dj.unsigned_abs())?
    } else {
        j.checked_add(dj as usize)?
    };

    Some((new_i, new_j))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let grid = example_grid();
        assert_eq!(grid[(0, 0)], 1);
        assert_eq!(grid[(1, 2)], 6);
    }

    #[test]
    fn test_index_mut() {
        let mut grid = example_grid();
        grid[(1, 2)] = 7;
        assert_eq!(grid[(1, 2)], 7);
    }

    #[test]
    fn test_transpose() {
        let grid = example_grid();
        let transposed = grid.transposed();
        assert_eq!(
            transposed,
            Grid {
                rows: vec![vec![1, 4], vec![2, 5], vec![3, 6],]
            }
        )
    }

    #[test]
    fn test_neighbors() {
        let grid = Grid {
            rows: vec![
                vec!["NW", "N", "NE"],
                vec!["W", ".", "E"],
                vec!["SW", "S", "SE"],
            ],
        };
        let nc = grid.neighbor_context();
        assert_eq!(
            nc.cardinal_neighbors_of((1, 1))
                .map(|x| grid[x])
                .collect::<Vec<&str>>(),
            vec!["N", "E", "S", "W"]
        );
        assert_eq!(
            nc.all_neighbors_of((1, 1))
                .map(|x| grid[x])
                .collect::<Vec<&str>>(),
            vec!["N", "NE", "E", "SE", "S", "SW", "W", "NW"]
        );
    }

    fn example_grid() -> Grid<u32> {
        Grid {
            rows: vec![vec![1, 2, 3], vec![4, 5, 6]],
        }
    }
}
