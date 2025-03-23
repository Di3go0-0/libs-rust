use num_traits::{Float, Num, One, Zero};

use super::methods::adjugate::adjugate;
use super::methods::determinant::determinant;
use super::methods::inverse::inverse;
use super::methods::multiplication::multiply;
use super::methods::pseudoinverse::pseudo_inverse;
use super::methods::transpose::transpose;
use core::f64;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Matrix<T>
where
    T: Clone
        + Display
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Default
        + PartialEq,
{
    size_x: usize,
    size_y: usize,
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Clone
        + Display
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Default
        + PartialEq,
{
    // Constructor
    pub fn new(size_x: usize, size_y: usize) -> Self {
        Matrix {
            size_x,
            size_y,
            data: vec![vec![T::default(); size_y]; size_x], // size_x filas, size_y columnas
        }
    }

    fn convert_to_f64_if_needed(&self) -> Matrix<f64>
    where
        T: Copy + Into<f64>, // Usa Into<f64> en lugar de From<T>
    {
        let new_data = self
            .data
            .iter()
            .map(|row| row.iter().map(|&x| x.into()).collect()) // Usa x.into() en lugar de f64::from(x)
            .collect();

        Matrix {
            size_x: self.size_x,
            size_y: self.size_y,
            data: new_data,
        }
    }

    fn convert_from_f64_if_needed(matrix: Matrix<f64>) -> Matrix<T>
    where
        T: From<f64> + Copy,
    {
        let new_data = matrix
            .data
            .into_iter()
            .map(|row| row.into_iter().map(T::from).collect())
            .collect();

        Matrix {
            size_x: matrix.size_x,
            size_y: matrix.size_y,
            data: new_data,
        }
    }
    // pasar a f64
    pub fn to_f64(&self) -> Matrix<f64>
    where
        T: Copy,
        f64: From<T>,
    {
        let new_data = self
            .data
            .iter()
            .map(|row| row.iter().map(|&x| f64::from(x)).collect())
            .collect();

        Matrix {
            size_x: self.size_x,
            size_y: self.size_y,
            data: new_data,
        }
    }

    // set vector like a row
    pub fn set_row(&mut self, row: usize, values: Vec<T>) -> Result<(), &'static str> {
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
    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        if x < self.size_x && y < self.size_y {
            Some(self.data[x][y].clone()) // x accede a la fila, y a la columna
        } else {
            None
        }
    }

    // set in a specific value donde x es fila, y es columna
    pub fn set(&mut self, x: usize, y: usize, value: T) -> Result<(), &'static str> {
        if x < self.size_x && y < self.size_y {
            self.data[x][y] = value; // x accede a la fila, y a la columna
            Ok(())
        } else {
            Err("Fuera de rango")
        }
    }

    // Obtain a determinant of matrix
    pub fn determinant(&self) -> T
    where
        T: Num + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Float + 'static,
    {
        if std::any::TypeId::of::<T>() == std::any::TypeId::of::<i32>() {
            let matrix_f64 = self.convert_to_f64_if_needed();
            let det_f64 = determinant(&matrix_f64.data);
            // Convertimos el resultado de vuelta al tipo T
            T::from(det_f64).unwrap_or(T::zero())
        } else {
            determinant(&self.data)
        }
    }

    // Obtain a traspose of matrix
    pub fn transpose(&self) -> Matrix<T>
    where
        T: Num
            + Copy
            + Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Float
            + Default
            + Clone
            + Into<f64>
            + From<f64>,
    {
        let trans_data: Vec<Vec<T>> = transpose(&self.data);
        if trans_data.is_empty() {
            return Matrix {
                size_x: 0,
                size_y: 0,
                data: vec![], // Retorna una matriz vacía si la entrada no es válida
            };
        }
        Matrix {
            size_x: self.size_y,
            size_y: self.size_x,
            data: trans_data,
        }
    }

    // Obtain adjugate of matrix
    pub fn adjugte(&self) -> Matrix<T>
    where
        T: Num + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Float + Default,
    {
        let adju_data = adjugate(&self.data);
        Matrix {
            size_x: self.size_y,
            size_y: self.size_x,
            data: adju_data,
        }
    }
    // Obtain adjugate of matrix
    pub fn inverse(&self) -> Matrix<T>
    where
        T: Num + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Float + Default,
    {
        let inverse_data: Vec<Vec<T>>;
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
    pub fn multiplication(&self, data: Matrix<T>) -> Matrix<T>
    where
        T: Num + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Float + Default,
    {
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

/// Implementación de `Display` para imprimir la matriz
impl<T> Display for Matrix<T>
where
    T: Clone
        + Num
        + Display
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Default
        + PartialEq
        + Zero
        + One,
{
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        for row in &self.data {
            write!(f, "[")?;
            for (i, value) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", value)?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

// Implementation indexation to use matrix[i][j]
impl<T> std::ops::Index<usize> for Matrix<T>
where
    T: Clone
        + Display
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Default
        + PartialEq,
{
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

// Implementation indexation to reemplace in Matric[i][j] = value
impl<T> std::ops::IndexMut<usize> for Matrix<T>
where
    T: Clone
        + Display
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Default
        + PartialEq,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
