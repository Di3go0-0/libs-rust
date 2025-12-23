use binary_search::BinarySearch;
use bubble_sort::BubbleSort;
use insertion_sort::InsertionSort;
use linear_search::LinealSearch;
use merge_sort::MergeSort;
use quick_sort::QuickSort;

fn main() {
    let base = vec![5, 2, 4, 6, 1, 3];
    println!("Base: {:?}", base);

    // insertionSort(&base);
    // bubbleSort(&base);
    // mergeSort(&base);
    quick_sort(&base)
}

fn linearSearch(vec: &[i32]) {
    let searcher = LinealSearch::new(vec.to_vec());

    match searcher.search(30) {
        Some(index) => println!("Element find in the position {}", index),
        None => println!("Element not find"),
    }
}

fn binarySearch(vec: &[i32]) {
    let bs = BinarySearch::new(vec.to_vec());

    match bs.search(7) {
        Some(index) => println!("Element find in the position {}", index),
        None => println!("Element not find"),
    }
}

fn insertionSort(vec: &[i32]) {
    let mut sort = InsertionSort::new(vec.to_vec());

    sort.sort();

    println!("{:?}", sort.data);
}

fn bubbleSort(vec: &[i32]) {
    let mut bubble = BubbleSort::new(vec.to_vec());

    bubble.sort();

    println!("{:?}", bubble.data);
}

fn mergeSort(vec: &[i32]) {
    let mut sort = MergeSort::new(vec.to_vec());

    sort.sort();

    println!("{:?}", sort.data);
}

fn quick_sort(vec: &[i32]) {
    let mut sort = QuickSort::new(vec.to_vec());

    sort.sort();

    println!("{:?}", sort.data);
}
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
