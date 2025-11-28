use std::collections::HashSet;
use crate::core::data_structures::graphs::GraphBase;

pub fn bfs<G>(
    graph: &G,
    start: G::NodeId,
    goal: G::NodeId,
) -> Option<Vec<G::NodeId>>
where
    G: GraphBase,
    G::NodeId: Clone + Eq + std::hash::Hash,
{
    use std::collections::{VecDeque, HashSet};

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent_map: std::collections::HashMap<G::NodeId, G::NodeId> = std::collections::HashMap::new();

    queue.push_back(start.clone());
    visited.insert(start.clone());

    while let Some(current) = queue.pop_front() {
        if current == goal {
            // Reconstrucción del camino
            let mut path = Vec::new();
            let mut node = goal.clone();

            while node != start {
                path.push(node.clone());
                node = parent_map.get(&node).unwrap().clone();
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        for neighbor in graph.neighbors(current.clone()) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor.clone());
                parent_map.insert(neighbor.clone(), current.clone());
                queue.push_back(neighbor);
            }
        }
    }

    None
}

pub fn bfs_traversal<G>(
    graph: &G,
    start: G::NodeId,
) -> (std::collections::HashSet<G::NodeId>, Vec<G::NodeId>)
where
    G: GraphBase,
    G::NodeId: Clone + Eq + std::hash::Hash,
{
    use std::collections::{VecDeque, HashSet};

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut traversal_order = Vec::new();

    queue.push_back(start.clone());
    visited.insert(start.clone());

    while let Some(current) = queue.pop_front() {
        traversal_order.push(current.clone());

        for neighbor in graph.neighbors(current.clone()) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor.clone());
                queue.push_back(neighbor);
            }
        }
    }

    ( visited, traversal_order)
}

pub fn dfs<G>(
    graph: &G,
    start: G::NodeId,
    goal: G::NodeId,
) -> Option<Vec<G::NodeId>>
where
    G: GraphBase,
    G::NodeId: Clone + Eq + std::hash::Hash,
{
    use std::collections::{VecDeque, HashSet};

    let mut stack = VecDeque::new();
    let mut visited = HashSet::new();
    let mut parent_map: std::collections::HashMap<G::NodeId, G::NodeId> = std::collections::HashMap::new();

    stack.push_back(start.clone());
    visited.insert(start.clone());

    while let Some(current) = stack.pop_back() {
        if current == goal {
            // Reconstrucción del camino
            let mut path = Vec::new();
            let mut node = goal.clone();

            while node != start {
                path.push(node.clone());
                node = parent_map.get(&node).unwrap().clone();
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        for neighbor in graph.neighbors(current.clone()) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor.clone());
                parent_map.insert(neighbor.clone(), current.clone());
                stack.push_back(neighbor);
            }
        }
    }

    None
}

pub fn dfs_cycle<G>(graph: &G, start: G::NodeId, current: G::NodeId, visited: &mut HashSet<G::NodeId>) -> bool
where
    G: GraphBase,
    G::NodeId: Clone + Eq + std::hash::Hash,
{
    if visited.contains(&current) { return false; }

    visited.insert(current.clone());

    for neighbor in graph.neighbors(current.clone()) {
        if neighbor == start { return true; } // ciclo detectado
        if dfs_cycle(graph, start.clone(), neighbor.clone(), visited) {
            return true;
        }
    }

    visited.remove(&current);
    false
}

pub fn dfs_traversal<G>(
    graph: &G,
    start: G::NodeId,
) -> (std::collections::HashSet<G::NodeId>, Vec<G::NodeId>)
where
    G: GraphBase,
    G::NodeId: Clone + Eq + std::hash::Hash,
{
    use std::collections::{VecDeque, HashSet};

    let mut stack = VecDeque::new();
    let mut visited = HashSet::new();
    let mut traversal_order = Vec::new();

    stack.push_back(start.clone());
    visited.insert(start.clone());

    while let Some(current) = stack.pop_back() {
        traversal_order.push(current.clone());

        for neighbor in graph.neighbors(current.clone()) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor.clone());
                stack.push_back(neighbor);
            }
        }
    }

    ( visited, traversal_order)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::data_structures::graphs::BaseGraph;

    fn setup_graph() -> (BaseGraph<&'static str, i32>, Vec<usize>) {
        let mut graph: BaseGraph<&str, i32> = BaseGraph::new();
        let mut nodes_id = Vec::new();
        let n0 = graph.add_node("A");
        let n1 = graph.add_node("B");
        let n2 = graph.add_node("C");
        let n3 = graph.add_node("D");
        let n4 = graph.add_node("E");

        graph.add_edge(n0, n1, 1);
        graph.add_edge(n1, n0, 1);
        graph.add_edge(n0, n2, 1);
        graph.add_edge(n2, n0, 1);
        graph.add_edge(n1, n3, 1);
        graph.add_edge(n3, n1, 1);
        graph.add_edge(n2, n4, 1);
        graph.add_edge(n4, n2, 1);
        graph.add_edge(n3, n4, 1);
        graph.add_edge(n4, n3, 1);

        nodes_id.push(n0);
        nodes_id.push(n1);
        nodes_id.push(n2);
        nodes_id.push(n3);
        nodes_id.push(n4);

        (graph, nodes_id)
    }

    #[test]
    fn test_bfs() {
        let (graph, nodes_id) = setup_graph();

        let path = bfs(&graph, nodes_id[0], nodes_id[4]).unwrap();
        let expected_path = vec![nodes_id[0], nodes_id[2], nodes_id[4]];

        assert_eq!(path, expected_path);
    }

    #[test]
    fn test_dfs() {
        let (graph, nodes_id) = setup_graph();

        let path = dfs(&graph, nodes_id[0], nodes_id[4]).unwrap();
        let expected_path = vec![nodes_id[0], nodes_id[2], nodes_id[4]];

        assert_eq!(path, expected_path);
    }

    #[test]
    fn test_dfs_cycle() {
        let (graph, nodes_id) = setup_graph();

        let mut visited = HashSet::new();
        let has_cycle = dfs_cycle(&graph, nodes_id[0], nodes_id[0], &mut visited);

        assert_eq!(has_cycle, true);
    }
}

