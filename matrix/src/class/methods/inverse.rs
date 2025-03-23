use super::{adjugate::adjugate, determinant::determinant, transpose::transpose};
use num_traits::{Float, Num, One, Zero};
use std::ops::{Add, Div, Mul, Sub};
/// Calcula la inversa de una matriz usando la fórmula: A⁻¹ = adj(Aᵀ)/det(A)
///
/// # Arguments
/// * `matrix` - Matriz de entrada como Vec<Vec<f64>>
///
/// # Returns
/// * `Option<Vec<Vec<f64>>>` - Some(matriz_inversa) si existe, None si no existe
pub fn inverse<T>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Num
        + Copy
        + Zero
        + One
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Float
        + Default,
{
    let n = matrix.len();

    // Calculamos el determinante
    let det = determinant(matrix);
    if det == T::zero() {
        return vec![]; // No tiene inversa
    }

    // 1. Calculamos la transpuesta
    let transposed = transpose(matrix);

    // 2. Calculamos la adjunta de la transpuesta
    let adj_trans = adjugate(&transposed);

    // 3. Dividimos por el determinante
    let mut inverse = vec![vec![T::zero(); n]; n];
    let inv_det = T::one() / det;

    for i in 0..n {
        for j in 0..n {
            inverse[i][j] = adj_trans[i][j] * inv_det;
        }
    }

    inverse
}
