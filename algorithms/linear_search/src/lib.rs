pub struct LinealSearch {
    data: Vec<i32>,
}

impl LinealSearch {
    pub fn new(data: Vec<i32>) -> Self {
        Self { data }
    }

    pub fn search(&self, target: i32) -> Option<usize> {
        for (index, value) in self.data.iter().enumerate() {
            if *value == target {
                return Some(index);
            }
        }
        None
    }
}
