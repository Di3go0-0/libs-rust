/// Calcula el determinante de una matriz cuadrada
///
/// # Arguments
/// * `matrix` - Vector de vectores que representa la matriz cuadrada
///
/// # Returns
/// * `f64` - El determinante de la matriz
pub fn determinant(matrix: &Vec<Vec<f64>>) -> f64 {
    let n = matrix.len();

    // Caso base: matriz 1x1
    if n == 1 {
        return matrix[0][0];
    }

    // Caso base: matriz 2x2
    if n == 2 {
        return matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
    }

    let mut det = 0.0;

    // Expandimos por la primera fila
    for j in 0..n {
        det += matrix[0][j] * cofactor(&matrix, 0, j);
    }

    det
}

/// Calcula el cofactor para un elemento específico de la matriz
fn cofactor(matrix: &Vec<Vec<f64>>, row: usize, col: usize) -> f64 {
    let sign = if (row + col) % 2 == 0 { 1.0 } else { -1.0 };
    sign * determinant(&get_minor(matrix, row, col))
}

/// Obtiene la matriz menor eliminando una fila y columna específicas
fn get_minor(matrix: &Vec<Vec<f64>>, row: usize, col: usize) -> Vec<Vec<f64>> {
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
