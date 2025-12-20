pub struct InsertionSort {
    pub data: Vec<i32>,
}

impl InsertionSort {
    pub fn new(data: Vec<i32>) -> Self {
        InsertionSort { data }
    }

    pub fn sort(&mut self) {
        let n = self.data.len();

        for i in 1..n {
            let key = self.data[i];
            let mut j = i;

            while j > 0 && self.data[j - 1] > key {
                self.data[j] = self.data[j - 1];
                j -= 1;
            }

            self.data[j] = key;
        }
    }
}
