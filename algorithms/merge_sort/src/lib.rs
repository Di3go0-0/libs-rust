
pub struct MergeSort {
    pub data: Vec<i32>,
}

impl MergeSort {
    pub fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    pub fn sort(&mut self) {
        if self.data.len() <= 1 {
            return;
        }

        let mid = self.data.len() / 2;

        let mut left = self.data[..mid].to_vec();
        let mut right = self.data[mid..].to_vec();

        Self::sort(&mut left);
        Self::sort(&mut right);

        Self::merge(self.data, &left, &right);
    }

    pub fn merge(result: &mut self.data, left: &Vec<i32>, right: &Vec<i32>) {
        let mut i = 0,
        let mut j = 0,
        let mut k = 0,

        while i < left.len() && j < right.len() {
            if left[i] <= right[j]{
                result[k] = left[i];
                i += 1;
            } else {
                result[k] = right[j];
                j += 1;
            }
            k += 1;
        }

        while i < left.len() {
            result[k] = left[i];
            i += 1;
            k += 1;
        }
        while j < right.len() {
            result[k] = right[j];
            j += 1;
            k += 1;
        }
    }
}

