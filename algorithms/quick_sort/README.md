# Quick Sort

A Rust implementation of the quick sort algorithm for sorting integer arrays.

## Description

Quick sort is an efficient, in-place sorting algorithm that uses a divide-and-conquer strategy. It works by selecting a 'pivot' element and partitioning the array around the pivot, such that elements less than the pivot come before it and elements greater come after it.

## Usage

```rust
use quick_sort::QuickSort;

let mut sorter = QuickSort::new(vec![10, 7, 8, 9, 1, 5]);
sorter.sort();

println!("Sorted data: {:?}", sorter.data);
```

## Time Complexity

- **Best Case:** O(n log n) - when the pivot always divides the array into roughly equal halves
- **Worst Case:** O(n²) - when the pivot is always the smallest or largest element
- **Average Case:** O(n log n)

## Space Complexity

O(log n) - due to the recursive call stack depth (in-place partitioning)

## Characteristics

- **In-place Sorting:** Requires minimal additional memory
- **Divide and Conquer:** Recursively partitions the array
- **Efficient in Practice:** Often faster than merge sort due to lower constant factors
- **Unstable Sort:** Does not maintain the relative order of equal elements
- **Pivot Selection:** Performance depends heavily on pivot choice
- **Worst-case Rare:** With good pivot selection (like median-of-three), worst case is extremely rare

## Pivot Strategy

This implementation uses the last element as the pivot, which is simple but can lead to worst-case behavior on already sorted arrays. For production use, consider implementing:
- Random pivot selection
- Median-of-three pivot selection
- Dutch national flag partitioning for handling duplicates