use num_traits::{Num, Zero};
use std::ops::{Add, Mul, Sub};

/// Calcula la matriz transpuesta
///
/// # Arguments
/// * `matrix` - Vector de vectores que representa la matriz original
///
/// # Returns
/// * `Vec<Vec<f64>>` - La matriz transpuesta
pub fn transpose<T>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Num + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut result = vec![vec![T::zero(); rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            result[j][i] = matrix[i][j];
        }
    }

    result
}
