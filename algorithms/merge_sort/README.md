# Merge Sort

A Rust implementation of the merge sort algorithm for sorting integer arrays.

## Description

Merge sort is an efficient, general-purpose, and comparison-based sorting algorithm. It works by dividing the unsorted list into n sublists, each containing one element, and repeatedly merging sublists to produce new sorted sublists until there is only one sublist remaining.

## Usage

```rust
use merge_sort::MergeSort;

let mut sorter = MergeSort::new(vec![12, 11, 13, 5, 6, 7]);
sorter.sort();

println!("Sorted data: {:?}", sorter.data);
```

## Time Complexity

- **Best Case:** O(n log n)
- **Worst Case:** O(n log n)
- **Average Case:** O(n log n)

## Space Complexity

O(n) - requires additional space proportional to the input size for merging

## Characteristics

- **Divide and Conquer:** Recursively divides the array into halves
- **Stable Sort:** Maintains the relative order of equal elements
- **Predictable Performance:** Consistent O(n log n) time complexity regardless of input
- **Parallelizable:** Can be easily parallelized due to independent subproblems
- **Memory Efficient:** Better worst-case performance than quicksort but requires more space
- **Suitable for Linked Lists:** Excellent performance with linked lists as no random access is needed