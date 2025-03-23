use super::{adjugate::adjugate, determinant::determinant, transpose::transpose};

/// Calcula la inversa de una matriz usando la fórmula: A⁻¹ = adj(Aᵀ)/det(A)
///
/// # Arguments
/// * `matrix` - Matriz de entrada como Vec<Vec<f64>>
///
/// # Returns
/// * `Option<Vec<Vec<f64>>>` - Some(matriz_inversa) si existe, None si no existe
pub fn inverse(matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let n = matrix.len();

    // Calculamos el determinante
    let det = determinant(matrix);

    // 1. Calculamos la transpuesta
    let transposed = transpose(matrix);

    // 2. Calculamos la adjunta de la transpuesta
    let adj_trans = adjugate(&transposed);

    // 3. Dividimos por el determinante
    let mut inverse = vec![vec![0.0; n]; n];
    let inv_det = 1.0 / det;

    for i in 0..n {
        for j in 0..n {
            inverse[i][j] = adj_trans[i][j] * inv_det;
        }
    }

    inverse
}
