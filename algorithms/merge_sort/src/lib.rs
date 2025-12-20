pub struct MergeSort {
    pub data: Vec<i32>,
}

impl MergeSort {
    pub fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    pub fn sort(&mut self) {
        let sorted = Self::merge_sort(self.data.clone());
        self.data = sorted;
    }

    fn merge_sort(data: Vec<i32>) -> Vec<i32> {
        if data.len() <= 1 {
            return data;
        }

        let mid = data.len() / 2;

        let left = Self::merge_sort(data[..mid].to_vec());
        let right = Self::merge_sort(data[mid..].to_vec());

        Self::merge(left, right)
    }

    fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::with_capacity(left.len() + right.len());

        let mut i = 0;
        let mut j = 0;

        while i < left.len() && j < right.len() {
            if left[i] <= right[j] {
                result.push(left[i]);
                i += 1;
            } else {
                result.push(right[j]);
                j += 1;
            }
        }

        result.extend_from_slice(&left[i..]);
        result.extend_from_slice(&right[j..]);

        result
    }
}
