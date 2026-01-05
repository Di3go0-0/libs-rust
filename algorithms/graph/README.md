# Graph

A Rust implementation of a graph data structure with Depth-First Search (DFS) and Breadth-First Search (BFS) traversal algorithms.

## Description

This library provides a simple directed graph implementation using an adjacency list representation. It supports basic graph operations and traversal algorithms.

## Usage

```rust
use graph::Graph;

// Create a graph with 5 nodes
let mut graph = Graph::new(5);

// Add edges
graph.add_edge(0, 1);
graph.add_edge(0, 2);
graph.add_edge(1, 3);
graph.add_edge(2, 4);

// Perform DFS traversal
println!("DFS traversal:");
graph.dfs(0);

// Perform BFS traversal
println!("BFS traversal:");
graph.bfs(0);
```

## Features

- **Adjacency List Representation:** Efficient memory usage for sparse graphs
- **DFS Traversal:** Explores as far as possible along each branch before backtracking
- **BFS Traversal:** Explores all neighbors at the current depth before moving to the next depth

## Time Complexity

- **Adding Edge:** O(1)
- **DFS Traversal:** O(V + E) where V is vertices and E is edges
- **BFS Traversal:** O(V + E) where V is vertices and E is edges

## Space Complexity

- **Graph Storage:** O(V + E)
- **DFS Traversal:** O(V) for the visited array and call stack
- **BFS Traversal:** O(V) for the visited array and queue