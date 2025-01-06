use {
    aoc_core::{
        abs_diff, basic_grid,
        grid::{Grid, Position},
        min_max, Solution,
    },
    std::collections::HashSet,
};

pub struct Day11;

impl Solution<'_> for Day11 {
    type Input = Grid<u8>;
    type Output1 = usize;
    type Output2 = usize;

    fn parse_input(data: &str) -> Self::Input {
        basic_grid(data)
    }

    fn part_1(input: Self::Input) -> Self::Output1 {
        let map = explicit_expand_space(input);
        let galaxies: Vec<Position> = map.index_range().filter(|x| map[*x] == b'#').collect();
        let mut total = 0;
        for (i, &(x, y)) in galaxies.iter().enumerate() {
            for &(u, v) in &galaxies[(i + 1)..] {
                total += abs_diff(x, u) + abs_diff(y, v);
            }
        }
        total
    }

    fn part_2(input: Self::Input) -> Self::Output2 {
        part2_solution(input, 1_000_000)
    }
}

fn part2_solution(input: Grid<u8>, expansion_factor: usize) -> usize {
    let empty_rows: HashSet<usize> = rows_to_expand(&input);
    let empty_cols: HashSet<usize> = cols_to_expand(&input);
    let galaxies: Vec<Position> = input.index_range().filter(|x| input[*x] == b'#').collect();

    let mut total = 0;
    for (i, &(x, y)) in galaxies.iter().enumerate() {
        for &(u, v) in &galaxies[(i + 1)..] {
            let (row_start, row_end) = min_max(x, u);
            let (col_start, col_end) = min_max(y, v);

            total += traverse(row_start, row_end, expansion_factor, &empty_rows);
            total += traverse(col_start, col_end, expansion_factor, &empty_cols);
        }
    }
    total
}

fn rows_to_expand<C>(grid: &Grid<u8>) -> C
where
    C: FromIterator<usize>,
{
    let n = grid.n_rows();
    (0..n)
        .filter(|i| grid.rows[*i].iter().all(|x| *x == b'.'))
        .collect()
}

fn cols_to_expand<C>(grid: &Grid<u8>) -> C
where
    C: FromIterator<usize>,
{
    let m = grid.n_cols();
    let col_first = grid.transposed();
    (0..m)
        .filter(|j| col_first.rows[*j].iter().all(|x| *x == b'.'))
        .collect()
}

fn traverse(start: usize, end: usize, expansion_factor: usize, expanded: &HashSet<usize>) -> usize {
    let mut total = 0;

    for i in (start + 1)..=end {
        if expanded.contains(&i) {
            total += expansion_factor;
        } else {
            total += 1;
        }
    }

    total
}

fn explicit_expand_space(mut grid: Grid<u8>) -> Grid<u8> {
    let empty_rows: Vec<usize> = rows_to_expand(&grid);
    let empty_cols: Vec<usize> = cols_to_expand(&grid);

    let m = grid.n_cols();
    let mut shift = 0;
    let empty_row: Vec<u8> = (0..m).map(|_| b'.').collect();
    for i in empty_rows {
        grid.rows.insert(i + shift, empty_row.clone());
        shift += 1;
    }

    shift = 0;
    for j in empty_cols {
        for row in grid.rows.iter_mut() {
            row.insert(j + shift, b'.');
        }
        shift += 1;
    }

    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("res/day11_example.txt");

    #[test]
    fn test_part1() {
        let input = Day11::parse_input(EXAMPLE_INPUT);
        let output = Day11::part_1(input);
        assert_eq!(output, 374);
    }

    #[test]
    fn test_part2() {
        let input = Day11::parse_input(EXAMPLE_INPUT);
        let output = part2_solution(input, 100);
        assert_eq!(output, 8410);
    }
}
