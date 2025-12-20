use crate::Matrix;

pub fn add(m1: &Matrix, m2: &Matrix) -> Matrix {
    let mut matrix = Matrix::new(m1.size_x, m1.size_y);

    for i in 0..m1.size_x {
        for j in 0..m1.size_y {
            matrix[i][j] = m1[i][j] + m2[i][j];
        }
    }
    matrix
}
