pub struct BinarySearch {
    data: Vec<i32>,
}

impl BinarySearch {
    pub fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    pub fn search(&self, target: i32) -> Option<usize> {
        let mut left = 0;
        let mut right = self.data.len();

        while left < right {
            let mid = left + (right - left) / 2;

            if self.data[mid] == target {
                return Some(mid);
            }

            if self.data[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        None
    }
}
