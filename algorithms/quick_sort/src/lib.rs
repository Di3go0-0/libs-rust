pub struct QuickSort {
    pub data: Vec<i32>,
}

impl QuickSort {
    pub fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    pub fn sort(&mut self) {
        let len = self.data.len();
        if len <= 1 {
            return;
        }

        self.quick_sort(0, (len - 1) as isize);
    }

    fn quick_sort(&mut self, low: isize, high: isize) {
        if low < high {
            let pivot_index = self.partition(low, high);

            self.quick_sort(low, pivot_index - 1);
            self.quick_sort(pivot_index + 1, high);
        }
    }

    fn partition(&mut self, low: isize, high: isize) -> isize {
        let pivot = self.data[high as usize];
        let mut i = low - 1;

        for j in low..high {
            if self.data[j as usize] <= pivot {
                i += 1;
                self.data.swap(i as usize, j as usize);
            }
        }

        self.data.swap((i + 1) as usize, high as usize);
        i + 1
    }
}
