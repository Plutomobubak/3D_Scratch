use std::ops::{Add, Deref, DerefMut, Mul, Sub};
use std::sync::atomic::{AtomicUsize, Ordering};

// Global counter for matrix multiplications
static MATRIX_MUL_COUNT: AtomicUsize = AtomicUsize::new(0);
#[derive(Clone, Debug)]
pub struct Matrix(Vec<Vec<f32>>);

impl Deref for Matrix {
    type Target = Vec<Vec<f32>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Matrix {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<Vec<f32>>> for Matrix {
    fn from(vec: Vec<Vec<f32>>) -> Self {
        Matrix(vec)
    }
}
impl Into<[f32; 3]> for Matrix {
    fn into(self) -> [f32; 3] {
        [self[0][0], self[0][1], self[0][2]]
    }
}

impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Self) -> Self::Output {
        MATRIX_MUL_COUNT.fetch_add(1, Ordering::Relaxed);
        let mut res: Matrix = vec![vec![0.0; self[0].len()]; rhs.len()].into();
        for i in 0..rhs.len() {
            for j in 0..self[0].len() {
                for k in 0..rhs[0].len() {
                    res[i][j] += rhs[i][k] * self[k][j];
                }
            }
        }

        res
    }
}
impl Mul for &Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Self) -> Self::Output {
        MATRIX_MUL_COUNT.fetch_add(1, Ordering::Relaxed);
        let mut res: Matrix = vec![vec![0.0; self[0].len()]; rhs.len()].into();
        for i in 0..rhs.len() {
            for j in 0..self[0].len() {
                for k in 0..rhs[0].len() {
                    res[i][j] += rhs[i][k] * self[k][j];
                }
            }
        }

        res
    }
}
impl Mul<f32> for Matrix {
    type Output = Matrix;

    fn mul(self, scalar: f32) -> Self::Output {
        let mut res: Matrix = self.clone();
        for i in 0..self.len() {
            for j in 0..self[0].len() {
                res[i][j] *= scalar;
            }
        }
        res
    }
}
impl Mul<[f32; 3]> for &Matrix {
    type Output = [f32; 3];

    fn mul(self, v0: [f32; 3]) -> [f32; 3] {
        let mut v: Matrix = vec![v0.to_vec()].into();

        v[0].push(1.0);
        v = self * &v;
        [v[0][0], v[0][1], v[0][2]]
    }
}
impl Add for Matrix {
    type Output = Matrix;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res: Matrix = self.clone();
        for i in 0..self.len() {
            for j in 0..self[0].len() {
                res[i][j] += rhs[i][j];
            }
        }
        res
    }
}
impl Add for &Matrix {
    type Output = Matrix;
    fn add(self, rhs: Self) -> Self::Output {
        let mut res: Matrix = self.clone();
        for i in 0..self.len() {
            for j in 0..self[0].len() {
                res[i][j] += rhs[i][j];
            }
        }
        res
    }
}
impl Add<f32> for Matrix {
    type Output = Matrix;
    fn add(self, scalar: f32) -> Self::Output {
        let mut res: Matrix = self.clone();
        for i in 0..self.len() {
            for j in 0..self[0].len() {
                res[i][j] += scalar;
            }
        }
        res
    }
}
impl Sub for &Matrix {
    type Output = Matrix;
    fn sub(self, rhs: Self) -> Self::Output {
        let mut res: Matrix = self.clone();
        for i in 0..self.len() {
            for j in 0..self[0].len() {
                res[i][j] -= rhs[i][j];
            }
        }
        res
    }
}
impl Matrix {
    // Returns view matrix defined by z_offset
    pub fn trans(offset: [f32; 3]) -> Matrix {
        vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![offset[0], offset[1], offset[2], 1.0],
        ]
        .into()
    }
    pub fn rotate(self, rotation: [f32; 3]) -> Matrix {
        // Apply rotation
        let sina = rotation[0].sin();
        let cosa = rotation[0].cos();
        let sinb = rotation[1].sin();
        let cosb = rotation[1].cos();
        let sinc = rotation[2].sin();
        let cosc = rotation[2].cos();

        let rot_matrix: Matrix = vec![
            vec![
                cosb * cosc,
                (sina * sinb * cosc) - (cosa * sinc),
                (cosa * sinb * cosc) + (sina * sinc),
                0.0,
            ],
            vec![
                cosb * sinc,
                (sina * sinb * sinc) + (cosa * cosc),
                (cosa * sinb * sinc) - (sina * cosc),
                0.0,
            ],
            vec![-sinb, sina * cosb, cosa * cosb, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]
        .into();
        self * rot_matrix
    }
    // Creates projection matrix by given params
    pub fn projection(fov_y: f32, aspect_ratio: f32, near: f32, far: f32) -> Matrix {
        let f = 1.0 / (fov_y / 2.0).tan();
        let nf = 1.0 / (near - far);
        vec![
            vec![f / aspect_ratio, 0.0, 0.0, 0.0],
            vec![0.0, f, 0.0, 0.0],
            vec![0.0, 0.0, (far + near) * nf, -1.0],
            vec![0.0, 0.0, (2.0 * far * near) * nf, 0.0],
        ]
        .into()
    }
    // Identity
    pub fn identity() -> Matrix {
        vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]
        .into()
    }
    pub fn gaussian_elim(&self) -> (Self, u32) {
        let mut swaps = 0;
        let mut res = self.clone();
        let n = res.len();
        for i in 0..n {
            let mut max_row = i;
            for j in i + 1..n {
                if res[j][i].abs() > res[max_row][i].abs() {
                    max_row = j;
                }
            }
            if max_row != i {
                swaps += 1;
            }
            let temp = res[i].clone();
            res[i] = res[max_row].clone();
            res[max_row] = temp;

            if res[i][i] == 0.0 {
                return (res, swaps);
            }
            for j in i + 1..n {
                let a = res[j][i] / res[i][i];
                // println!("{:?}", a);
                for k in i..res[j].len() {
                    res[j][k] -= res[i][k] * a;
                }
            }
            // println!("{:?}", res);
        }
        (res, swaps)
    }
    pub fn det(&self) -> f32 {
        let elim = self.gaussian_elim();
        let mut a = 1.0;
        for i in 0..self.len() {
            a *= elim.0[i][i];
        }
        a / (-2.0 * ((elim.1 % 2) as f32) + 1.0)
    }
    pub fn inverse(&self) -> Self {
        let mut augmat = self.clone();

        // Create an identity matrix of the same size
        let n = augmat.len();
        for i in 0..n {
            augmat[i].extend(vec![0.0; n]); // Extend each row by n elements
            augmat[i][i + n] = 1.0; // Identity on the right side
        }

        // Perform Gauss-Jordan elimination
        for i in 0..n {
            // Find the row with the maximum element in the i-th column
            let mut max_row = i;
            for j in i + 1..n {
                if augmat[j][i].abs() > augmat[max_row][i].abs() {
                    max_row = j;
                }
            }

            // Swap rows if necessary
            if max_row != i {
                augmat.swap(i, max_row);
            }

            // If the pivot is zero, the matrix is singular (no inverse)
            if augmat[i][i] == 0.0 {
                return augmat; // No inverse exists
            }

            // Normalize the pivot row
            let pivot = augmat[i][i];
            for j in 0..2 * n {
                augmat[i][j] /= pivot;
            }

            // Eliminate the i-th column in all other rows
            for j in 0..n {
                if j != i {
                    let factor = augmat[j][i];
                    for k in 0..2 * n {
                        augmat[j][k] -= augmat[i][k] * factor;
                    }
                }
            }
        }

        // Extract the inverse matrix (right half of the augmented matrix)
        let mut inverse = Vec::with_capacity(n);
        for i in 0..n {
            let mut row = Vec::with_capacity(n);
            for j in n..2 * n {
                row.push(augmat[i][j]);
            }
            inverse.push(row);
        }
        inverse.into()
    }
    pub fn transpose(&mut self) -> Self {
        let n = self.len();
        let mut transposed: Matrix = vec![vec![0.0; n]; n].into();

        for i in 0..n {
            for j in 0..n {
                transposed[i][j] = self[j][i];
            }
        }

        transposed
    }
}
pub fn get_matrix_mul_count() -> usize {
    MATRIX_MUL_COUNT.load(Ordering::Relaxed)
}
pub fn reset_matrix_mul_count() {
    MATRIX_MUL_COUNT.store(0, Ordering::Relaxed);
}
