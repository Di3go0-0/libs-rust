# Matrix

A comprehensive Rust implementation of a matrix data structure with common mathematical operations.

## Description

This library provides a fully-featured matrix implementation supporting basic matrix operations, linear algebra computations, and random matrix generation. It's designed for machine learning, scientific computing, and mathematical applications.

## Features

- **Matrix Creation:** Multiple constructors for different use cases
- **Basic Operations:** Addition, subtraction, multiplication, transpose
- **Advanced Operations:** Determinant, inverse, adjugate, pseudoinverse
- **Element-wise Operations:** Element-wise multiplication
- **Random Generation:** Random matrices with customizable ranges
- **Indexing Support:** Natural matrix[i][j] indexing syntax
- **Memory Efficient:** In-place operations where possible

## Usage

```rust
use matrix::Matrix;

// Create a 3x3 matrix
let mut matrix = Matrix::new(3, 3);

// Set values
matrix.set(0, 0, 1.0).unwrap();
matrix.set(0, 1, 2.0).unwrap();
matrix.set(1, 0, 3.0).unwrap();
matrix.set(1, 1, 4.0).unwrap();

// Matrix operations
let matrix2 = Matrix::random(3, 3);
let sum = matrix.add(&matrix2).unwrap();
let product = matrix.multiplication(&matrix2);
let transposed = matrix.transpose();

// Advanced operations
if matrix.is_square() {
    let det = matrix.determinant();
    let inverse = matrix.inverse();
}
```

## Key Methods

### Creation
- `new(rows, cols)` - Create empty matrix
- `from(vec)` - Create from vector (column matrix)
- `new2(data)` - Create from 2D vector
- `random(rows, cols)` - Random matrix (0.0 to 1.0)
- `random_range(rows, cols, min, max)` - Random matrix with range

### Basic Operations
- `get(row, col)` - Get element value
- `set(row, col, value)` - Set element value
- `add(&other)` - Matrix addition
- `subtract(&other)` - Matrix subtraction
- `multiplication(&other)` - Matrix multiplication
- `transpose()` - Matrix transpose

### Advanced Operations
- `determinant()` - Calculate determinant
- `inverse()` - Calculate inverse (or pseudoinverse for non-square)
- `adjugate()` - Calculate adjugate matrix
- `elementwise_multiply(&other)` - Element-wise multiplication

## Time Complexity

- **Creation:** O(n×m) for n×m matrix
- **Basic Access (get/set):** O(1)
- **Addition/Subtraction:** O(n×m)
- **Multiplication:** O(n×p×m) for n×m × m×p matrices
- **Transpose:** O(n×m)
- **Determinant:** O(n!) (recursive implementation)
- **Inverse:** O(n³) (through Gaussian elimination)

## Memory Complexity

O(n×m) - proportional to matrix size

## Implementation Details

- Uses `Vec<Vec<f64>>` for row-major storage
- Supports in-place operations with `*_mut` variants
- Uses He initialization for neural network weight initialization
- Implements `Display`, `Index`, and `IndexMut` traits for natural usage
- Bounds checking with graceful error handling using `Result` types