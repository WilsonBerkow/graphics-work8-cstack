use std::fmt;
use std::ops::{Add, Sub, Mul};
use std::iter::Iterator;

/// 4xN matrices
#[derive(Clone)]
pub struct Matrix {
    v: Vec<[f64; 4]>
}

impl Matrix {
    /// Make a 4xN matrix.
    pub fn new(columns: Vec<[f64; 4]>) -> Matrix {
        Matrix { v: columns }
    }

    /// Make an empty (4x0) matrix.
    pub fn empty() -> Matrix {
        Matrix::new(vec![])
    }

    /// Make the column matrix representing the origin.
    pub fn origin() -> Matrix {
        Matrix::new(vec![[0.0, 0.0, 0.0, 1.0]])
    }

    /// Make a 4x4 matrix given each cell value (listed
    /// row-by-row).
    pub fn new4x4(
        a: f64, b: f64, c: f64, d: f64,
        e: f64, f: f64, g: f64, h: f64,
        i: f64, j: f64, k: f64, l: f64,
        m: f64, n: f64, o: f64, p: f64) -> Matrix {
        Matrix {
            v: vec![
                [a, e, i, m],
                [b, f, j, m],
                [c, g, k, o],
                [d, h, l, p]
            ]
        }
    }

    /// Make a 4x4 dilation matrix dilating by `s` in
    /// x, y, and z.
    pub fn dilation(s: f64) -> Matrix {
        s * &Matrix::identity()
    }

    /// Make a 4x4 dilation matrix dilating by `sx` in
    /// x, `sy`, in y, and `sz` in z.
    pub fn dilation_xyz(sx: f64, sy: f64, sz: f64) -> Matrix {
        Matrix::new4x4(
            sx, 0.0, 0.0, 0.0,
            0.0, sy, 0.0, 0.0,
            0.0, 0.0, sz, 0.0,
            0.0, 0.0, 0.0, 1.0)
    }

    /// Make a 4x4 identity matrix
    pub fn identity() -> Matrix {
        Matrix::new(vec![
                    [1.0, 0.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0, 0.0],
                    [0.0, 0.0, 1.0, 0.0],
                    [0.0, 0.0, 0.0, 1.0]])
    }

    pub fn col(&self, colnum: usize) -> [f64; 4] {
        let width = self.v.len();
        if colnum > width {
            panic!("Attempted to get column {} of a matrix of width {}", colnum, width);
        }
        self.v[colnum]
    }

    pub fn col_vec(&self, colnum: usize) -> Vec<f64> {
        let width = self.v.len();
        if colnum > width {
            panic!("Attempted to get column {} of a matrix of width {}", colnum, width);
        }
        let col = &self.v[colnum];
        vec![col[0], col[1], col[2], col[3]] // TODO: Into<Vec<T>>?
    }

    /// Push a column to the right side of `self`.
    pub fn push_col(&mut self, col: [f64; 4]) {
        self.v.push(col)
    }

    /// Push each column of `m` to `self`
    pub fn append(&mut self, m: Matrix) {
        for col in 0..m.width() {
            self.push_col(m.col(col));
        }
    }

    /// Push an edge, i.e. two points, to `self` (think of `self` as an edge list).
    pub fn push_edge(&mut self, colA: [f64; 4], colB: [f64; 4]) {
        self.push_col(colA);
        self.push_col(colB);
    }

    pub fn row(&self, rownum: usize) -> Vec<f64> {
        if rownum > 3 {
            panic!("Attempted to get row {} of a matrix of height 4", rownum);
        }
        let mut items = vec![];
        for column in &self.v {
            items.push(column[rownum]);
        }
        items
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.v[col][row]
    }


    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        self.v[col][row] = val;
    }

    pub fn width(&self) -> usize {
        self.v.len()
    }
}

// ref plus ref
impl<'a, 'b> Add<&'a Matrix> for &'b Matrix {
    type Output = Matrix;
    /// Add two matrices, assuming they are of the same width
    fn add(self, rhs: &Matrix) -> Matrix {
        let mut v = self.v.clone();
        for (vcol, rcol) in v.iter_mut().zip(rhs.v.iter()) {
            vcol[0] += rcol[0];
            vcol[1] += rcol[1];
            vcol[2] += rcol[2];
            vcol[3] += rcol[3];
        }
        Matrix::new(v)
    }
}

// owned plus ref
impl<'a> Add<&'a Matrix> for Matrix {
    type Output = Matrix;
    /// Add two matrices, assuming they are of the same width
    fn add(self, rhs: &Matrix) -> Matrix {
        &self + rhs
    }
}

// ref plus owned
impl<'a> Add<Matrix> for &'a Matrix {
    type Output = Matrix;
    /// Add two matrices, assuming they are of the same width
    fn add(self, rhs: Matrix) -> Matrix {
        self + &rhs
    }
}

// owned plus owned
impl Add<Matrix> for Matrix {
    type Output = Matrix;
    /// Add two matrices, assuming they are of the same width
    fn add(self, rhs: Matrix) -> Matrix {
        &self + &rhs
    }
}

// TODO: add owned version of impls for Sub and Mul (as done with Add above)
impl<'a, 'b> Sub<&'a Matrix> for &'b Matrix {
    type Output = Matrix;
    /// Add two matrices, assuming they are of the same width
    fn sub(self, rhs: &Matrix) -> Matrix {
        let mrhs = rhs * -1.0;
        self + &mrhs
    }
}

impl<'a, 'b> Mul<&'a Matrix> for &'b Matrix {
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Matrix {
        let mut m = Matrix::new(vec![[0.0; 4]; 4]);
        for i in 0..4 {
            for j in 0..rhs.width() {
                let val: f64 = dot_product_refs(self.row(i).iter(), rhs.col(j).iter());
                m.set(i, j, val);
            }
        }
        m
    }
}

fn dot_product_refs<'a, 'b, T: Iterator<Item=&'a f64>, U: Iterator<Item=&'b f64>>(v: T, u: U) -> f64 {
    let mut sum = 0.0;
    for (&a, &b) in v.zip(u) {
        sum += a * b;
    }
    sum
}

fn dot_product<T: Iterator<Item=f64>, U: Iterator<Item=f64>>(v: T, u: U) -> f64 {
    let mut sum = 0.0;
    for (a, b) in v.zip(u) {
        sum += a * b;
    }
    sum
}

fn scale_matrix(scalar: f64, mat: &Matrix) -> Matrix {
    let mut result = Matrix::new(vec![]);
    for row in 0..4 {
        for col in 0..mat.width() {
            result.set(row, col, scalar * mat.get(row, col));
        }
    }
    result
}

impl<'a> Mul<f64> for &'a Matrix {
    type Output = Matrix;
    fn mul(self, rhs: f64) -> Matrix {
        scale_matrix(rhs, self)
    }
}

impl<'a> Mul<&'a Matrix> for f64 {
    type Output = Matrix;
    fn mul(self, rhs: &Matrix) -> Matrix {
        scale_matrix(self, rhs)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::from("");
        for row in 0..4 {
            s.push_str(match row {
                0 => "/ ",
                3 => "\\ ",
                _ => "| "
            });
            for col in 0..self.width() {
                s.push_str(&format!("{} ", self.get(row, col)));
            }
            s.push_str(match row {
                0 => "\\\n",
                3 => "/\n",
                _ => "|\n"
            });
        }
        write!(f, "{}", s)
    }
}