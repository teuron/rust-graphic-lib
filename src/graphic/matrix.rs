/*
use std;
use std::ops::Mul;

pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    // A public constructor method
    pub fn new(rows: usize, cols: usize, data: Vec<Vec<f64>>) -> Matrix {
        assert_eq!(rows, cols);
        Matrix {
            rows: rows,
            cols: cols,
            data: data
        }
    }
    pub fn identity_matrix(rc: usize) -> Matrix {
        let mut data = Vec::new();
        for a in 0..rc {
            let mut v = Vec::new();
            for b in 0..rc {
                if a == b { v.push(1.0) } else { v.push(0.0) }
            }
            data.push(v);
        }
        Matrix {
            rows: rc,
            cols: rc,
            data: data
        }
    }
}

impl Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        assert_eq!(self.rows, rhs.rows);
        assert_eq!(self.cols, rhs.cols);
        let mut data = Vec::new();
        for a in 0..self.rows {
            let mut v = Vec::new();
            for b in 0..self.cols {
                let mut sum = 0.0;
                for c in 0..self.rows {
                    sum = sum + self.data[a][c] * rhs.data[c][b];
                }
                v.push(sum);
            }
            data.push(v);
        }
        Matrix::new(self.rows, self.cols, data)
    }
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = "Matrix: \n".to_string();
        for r in 0..self.rows {
            for c in 0..self.cols {
                s.push_str(&*self.data[c][r].to_string());
                s.push_str("\t");
            }
            s.push_str("\n");
        }
        write!(f, "{}", s)
    }
}*/
