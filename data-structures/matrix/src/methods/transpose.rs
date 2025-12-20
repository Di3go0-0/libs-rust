/// Calcula la matriz transpuesta
///
/// # Arguments
/// * `matrix` - Vector de vectores que representa la matriz original
///
/// # Returns
/// * `Vec<Vec<f64>>` - La matriz transpuesta
pub fn transpose(matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return vec![];
    }

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut result = vec![vec![0.0; rows]; cols];

    for i in 0..rows {
        for j in 0..cols {
            result[j][i] = matrix[i][j];
        }
    }

    result
}
