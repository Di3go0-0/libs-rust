use class::class::Matrix;

mod class;
fn main() {
    let mut matrix = Matrix::new(3, 3);
    println!("Matrix:\n{}", matrix);

    matrix.set(0, 0, 2.0).unwrap();
    matrix.set(0, 1, -2.0).unwrap();
    matrix.set(0, 2, 2.0).unwrap();
    matrix.set(1, 0, 2.0).unwrap();
    matrix.set(1, 1, 1.0).unwrap();
    matrix.set(1, 2, 0.0).unwrap();
    matrix.set(2, 0, 3.0).unwrap();
    matrix.set(2, 1, -2.0).unwrap();
    matrix.set(2, 2, 2.0).unwrap();

    println!("Matrix:\n{}", matrix);

    let det = matrix.determinant();
    println!("Det: {}", det);
    let trans = matrix.transpose();
    println!("trans: \n{}", trans);
    let adju = trans.adjugte();
    println!("adju: \n{}", adju);
}
