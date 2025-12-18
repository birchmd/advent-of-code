use crate::gcd;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerMatrix {
    pub rows: Vec<Vec<i64>>,
}

impl IntegerMatrix {
    pub fn n_rows(&self) -> usize {
        self.rows.len()
    }

    pub fn n_cols(&self) -> usize {
        self.rows.first().map(|r| r.len()).unwrap_or_default()
    }

    pub fn evaluate_augmented_row<I>(&self, var: (usize, usize), values: I) -> Option<i64>
    where
        I: IntoIterator<Item = (usize, i64)>,
    {
        let m = self.n_cols();
        let (i, j) = var;
        let row = &self.rows[i];
        let coeff = row[j];
        let total = values
            .into_iter()
            .fold(row[m - 1], |acc, (col, value)| acc - row[col] * value);
        if total % coeff == 0 {
            Some(total / coeff)
        } else {
            None
        }
    }

    // Row reduce the matrix (as best we can without fractions),
    // returning the column indices of the free variables and the
    // positions in the matrix of the fixed variable coefficients.
    pub fn partial_row_reduce(&mut self) -> (Vec<usize>, Vec<(usize, usize)>) {
        let n = self.n_rows();
        let m = self.n_cols();

        let mut buf: Vec<i64> = vec![0; m];
        let mut fixed_vars = Vec::new();
        let mut free_vars = Vec::new();
        let mut current_row = 0;
        for current_col in 0..m {
            let Some(_) = self.find_pivot_row(current_row, current_col) else {
                free_vars.push(current_col);
                continue;
            };

            // Ensure the leading coefficient is positive
            if self.rows[current_row][current_col] < 0 {
                for x in self.rows[current_row].iter_mut() {
                    *x *= -1;
                }
            }

            // Try to divide out a common factor if possible
            let leading_coeff = self.divide_out_common_factor(current_row, current_col);

            // Zero out other rows
            for r in 0..n {
                if r == current_row || self.rows[r][current_col] == 0 {
                    continue;
                }

                let d = gcd(leading_coeff, self.rows[r][current_col].unsigned_abs());
                let a = (leading_coeff / d) as i64;
                let b = self.rows[r][current_col] / (d as i64);
                for (t, (x, y)) in buf
                    .iter_mut()
                    .zip(self.rows[r].iter().zip(&self.rows[current_row]))
                {
                    *t = a * x - b * y;
                }
                std::mem::swap(&mut buf, &mut self.rows[r]);
            }

            fixed_vars.push((current_row, current_col));
            current_row += 1;
        }

        // After reduction is finished, do a final pass to divide out any common factors.
        for (r, c) in &fixed_vars {
            self.divide_out_common_factor(*r, *c);
        }

        (free_vars, fixed_vars)
    }

    fn find_pivot_row(&mut self, current_row: usize, current_col: usize) -> Option<usize> {
        let n = self.n_rows();
        for r in current_row..n {
            if self.rows[r][current_col] != 0 {
                self.rows.swap(r, current_row);
                return Some(r);
            }
        }
        None
    }

    // Try to divide out a common factor from the `current_row` using the element in
    // `current_col` as the first term.
    // This function returns that matrix element after the division is done.
    fn divide_out_common_factor(&mut self, current_row: usize, current_col: usize) -> u64 {
        let leading_coeff = self.rows[current_row][current_col] as u64;
        let d = self.rows[current_row].iter().fold(leading_coeff, |acc, x| {
            if *x == 0 {
                acc
            } else {
                gcd(acc, x.unsigned_abs())
            }
        }) as i64;
        if d > 1 {
            for x in self.rows[current_row].iter_mut() {
                *x /= d;
            }
        }
        self.rows[current_row][current_col] as u64
    }
}

#[test]
fn test_row_reduce() {
    let mut matrix = IntegerMatrix {
        rows: vec![
            vec![1, 1, 0, 1, 1, 1, 1, 81],
            vec![0, 1, 0, 1, 0, 0, 1, 41],
            vec![0, 1, 1, 0, 0, 1, 0, 31],
            vec![0, 1, 0, 1, 1, 1, 1, 72],
            vec![0, 1, 1, 0, 1, 0, 0, 36],
            vec![0, 1, 0, 0, 0, 0, 1, 21],
            vec![1, 1, 1, 0, 0, 1, 1, 55],
            vec![1, 0, 0, 1, 0, 0, 0, 29],
            vec![1, 0, 0, 1, 0, 0, 1, 44],
        ],
    };
    let (free_vars, fixed_vars) = matrix.partial_row_reduce();

    assert_eq!(&free_vars, &[7]);
    for (i, (row, col)) in fixed_vars.into_iter().enumerate() {
        assert_eq!(i, row);
        assert_eq!(row, col);

        assert!(matrix.rows[row][0..col].iter().all(|x| *x == 0));
        assert_eq!(matrix.rows[row][col], 1);
        assert!(matrix.rows[row][(col + 1)..7].iter().all(|x| *x == 0));
    }

    let solution = [9, 6, 12, 20, 18, 13, 15, 0, 0];
    for (row, answer) in matrix.rows.iter().zip(&solution) {
        assert_eq!(row.last().unwrap(), answer);
    }
}
