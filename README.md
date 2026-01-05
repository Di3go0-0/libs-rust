# Rust Libraries Collection

A comprehensive collection of Rust implementations covering fundamental algorithms, data structures, and machine learning components. This repository serves as both a learning resource and a library of reusable components for various programming tasks.

## 📁 Repository Structure

```
libs-rust/
├── algorithms/           # Search and sorting algorithms
│   ├── binary_search/   # Binary search implementation
│   ├── bubble_sort/     # Bubble sort implementation
│   ├── graph/           # Graph data structure with DFS/BFS
│   ├── insertion_sort/  # Insertion sort implementation
│   ├── linear_search/  # Linear search implementation
│   ├── merge_sort/      # Merge sort implementation
│   └── quick_sort/      # Quick sort implementation
├── data-structures/     # Custom data structure implementations
│   ├── matrix/          # Matrix operations and linear algebra
│   └── vector/          # Custom vector implementation
├── network/             # Neural network implementation
└── pruebita/           # Test/example project
```

## 🚀 Getting Started

Each library is a standalone Rust crate with its own `Cargo.toml`. You can use them individually or as dependencies in your projects.

### Using a Library

Add the library as a dependency in your `Cargo.toml`:

```toml
[dependencies]
# Example: Using the binary search library
binary_search = { path = "libs-rust/algorithms/binary_search" }
```

Then use it in your code:

```rust
use binary_search::BinarySearch;

fn main() {
    let data = vec![1, 3, 5, 7, 9];
    let searcher = BinarySearch::new(data);
    
    match searcher.search(5) {
        Some(index) => println!("Found at index: {}", index),
        None => println!("Not found"),
    }
}
```

## 📚 Library Categories

### 🔍 Algorithms

**Search Algorithms:**
- **Binary Search** - O(log n) search in sorted arrays
- **Linear Search** - O(n) sequential search

**Sorting Algorithms:**
- **Bubble Sort** - Simple O(n²) comparison sort
- **Insertion Sort** - Efficient for small/nearly-sorted datasets
- **Merge Sort** - Stable O(n log n) divide-and-conquer sort
- **Quick Sort** - Efficient in-place O(n log n) sort

**Graph Algorithms:**
- **Graph Data Structure** - Adjacency list representation
- **DFS (Depth-First Search)** - Recursive graph traversal
- **BFS (Breadth-First Search)** - Level-by-level graph traversal

### 🏗️ Data Structures

**Matrix Operations:**
- Basic matrix operations (addition, subtraction, multiplication)
- Advanced linear algebra (determinant, inverse, transpose)
- Random matrix generation with He initialization
- Element-wise operations and transformations

**Custom Vector:**
- Manual memory management implementation
- Dynamic growth with exponential capacity scaling
- Educational implementation showing vector internals

### 🧠 Machine Learning

**Neural Network:**
- Feedforward neural network with backpropagation
- Multiple activation functions (Sigmoid, ReLU, Tanh, ELU, Swish)
- Customizable architecture and learning rate
- Gradient descent-based training

## 🎯 Learning Objectives

This collection is designed to help you understand:

1. **Algorithm Complexity:** Different time/space complexity trade-offs
2. **Data Structure Design:** Custom implementations and memory management
3. **Rust Programming:** Memory safety, ownership, and unsafe Rust concepts
4. **Machine Learning Basics:** Neural network fundamentals and training
5. **Code Organization:** Modular design and crate structure

## 🛠️ Development

### Building All Libraries

```bash
# Build all libraries
find . -name "Cargo.toml" -exec cargo build --manifest-path {} \;

# Run tests for all libraries
find . -name "Cargo.toml" -exec cargo test --manifest-path {} \;
```

### Code Quality

Each library follows Rust best practices:
- Comprehensive documentation with examples
- Proper error handling using `Result` types
- Memory-efficient implementations
- Clear separation of concerns

## 📖 Educational Value

**For Beginners:**
- Clear, well-documented implementations
- Step-by-step algorithm explanations
- Complexity analysis and trade-offs

**For Intermediate Developers:**
- Advanced Rust concepts (unsafe code, memory management)
- Performance optimization techniques
- Custom trait implementations

**For Advanced Users:**
- Foundation for more complex implementations
- Benchmarking and optimization opportunities
- Extensions and customization possibilities

## 🤝 Contributing

Feel free to:
- Submit bug reports and issues
- Propose new algorithms or data structures
- Improve documentation
- Add benchmarks and performance tests

## 📄 License

This project is open source and available under the MIT License.

## 🔗 Related Resources

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Algorithms, 4th Edition](https://algs4.cs.princeton.edu/home/) - Sedgewick & Wayne
- [Neural Networks and Deep Learning](http://neuralnetworksanddeeplearning.com/) - Michael Nielsen
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

---

*This repository is continuously updated with new implementations and improvements. Check back regularly for updates! 🎉*