pub struct Activation {
    pub function: fn(&f64) -> f64,
    pub derivative: fn(&f64) -> f64,
}

pub fn SIGMOID(x: &f64) -> f64 {
    let e_x = (-x).exp();
    return (1.0 / (1.0 + e_x));
}

pub fn SIGMOID_DERIVATIVE(output: &f64) -> f64 {
    output * (1.0 - output) // Se asume que la entrada ya es la salida del sigmoide
}
