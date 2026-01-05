# Neural Network

A Rust implementation of a feedforward neural network with backpropagation training algorithm and multiple activation functions.

## Description

This library provides a complete neural network implementation supporting customizable architecture, various activation functions, and gradient descent-based training. It's designed for educational purposes and small-scale machine learning tasks.

## Features

- **Custom Architecture:** Define any number of layers and neurons
- **Multiple Activation Functions:** Sigmoid, ReLU, Leaky ReLU, Tanh, ELU, Swish
- **Backpropagation:** Gradient-based learning algorithm
- **He Initialization:** Proper weight initialization for deep networks
- **Matrix Operations:** Efficient computation using custom matrix library
- **Flexible Training:** Configurable learning rate and epochs

## Usage

```rust
use network::{Network, Activation};

// Create a neural network with:
// - 2 input neurons
// - 3 hidden neurons  
// - 1 output neuron
// - ReLU activation
// - 0.1 learning rate
let mut network = Network::new(
    vec![2, 3, 1],
    Activation { 
        function: network::RELU,
        derivative: network::RELU_DERIVATIVE 
    },
    0.1
);

// Training data (XOR problem example)
let inputs = vec![
    vec![0.0, 0.0],
    vec![0.0, 1.0], 
    vec![1.0, 0.0],
    vec![1.0, 1.0]
];

let targets = vec![
    vec![0.0],
    vec![1.0],
    vec![1.0], 
    vec![0.0]
];

// Train the network
network.train(&inputs, &targets, 1000);

// Make predictions
let input = matrix::Matrix::from(vec![1.0, 0.0]);
let output = network.feed_forward(input);
println!("Prediction: {:?}", output);
```

## Activation Functions

### Sigmoid
- **Function:** `σ(x) = 1 / (1 + e^(-x))`
- **Range:** (0, 1)
- **Use Case:** Binary classification, output layers

### ReLU (Rectified Linear Unit)
- **Function:** `f(x) = max(0, x)`
- **Range:** [0, ∞)
- **Use Case:** Hidden layers, deep networks

### Leaky ReLU
- **Function:** `f(x) = max(0.01x, x)`
- **Range:** (-∞, ∞)
- **Use Case:** Addresses dying ReLU problem

### Tanh (Hyperbolic Tangent)
- **Function:** `tanh(x)`
- **Range:** (-1, 1)
- **Use Case:** Hidden layers, zero-centered output

### ELU (Exponential Linear Unit)
- **Function:** `f(x) = x if x > 0, else α(e^x - 1)`
- **Range:** (-α, ∞)
- **Use Case:** Alternative to ReLU, faster convergence

### Swish
- **Function:** `f(x) = x / (1 + e^(-x))`
- **Range:** (-∞, ∞)
- **Use Case:** Self-gated activation, smooth alternative

## Architecture

### Network Structure
```
Input Layer    Hidden Layer(s)    Output Layer
    │              │ │ │              │
    ├─→ Weights → ├─→ Weights → ──┤
    │              │ │ │              │
    └─→ Biases  → └─→ Biases  → ──┘
```

### Key Components
- **Layers:** Vector defining neurons per layer
- **Weights:** Matrices between consecutive layers
- **Biases:** Bias vectors for each layer
- **Activation Function:** Applied to each layer output

## Training Process

1. **Forward Propagation:**
   - Input × Weights + Bias → Activation
   - Store intermediate results for backpropagation

2. **Backpropagation:**
   - Calculate output error: `error = target - output`
   - Compute gradients using activation derivatives
   - Update weights: `weights += learning_rate × gradients`

3. **Iteration:**
   - Repeat for all training samples
   - Continue for specified epochs

## Implementation Details

### Weight Initialization
- Uses He initialization: `weights ~ N(0, 1/fan_in)`
- Prevents vanishing/exploding gradients
- Better for ReLU-based networks

### Memory Efficiency
- Stores intermediate activations during forward pass
- Reuses computations during backpropagation
- Matrix operations optimized for performance

## Performance Characteristics

### Time Complexity
- **Forward Pass:** O(L × N²) where L is layers, N is neurons
- **Backward Pass:** O(L × N²) similar to forward pass
- **Training:** O(epochs × samples × L × N²)

### Space Complexity
- **Weights Storage:** O(Σ(n_i × n_{i+1})) for all layer pairs
- **Activation Storage:** O(Σ(n_i)) for all layers

## Limitations

- Single-threaded implementation
- No regularization (L1/L2 dropout)
- Fixed learning rate (no adaptive methods)
- Only supports fully connected layers
- No batch processing (online learning only)

## Applications

- Pattern recognition
- Function approximation
- Binary/multi-class classification
- Time series prediction (with modifications)
- Educational purposes and research