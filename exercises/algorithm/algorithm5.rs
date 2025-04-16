/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

use std::collections::{VecDeque, HashSet};

// 定义图结构
struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    // 创建具有n个顶点的新图
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // 添加无向边
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    // 广度优先搜索，返回访问顺序
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        let mut visit_order = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        // 检查起始节点是否有效
        if start >= self.adj.len() {
            return visit_order;
        }

        // 初始化
        queue.push_back(start);
        visited.insert(start);

        while let Some(current) = queue.pop_front() {
            visit_order.push(current);
            
            // 访问所有相邻节点
            for &neighbor in &self.adj[current] {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);
        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);
        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);
        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}