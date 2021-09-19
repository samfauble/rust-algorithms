pub mod max_flow_and_lp_algos {
    use crate::graph::*;

    pub fn edmonds_karp(graph: Graph, capacities: Vec<i32>, start: Vertex, end: Vertex) {
        let mut current_flow = 0;
        let mut res_net_edges: Vec<Edge> = Vec::new();

        for e in 0..graph.edges.len() - 1 {
            let edge = graph.edges[e];
            let c = capacities[e];
            if edge.weight < c {
                res_net_edges.push(Edge::new(edge.from, edge.to, c - edge.weight));
            } else if edge.weight > 0 {
                res_net_edges.push(Edge::new(edge.to, edge.from, edge.weight));
            }
        }

        let graph_f = Graph::new(res_net_edges, graph.vertices);
    }

    pub fn simplex() {

    }
}