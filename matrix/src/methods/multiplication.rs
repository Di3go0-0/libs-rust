/// Multiplica dos matrices
///
/// # Arguments
/// * `matrix_a` - Primera matriz (m x n)
/// * `matrix_b` - Segunda matriz (n x p)
///
/// # Returns
/// * `Option<Vec<Vec<f64>>>` - Resultado de la multiplicación (m x p) o None si las dimensiones no son compatibles
pub fn multiply(matrix_a: &Vec<Vec<f64>>, matrix_b: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    // Verificamos que las matrices no estén vacías

    let m = matrix_a.len(); // Filas de A
    let n = matrix_a[0].len(); // Columnas de A
    let p = matrix_b[0].len(); // Columnas de B

    // Verificamos que las dimensiones sean compatibles

    // Creamos la matriz resultado (m x p)
    let mut result = vec![vec![0.0; p]; m];

    // Realizamos la multiplicación
    for i in 0..m {
        for j in 0..p {
            for k in 0..n {
                result[i][j] += matrix_a[i][k] * matrix_b[k][j];
            }
        }
    }

    result
}
