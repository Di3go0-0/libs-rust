# Vector

A custom implementation of a dynamic array (vector) data structure in Rust, similar to `std::vec::Vec`.

## Description

This library implements a custom vector data structure from scratch using manual memory management. It demonstrates understanding of Rust's memory allocation, pointer operations, and the RAII pattern. This is an educational implementation showing how dynamic arrays work internally.

## Features

- **Dynamic Growth:** Automatically resizes when capacity is exceeded
- **Memory Efficient:** Grows capacity exponentially (doubles when full)
- **Manual Memory Management:** Uses unsafe Rust for direct memory operations
- **Basic Operations:** Push, pop, get, length queries
- **Proper Cleanup:** Manages memory allocation and deallocation

## Usage

```rust
use vector::MyVec;

let mut vec = MyVec::new();

// Push elements
vec.push(42);
vec.push(10);
vec.push(7);

// Get elements
match vec.get(1) {
    Some(value) => println!("Value at index 1: {}", value),
    None => println!("Index out of bounds"),
}

// Pop elements
match vec.pop() {
    Some(value) => println!("Popped value: {}", value),
    None => println!("Vector is empty"),
}

// Check length
println!("Vector length: {}", vec.len());
```

## Key Methods

- `new()` - Create empty vector
- `push(value)` - Add element to the end
- `get(index)` - Get element by index (returns Option)
- `pop()` - Remove and return last element (returns Option)
- `len()` - Get current length
- `drop()` - Clean up memory (called automatically)

## Implementation Details

### Memory Management
- Uses `std::alloc::Layout` for memory allocation
- Manages raw pointers (`*mut T`) for element storage
- Implements capacity-based growth strategy
- Properly handles memory deallocation on drop

### Growth Strategy
- Initial capacity: 0
- Growth factor: 2× (doubles when full)
- Minimizes reallocations for better performance
- Copies existing elements to new memory location

### Safety Considerations
- Uses `unsafe` blocks for direct memory operations
- Implements bounds checking for safe access
- Proper cleanup prevents memory leaks
- Handles edge cases (empty vector, single element)

## Time Complexity

- **Push (average case):** O(1) - amortized constant time
- **Push (worst case):** O(n) - when reallocation needed
- **Pop:** O(1) - constant time
- **Get:** O(1) - constant time with bounds checking
- **Length:** O(1) - constant time

## Space Complexity

- **Storage:** O(n) - proportional to number of elements
- **Overhead:** Minimal - stores only pointer, length, and capacity

## Educational Value

This implementation demonstrates:
- Manual memory management in Rust
- Understanding of dynamic array internals
- Safe usage of unsafe Rust
- RAII (Resource Acquisition Is Initialization) pattern
- Capacity vs length concepts
- Memory growth strategies

## Note

This is primarily an educational implementation. For production use, prefer Rust's standard `std::vec::Vec` which is more optimized and battle-tested.