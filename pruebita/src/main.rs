use merge_sort::MergeSort;

fn main() {
    let vec = vec![5, 2, 4, 6, 1, 3];
    let mut sort = MergeSort::new(vec);

    sort.sort();

    println!("{:?}", sort.data);
}

// use insertion_sort::InsertionSort;
//
// fn main() {
//     let vec = vec![5, 2, 4, 6, 1, 3];
//     let mut sort = InsertionSort::new(vec);
//
//     sort.sort();
//
//     println!("{:?}", sort.data);
// }

// use vector::MyVec;
// fn main() {
//     let mut v = MyVec::new();
//
//     v.push(10);
//     v.push(20);
//     v.push(30);
//
//     println!("{:?}", v.get(1));
//
//     v.pop();
//     v.pop();
//
//     println!("Length: {}", v.len());
//     println!("{:?}", v.get(0));
// }

// use linear_search::LinealSearch;
// fn main() {
//     let searcher = LinealSearch::new(vec![10, 20, 30, 40]);
//
//     match searcher.search(30) {
//         Some(index) => println!("Element find in the position {}", index),
//         None => println!("Element not find"),
//     }
// }

// use binary_search::BinarySearch;
// fn main() {
//     let bs = BinarySearch::new(vec![1, 3, 5, 7, 9, 11]);
//
//     match bs.search(7) {
//         Some(index) => println!("Element find in the position {}", index),
//         None => println!("Element not find"),
//     }
// }

// use bubble_sort::BubbleSort;
// fn main() {
//     let vec = vec![7, 2, 3, 5, 4, 10];
//     let mut bubble = BubbleSort::new(vec);
//
//     bubble.sort();
//
//     println!("{:?}", bubble.data);
// }
