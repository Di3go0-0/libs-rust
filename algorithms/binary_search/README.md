# Binary Search

A Rust implementation of the binary search algorithm for sorted integer arrays.

## Description

Binary search is an efficient algorithm for finding an item from a sorted list of items. It works by repeatedly dividing in half the portion of the list that could contain the item, until you've narrowed the possible locations to just one.

## Usage

```rust
use binary_search::BinarySearch;

let data = vec![1, 3, 5, 7, 9, 11, 13, 15];
let searcher = BinarySearch::new(data);

match searcher.search(7) {
    Some(index) => println!("Found at index: {}", index),
    None => println!("Not found"),
}
```

## Time Complexity

- **Best Case:** O(1) - when the target element is the middle element
- **Worst Case:** O(log n) - when the target element is not present or at the extremes
- **Average Case:** O(log n)

## Space Complexity

O(1) - constant space complexity