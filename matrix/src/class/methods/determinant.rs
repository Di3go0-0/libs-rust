use num_traits::{Float, One, Zero};
use std::ops::{Add, Div, Mul, Sub}; // Se usa para verificar si T es un número
/// Calcula el determinante de una matriz cuadrada
///
/// # Arguments
/// * `matrix` - Vector de vectores que representa la matriz cuadrada
///
/// # Returns
/// * `T` - El determinante de la matriz, o `T::default()` si no es un número
pub fn determinant<T>(matrix: &Vec<Vec<T>>) -> T
where
    T: Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Default
        + PartialEq
        + Zero
        + One
        + Float,
{
    let n = matrix.len();

    // Caso base: matriz 1x1
    if n == 1 {
        return matrix[0][0].clone();
    }

    // Caso base: matriz 2x2
    if n == 2 {
        return matrix[0][0].clone() * matrix[1][1].clone()
            - matrix[0][1].clone() * matrix[1][0].clone();
    }

    let mut det: T = T::zero();

    // Expandimos por la primera fila
    for j in 0..n {
        let cofactor_value = cofactor(matrix, 0, j);
        det = det + matrix[0][j].clone() * cofactor_value;
    }

    det
}

/// Calcula el cofactor para un elemento específico de la matriz
fn cofactor<T>(matrix: &Vec<Vec<T>>, row: usize, col: usize) -> T
where
    T: Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Default
        + PartialEq
        + Zero
        + One
        + Float,
{
    let sign = if (row + col) % 2 == 0 {
        T::one()
    } else {
        -T::one()
    };
    sign * determinant(&get_minor(matrix, row, col))
}

/// Obtiene la matriz menor eliminando una fila y columna específicas
fn get_minor<T>(matrix: &Vec<Vec<T>>, row: usize, col: usize) -> Vec<Vec<T>>
where
    T: Clone,
{
    let n: usize = matrix.len();
    let mut minor: Vec<Vec<T>> = Vec::new();

    for i in 0..n {
        if i == row {
            continue;
        }

        let mut new_row = Vec::new();
        for j in 0..n {
            if j == col {
                continue;
            }
            new_row.push(matrix[i][j].clone());
        }
        minor.push(new_row);
    }

    minor
}
