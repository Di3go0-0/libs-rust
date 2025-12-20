pub struct BubbleSort {
    pub data: Vec<i32>,
}

impl BubbleSort {
    pub fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    pub fn sort(&mut self) {
        let n = self.data.len();

        for i in 0..n {
            for j in 0..n - 1 - i {
                if self.data[j] > self.data[j + 1] {
                    self.data.swap(j, j + 1);
                }
            }
        }
    }
}
