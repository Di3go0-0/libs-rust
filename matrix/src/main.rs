use class::class::Matrix;

mod class;
fn main() {
    let mut matrix = Matrix::new(2, 3);
    println!("Matrix:\n{}", matrix);

    let x1 = vec![1.0, 2.0, 3.0];
    let x2 = vec![4.0, 5.0, 6.0];

    matrix.set_row(0, x1);
    matrix.set_row(1, x2);

    let pseudo = matrix.inverse();
    println!("inverse: \n{}", pseudo);
}
