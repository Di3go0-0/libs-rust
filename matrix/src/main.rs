use class::matrix::Matrix;

mod class;
fn main() {
    let mut matrix = Matrix::new(2, 3);
    println!("Matrix:\n{}", matrix);

    let x1 = vec![1.0, 2.0, 3.0];
    let x2 = vec![4.0, 5.0, 6.0];

    let _ = matrix.set_row(0, x1);
    let _ = matrix.set_row(1, x2);

    let mut pseudo = matrix.transpose();
    println!("inverse: \n{}", pseudo);

    println!("value before: {}", pseudo[0][1]);
    pseudo[0][1] = 0.0;

    println!("value after: {}", pseudo[0][1]);

    println!("inverse: \n{}", pseudo);
}
