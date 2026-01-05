# Linear Search

A Rust implementation of the linear search algorithm for finding elements in integer arrays.

## Description

Linear search (also known as sequential search) is a method for finding an element within a list. It sequentially checks each element of the list until a match is found or the whole list has been searched.

## Usage

```rust
use linear_search::LinealSearch;

let searcher = LinealSearch::new(vec![4, 2, 7, 1, 3, 9, 6]);

match searcher.search(7) {
    Some(index) => println!("Found at index: {}", index),
    None => println!("Not found"),
}
```

## Time Complexity

- **Best Case:** O(1) - when the target element is the first element
- **Worst Case:** O(n) - when the target element is the last element or not present
- **Average Case:** O(n)

## Space Complexity

O(1) - constant space complexity

## Characteristics

- Simple to implement
- Works on unsorted data
- No preprocessing required
- Suitable for small datasets
- Inefficient for large datasets compared to binary search (when data is sorted)