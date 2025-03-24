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
    layers: Vec<usize>,     // Neurons in each leayer
    weights: Vec<Matrix>,   // Wight matrix between consecutive layers
    biases: Vec<Matrix>,    // Biases vector on each leayer
    data: Vec<Matrix>,      // Stores the outputs of each leyer during forward propagation.
    activation: Activation, // Activation function to use
    learning_rate: f64,     //Learning rate for updating wights in back propagation.
}

impl Network {
    /// Creates a new neural network with the specific layers, activation function and
    /// learning_rate.
    ///
    /// # Arguments
    /// * `layers` - Vector with the number of neurons in each leayer. for example: [2,3,1] Creates
    ///             a network with 2 neurons in the input layer, 3 in the hiden layer and 1 en in output
    /// * `activation` - Activation function to use in all layers (ex: SIGMOID, RELU, etc)
    /// * `learning_rate` - learing rate in training (typically between 0.01 and 0.1)
    ///
    /// # Inicialización
    /// The Wights and biases are initialized using HE initialization:
    /// - The stantared deviation is calculate as 1/sqrt(fan_in) where fan_in is the number of
    ///   incomin connections
    /// - The weights and Biases are initialized randomly and scaled by this deviations.
    ///   This helps to avoid the saturation of neurons and allows fo better training
    pub fn new(layers: Vec<usize>, activation: Activation, learning_rate: f64) -> Self {
        let mut weights = vec![];
        let mut biases = vec![];

        for i in 0..layers.len() - 1 {
            let fan_in = layers[i];
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
                .multiplication(&current)
                .add(&self.biases[i])
                .unwrap()
                .apply(self.activation.function);

            self.data.push(current.clone());
        }

        current
    }

    pub fn back_propagate(&mut self, inputs: Matrix, targets: Matrix) {
        let mut errors: Matrix = targets.subtract(&inputs).unwrap(); // Usa la salida real de la red

        let mut gradients: Matrix = inputs.clone().apply(self.activation.derivative);

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
