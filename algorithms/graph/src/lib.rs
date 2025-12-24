use std::collections::VecDeque;

pub struct Graph {
    adj_list: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new(nodes: usize) -> Self {
        Self {
            adj_list: vec![Vec::new(); nodes],
        }
    }

    pub fn add_edge(&mut self, from: usize, to: usize) {
        self.adj_list[from].push(to);
    }

    pub fn dfs(&self, start: usize) {
        let mut visited = vec![false; self.adj_list.len()];
        self.dfs_recursive(start, &mut visited);
    }

    fn dfs_recursive(&self, node: usize, visited: &mut Vec<bool>) {
        visited[node] = true;
        println!("Visiting node {}", node);

        for &neighbor in &self.adj_list[node] {
            if !visited[neighbor] {
                self.dfs_recursive(neighbor, visited);
            }
        }
    }

    pub fn bfs(&self, start: usize) {
        let mut visited = vec![false; self.adj_list.len()];
        self.bfs_internal(start, &mut visited);
    }

    fn bfs_internal(&self, start: usize, visited: &mut Vec<bool>) {
        let mut queue = VecDeque::new();

        visited[start] = true;
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            println!("Visiting node {}", node);

            for &neighbor in &self.adj_list[node] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }
    }
}
