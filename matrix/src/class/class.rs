use super::methods::determinant;

#[derive(Debug, Clone)]
pub struct Matrix {
    size_x: usize,
    size_y: usize,
    data: Vec<Vec<f64>>,
}

impl Matrix {
    // Constructor
    pub fn new(size_x: usize, size_y: usize) -> Self {
        Matrix {
            size_x,
            size_y,
            data: vec![vec![0.0; size_y]; size_x], // size_x filas, size_y columnas
        }
    }

    // Get specific value donde x es fila, y es columna
    pub fn get(&self, x: usize, y: usize) -> Option<f64> {
        if x < self.size_x && y < self.size_y {
            Some(self.data[x][y]) // x accede a la fila, y a la columna
        } else {
            None
        }
    }

    // set in a specific value donde x es fila, y es columna
    pub fn set(&mut self, x: usize, y: usize, value: f64) -> Result<(), &'static str> {
        if x < self.size_x && y < self.size_y {
            self.data[x][y] = value; // x accede a la fila, y a la columna
            Ok(())
        } else {
            Err("Fuera de rango")
        }
    }

    // Obtain a determinant of matrix
    pub fn determinant(&self) -> f64 {
        let det: f64 = determinant::determinant(&self.data); // Esto está bien porque self.data ya es Vec<Vec<f64>>
        det
    }
}

// Implementación para poder imprimir la matriz
impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for x in 0..self.size_x {
            // Iteramos primero por filas
            write!(f, "[")?;
            for y in 0..self.size_y {
                // Luego por columnas
                if y > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:>8.2}", self.get(x, y).unwrap())?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}
