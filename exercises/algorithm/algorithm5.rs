/*
	bfs
	This problem requires you to implement a basic BFS algorithm
*/

// a piece of cake
use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>, 
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n], // nxn 二维数组
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        // 添加双向边
        self.adj[src].push(dest); 
        self.adj[dest].push(src); 
    }

    // Perform a breadth-first search on the graph, return the order of visited nodes
    fn bfs_with_return(&self, start: usize) ->  Vec<usize> {
        
		//TODO

        let mut visit_order: Vec<usize> = vec![];
        visit_order.push(start);
        let mut index = 0;
        while index < visit_order.len() {
            let current = visit_order[index];
            for j in &self.adj[current] {
                if !exist(&visit_order, j) {
                    visit_order.push(*j);
                }
            }
            index += 1;
        }
        visit_order
        /*
            let mut visit_order: Vec<usize> = Vec<usize>::new();
            visit_order.push(start);
            for i in visit_order{
                for j in &self.adj[i]{
                    if exist(&visit_order, j){
                        continue;
                    }else {
                        visit_order.push(*j);
                    }
                }
            }
            visit_order
         */
    }
}

fn exist(vector: &Vec<usize>, value: &usize) -> bool{
    let mut result = false;
    for v in vector{
        if v == value{
            result = true;
            break
        }
    }
    result
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

