mod activation;
pub use activation::{
    Activation, ELU, ELU_DERIVATIVE, LEAKY_RELU, LEAKY_RELU_DERIVATIVE, RELU, RELU_DERIVATIVE,
    SIGMOID, SIGMOID_DERIVATIVE, SWISH, SWISH_DERIVATIVE, TANH, TANH_DERIVATIVE,
};

// use activation::Activation;
use matrix::Matrix;

// use create::activations::activations;

// #[derive(Builder)]
pub struct Network {
    layers: Vec<usize>,
    weights: Vec<Matrix>,
    biases: Vec<Matrix>,
    data: Vec<Matrix>,
    activation: Activation,
    learning_rate: f64,
}

impl Network {
    pub fn new(layers: Vec<usize>, activation: Activation, learning_rate: f64) -> Self {
        let mut weights = vec![];
        let mut biases = vec![];

        for i in 0..layers.len() - 1 {
            let fan_in = layers[i]; // Número de neuronas en la capa anterior
            let std_dev = (1.0 / fan_in as f64).sqrt();

            weights.push(Matrix::random(layers[i + 1], layers[i]).apply(|x| x * std_dev));
            biases.push(Matrix::random(layers[i + 1], 1).apply(|x| x * std_dev));
        }

        Network {
            layers,
            weights,
            biases,
            data: vec![],
            activation,
            learning_rate,
        }
    }

    pub fn feed_forward(&mut self, inputs: Matrix) -> Matrix {
        assert!(
            self.layers[0] == inputs.data.len(),
            "Invalid number of inputs"
        );

        let mut current: Matrix = inputs;
        self.data = vec![current.clone()];

        for i in 0..self.weights.len() {
            current = self.weights[i]
                .multiplication(&current) // multiplica la matriz de pesos por la entrada actual
                .add(&self.biases[i]) // suma el bias
                .unwrap() // desenvuelve el resultado si `add` devuelve un `result`
                .apply(self.activation.function); // aplica la función de activación

            self.data.push(current.clone());
        }

        current
    }

    pub fn back_propagate(&mut self, inputs: Matrix, targets: Matrix) {
        let mut errors: Matrix = targets.subtract(&self.data.last().unwrap()).unwrap(); // Usa la salida real de la red

        let mut gradients: Matrix = self.data.last().unwrap().apply(self.activation.derivative);

        for i in (0..self.weights.len()).rev() {
            gradients = gradients
                .elementwise_multiply(&errors)
                .unwrap()
                .apply(|x: &f64| x * self.learning_rate); // Usa learning_rate correctamente

            self.weights[i] = self.weights[i]
                .add(&gradients.multiplication(&self.data[i].transpose()))
                .unwrap();

            self.biases[i] = self.biases[i].add(&gradients).unwrap();

            if i > 0 {
                errors = self.weights[i].transpose().multiplication(&errors);
                gradients = self.data[i].apply(self.activation.derivative);
            }
        }
    }

    pub fn train(&mut self, inputs: &Vec<Vec<f64>>, targets: &Vec<Vec<f64>>, epochs: u32) {
        for i in 1..=epochs {
            if epochs < 100 || i % (epochs / 100) == 0 {
                // println!("Epoch {} of {}", i, epochs);
            }
            for j in 0..inputs.len() {
                let output = self.feed_forward(Matrix::from(inputs[j].clone()));
                self.back_propagate(output, Matrix::from(targets[j].clone()));
            }
        }
    }
}
