pub struct Activation {
    pub function: fn(&f64) -> f64,
    pub derivative: fn(&f64) -> f64,
}

pub fn SIGMOID(x: &f64) -> f64 {
    let e_x = (-x).exp();
    return (1.0 / (1.0 + e_x));
}

pub fn SIGMOID_DERIVATIVE(x: &f64) -> f64 {
    let sigmoid_x = SIGMOID(x);
    sigmoid_x * (1.0 - sigmoid_x)
}
