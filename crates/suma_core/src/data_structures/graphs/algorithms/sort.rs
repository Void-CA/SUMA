use crate::data_structures::graphs::{Directed, GraphBase};

pub fn topological_sort<G:Directed>(graph : &G) -> Result<Vec<G::NodeId>, &'static str>
{
    use std::collections::HashSet;
    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    let mut on_stack = HashSet::new(); // para detección de ciclos

    fn dfs<G: Directed>(
        node: G::NodeId,
        graph: &G,
        visited: &mut HashSet<G::NodeId>,
        stack: &mut Vec<G::NodeId>,
        on_stack: &mut HashSet<G::NodeId>,
    ) -> Result<(), &'static str> {
        if on_stack.contains(&node) {
            return Err("Cycle detected");
        }
        if !visited.contains(&node) {
            visited.insert(node);
            on_stack.insert(node);
            for neighbor in graph.neighbors(node) {
                dfs(neighbor, graph, visited, stack, on_stack)?;
            }
            stack.push(node);
            on_stack.remove(&node);
        }
        Ok(())
    }

    for node in graph.nodes() {
        dfs(node, graph, &mut visited, &mut stack, &mut on_stack)?;
    }

    stack.reverse();
    Ok(stack)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data_structures::graphs::DirectedGraph;

    #[test]
    fn test_topological_sort_acyclic() {
        let mut graph: DirectedGraph<&str> = DirectedGraph::new();
        let a = graph.add_node("A");
        let b = graph.add_node("B");
        let c = graph.add_node("C");
        let d = graph.add_node("D");

        graph.add_directed_edge(a, b);
        graph.add_directed_edge(b, c);
        graph.add_directed_edge(c, d);

        let sorted = topological_sort(&graph).unwrap();
        assert_eq!(sorted, vec![a, b, c, d]);
    }

    #[test]
    fn test_topological_sort_cyclic() {
        let mut graph: DirectedGraph<&str> = DirectedGraph::new();
        let a = graph.add_node("A");
        let b = graph.add_node("B");
        let c = graph.add_node("C");

        graph.add_directed_edge(a, b);
        graph.add_directed_edge(b, c);
        graph.add_directed_edge(c, a); // Crea un ciclo

        let result = topological_sort(&graph);
        assert!(result.is_err());
    }
}