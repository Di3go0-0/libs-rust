# Insertion Sort

A Rust implementation of the insertion sort algorithm for sorting integer arrays.

## Description

Insertion sort is a simple sorting algorithm that builds the final sorted array one item at a time. It's much less efficient on large lists than more advanced algorithms such as quicksort, heapsort, or merge sort.

## Usage

```rust
use insertion_sort::InsertionSort;

let mut sorter = InsertionSort::new(vec![12, 11, 13, 5, 6]);
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

- Simple to implement
- Efficient for small datasets
- Adaptive (efficient for data that is already substantially sorted)
- Stable sort (maintains relative order of equal elements)
- Online (can sort a list as it receives it)
- More efficient in practice than most other simple quadratic algorithms such as bubble sort