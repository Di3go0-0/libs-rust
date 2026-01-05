# Bubble Sort

A Rust implementation of the bubble sort algorithm for sorting integer arrays.

## Description

Bubble sort is a simple sorting algorithm that repeatedly steps through the list, compares adjacent elements and swaps them if they are in the wrong order. The pass through the list is repeated until the list is sorted.

## Usage

```rust
use bubble_sort::BubbleSort;

let mut sorter = BubbleSort::new(vec![64, 34, 25, 12, 22, 11, 90]);
sorter.sort();

println!("Sorted data: {:?}", sorter.data);
```

## Time Complexity

- **Best Case:** O(n) - when the array is already sorted
- **Worst Case:** O(n²) - when the array is sorted in reverse order
- **Average Case:** O(n²)

## Space Complexity

O(1) - constant space complexity (in-place sorting)

## Characteristics

- Simple to understand and implement
- Stable sort (maintains relative order of equal elements)
- Not suitable for large datasets due to quadratic time complexity