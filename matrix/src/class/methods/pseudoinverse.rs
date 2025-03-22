use super::{inverse::inverse, multiplication::multiply, transpose::transpose};

/// Calcula la matriz pseudo-inversa de Moore-Penrose
///
/// # Arguments
/// * `matrix` - Matriz de entrada como Vec<Vec<f64>>
///
/// # Returns
/// * `Vec<Vec<f64>>` - La matriz pseudo-inversa
pub fn pseudo_inverse(matrix: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let rows = matrix.len();
    let cols = if rows > 0 { matrix[0].len() } else { 0 };

    if rows == cols {
        // Si la matriz es cuadrada, simplemente devolvemos su inversa
        return inverse(matrix);
    }

    let transposed = transpose(matrix);

    if rows > cols {
        // Caso m > n: A^+ = (A^T A)^(-1) A^T
        let ata = multiply(&transposed, matrix);
        let ata_inv = inverse(&ata);
        multiply(&ata_inv, &transposed)
    } else {
        // Caso m < n: A^+ = A^T (A A^T)^(-1)
        let aat = multiply(matrix, &transposed);
        let aat_inv = inverse(&aat);
        multiply(&transposed, &aat_inv)
    }
}
