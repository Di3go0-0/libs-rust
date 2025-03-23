use super::determinant::determinant;
use num_traits::{Float, Num, One, Zero};
use std::default::Default;
use std::ops::{Add, Mul, Neg, Sub};
/// Calcula la matriz adjunta (matriz de cofactores)
///
/// # Arguments
/// * `matrix` - Vector de vectores que representa la matriz cuadrada
///
/// # Returns
/// * `Vec<Vec<f64>>` - La matriz adjunta
pub fn adjugate<T>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Num
        + Copy
        + Zero
        + One
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Float
        + Default,
{
    let n = matrix.len();
    if n == 0 || matrix[0].len() != n {
        return vec![];
    }

    // Para una matriz 1x1, la adjunta es [1]
    if n == 1 {
        return vec![vec![T::one()]];
    }

    let mut result = vec![vec![T::zero(); n]; n];

    for i in 0..n {
        for j in 0..n {
            result[i][j] = cofactor(matrix, i, j);
        }
    }

    result
}

/// Calcula el cofactor para la posición (i,j)
fn cofactor<T>(matrix: &Vec<Vec<T>>, row: usize, col: usize) -> T
where
    T: Num
        + Copy
        + Zero
        + One
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + Float
        + Default,
{
    let sign = if (row + col) % 2 == 0 {
        T::one()
    } else {
        -T::one()
    };
    let minor = get_minor(matrix, row, col);
    sign * determinant(&minor)
}

/// Obtiene la matriz menor eliminando la fila y columna especificadas
fn get_minor<T>(matrix: &Vec<Vec<T>>, row: usize, col: usize) -> Vec<Vec<T>>
where
    T: Num
        + Copy
        + Zero
        + One
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Float
        + Default,
{
    let n = matrix.len();
    let mut minor = Vec::new();

    for i in 0..n {
        if i == row {
            continue;
        }
        let mut new_row = Vec::new();
        for j in 0..n {
            if j == col {
                continue;
            }
            new_row.push(matrix[i][j]);
        }
        minor.push(new_row);
    }

    minor
}
