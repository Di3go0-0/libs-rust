use class::class::Matrix;

mod class;
fn main() {
    let mut matrix = Matrix::new(3, 3);
    println!("Matrix:\n{}", matrix);
    let mut matrix1 = Matrix::new(3, 1);

    let x1 = vec![2.0, -2.0, 2.0];
    let x2 = vec![2.0, 1.0, 0.0];
    let x3 = vec![3.0, -2.0, 2.0];

    matrix.set_row(0, x1);
    matrix.set_row(1, x2);
    matrix.set_row(2, x3);

    matrix1.set(0, 0, 1.0);
    matrix1.set(1, 0, 1.0);
    matrix1.set(2, 0, 1.0);

    println!("Matrix:\n{}", matrix);
    println!("Matrix1:\n{}", matrix1);

    let multy = matrix.multiplication(matrix1);
    println!("Multi:\n{}", multy);

    // let det = matrix.determinant();
    // println!("Det: {}", det);
    // let trans = matrix.transpose();
    // println!("trans: \n{}", trans);
    // let adju = trans.adjugte();
    // println!("adju: \n{}", adju);
}
