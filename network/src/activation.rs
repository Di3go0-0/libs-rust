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

pub fn RELU(x: &f64) -> f64 {
    x.max(0.0)
}

pub fn RELU_DERIVATIVE(x: &f64) -> f64 {
    if *x > 0.0 { 1.0 } else { 0.0 }
}

pub fn LEAKY_RELU(x: &f64) -> f64 {
    if *x > 0.0 { *x } else { 0.01 * x }
}

pub fn LEAKY_RELU_DERIVATIVE(x: &f64) -> f64 {
    if *x > 0.0 { 1.0 } else { 0.01 }
}

pub fn TANH(x: &f64) -> f64 {
    x.tanh()
}

pub fn TANH_DERIVATIVE(output: &f64) -> f64 {
    1.0 - output.powi(2) // Se asume que la entrada ya es la salida de tanh
}

const ALPHA: f64 = 0.01;

pub fn ELU(x: &f64) -> f64 {
    if *x > 0.0 {
        *x
    } else {
        ALPHA * ((*x).exp() - 1.0)
    }
}

pub fn ELU_DERIVATIVE(x: &f64) -> f64 {
    if *x > 0.0 { 1.0 } else { ALPHA * x.exp() }
}

pub fn SWISH(x: &f64) -> f64 {
    *x / (1.0 + (-x).exp())
}

pub fn SWISH_DERIVATIVE(x: &f64) -> f64 {
    let sig = SIGMOID(x);
    sig + x * sig * (1.0 - sig)
}
