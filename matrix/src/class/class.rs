use super::methods::adjugate::adjugate;
use super::methods::determinant::determinant;
use super::methods::inverse::inverse;
use super::methods::multiplication::multiply;
use super::methods::pseudoinverse::pseudo_inverse;
use super::methods::transpose::transpose;

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

    // set vector like a row
    pub fn set_row(&mut self, row: usize, values: Vec<f64>) -> Result<(), &'static str> {
        if row >= self.size_x {
            return Err("Out of range");
        }
        if values.len() != self.size_y {
            return Err("Different Column sizes");
        }
        self.data[row] = values;
        Ok(())
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
        let det: f64 = determinant(&self.data); // Esto está bien porque self.data ya es Vec<Vec<f64>>
        det
    }

    // Obtain a traspose of matrix
    pub fn transpose(&self) -> Matrix {
        let trans_data = transpose(&self.data);
        Matrix {
            size_x: self.size_y,
            size_y: self.size_x,
            data: trans_data,
        }
    }

    // Obtain adjugate of matrix
    pub fn adjugte(&self) -> Matrix {
        let adju_data = adjugate(&self.data);
        Matrix {
            size_x: self.size_y,
            size_y: self.size_x,
            data: adju_data,
        }
    }
    // Obtain adjugate of matrix
    pub fn inverse(&self) -> Matrix {
        let mut inverse_data: Vec<Vec<f64>>;
        if self.is_square() {
            // Si la matriz es cuadrada, simplemente devolvemos su inversa
            inverse_data = inverse(&self.data);
            Matrix {
                size_x: self.size_y,
                size_y: self.size_x,
                data: inverse_data,
            }
        } else {
            inverse_data = pseudo_inverse(&self.data);
            let len = inverse_data.len();
            Matrix {
                size_x: len,
                size_y: if len > 0 { inverse_data[0].len() } else { 0 },
                data: inverse_data,
            }
        }
    }
    // multiplication of matrix
    pub fn multiplication(&self, data: Matrix) -> Matrix {
        let multi = multiply(&self.data, &data.data);
        let len = multi.len();
        Matrix {
            size_x: len,
            size_y: if len > 0 { multi[0].len() } else { 0 },
            data: multi,
        }
    }

    pub fn is_square(&self) -> bool {
        if self.size_x == self.size_y {
            return true;
        }
        false
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
