/*
	graph
	This problem requires you to implement a basic graph functio
*/

use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let adj= self.adjacency_table_mutable();
        let (node_1,node_2,weight) = edge;
        if !adj.contains_key(node_1) {
            adj.insert(node_1.to_string(), Vec::new());
        }

        if !adj.contains_key(node_2) {
            adj.insert(node_2.to_string(), Vec::new());
        }
        
        if let Some(vec) = adj.get_mut(node_1) {
            vec.push((String::from(node_2),weight));
        } else {
            panic!("error in add_edge");
        }
        if let Some(vec) = adj.get_mut(node_2) {
            vec.push((String::from(node_1),weight));
        } else {
            panic!("error in add_edge");
        }
    }
}
pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        let adj= self.adjacency_table_mutable();
        let k = String::from(node);
        if adj.contains_key(&k) {
            false
        } else {
            adj.insert(k, Vec::new());
            true
        }
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let adj= self.adjacency_table_mutable();
        if !adj.contains_key(edge.0) {
            adj.insert(edge.0.to_string(), Vec::new());
        }

        if let Some(vec) = adj.get_mut(edge.0) {
            vec.push((String::from(edge.1),edge.2));
        } else {
            panic!("error in add_edge");
        }
    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}