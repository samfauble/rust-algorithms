pub mod max_flow_and_lp_algos {
    extern crate num;
    extern crate rand;
    use crate::graph::*;
    use rand::{Rng, thread_rng};

    pub struct SimplexVertex<T> {
        values: Vec<T>,
        neighbors: Vec<usize>
    }

    pub fn edmonds_karp(graph: &mut Graph, capacities: Vec<i32>, start: Vertex, end: Vertex) -> u32 {
        let mut current_flow = 0;
        let mut no_path = false;
        let mut res_net_edges: Vec<Edge> = Vec::new();
        let mut verts: Vec<Vertex> = Vec::new();

        for r in 0..graph.vertices.len() - 1 {verts.push(graph.vertices[r]);}
        
        //Build the residual network Gf for current flow
        for e in 0..graph.edges.len() - 1 {
            let edge = graph.edges[e];
            let c = capacities[e];
            if edge.weight < c {  
                //add forward edge w/ capacity c - flow of edge
                res_net_edges.push(Edge::new(edge.from, edge.to, c - edge.weight));
            } 
            if edge.weight > 0 {
                //add reverse edge w/ capacity flow of edge
                res_net_edges.push(Edge::new(edge.to, edge.from, edge.weight));
            }
        }

        let mut graph_f = Graph::new(res_net_edges, verts);
        let path = dijkstra(&graph_f, start);

        let end_index = graph_f.vertices.iter().position(|x|{x == &end}).unwrap();
        no_path = match path.iter().find(|v| {v.unwrap() == end}) {
            Some(v) => false,
            None => true
        };

        if no_path {
            return current_flow;
        } else {
            let mut min_capacity = i32::MAX;
        let mut next_v = end_index;
        while path[next_v] != None {
            let from = path[next_v].unwrap();
            let to = graph_f.vertices[next_v];
            let connecting_edge = graph_f.edges.iter().find(|e|{e.from == from && e.to == to}).unwrap();
            if connecting_edge.weight < min_capacity {
                min_capacity = connecting_edge.weight;
            }
            
            next_v = graph_f.vertices.iter().position(|y|{y == &from}).unwrap();
        }

        next_v = end_index;
        while path[next_v] != None {
            let from = path[next_v].unwrap();
            let to = graph_f.vertices[next_v];
            let f_edge = graph_f.edges.iter_mut().find(|e|{e.from == from && e.to == to}).unwrap();
            let og_edge = graph.edges.iter().position(|e|{e.from == from && e.to == to});
            let og_edge_r = graph.edges.iter().position(|e|{e.from == to && e.to == from}).unwrap();
            
            match og_edge {
                Some(eg) => graph.edges[eg].weight += min_capacity,
                None => graph.edges[og_edge_r].weight -= min_capacity
            }
            
            next_v = graph_f.vertices.iter().position(|y|{y == &from}).unwrap();
        }

        edmonds_karp(graph, capacities, start, end);
        }

        current_flow

    }

    pub fn simplex<N>(feasible_vertices: Vec<SimplexVertex<u128>>, value: N)  -> (Vec<u128>, u128)
        where N: Fn(Vec<u128>) -> u128 {
            let mut starting_point: Vec<u128> = vec![];
            feasible_vertices[0].values.iter().for_each(|_val|{starting_point.push(0)});
            
            let mut answer = 0;
            let mut current_value = value(starting_point);
            let mut current_point = 0;
            while current_value >= answer {
                let n_length = feasible_vertices[current_point].neighbors.len();
                let rand = thread_rng().gen_range(0..n_length);
                let neighbor = feasible_vertices[current_point].neighbors[rand];
                let mut next_point = vec![];
                feasible_vertices[neighbor].values.iter().for_each(|_val|{next_point.push(0)});
                
                answer = current_value;
                current_value = value(next_point);
                current_point = neighbor;
            }

            let mut answer_arr = vec![]; 
            feasible_vertices[current_point].values.iter().for_each(|v| {answer_arr.push(*v)});

            (answer_arr, answer)
        }
}