pub mod graph_algos {
    extern crate queues;
    use queues::*;
    use std::collections::BTreeSet;
    use std::cmp::{Ord, Eq, PartialEq, PartialOrd};
    use std::ops::Index;
    use std::ptr::null;
    
    #[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Default)]
    pub struct Vertex {
        id: i32,
        pre_rank: i32,
        post_rank: i32
    }

    impl Vertex {
        pub fn new(num: i32) -> Vertex {
            Vertex {
                id: num,
                pre_rank: 0,
                post_rank: 0
            }
        }
    }

    pub struct Edge {
        from: Vertex,
        to: Vertex,
        weight: f32
    }

    impl Edge {
        pub fn new(v1: Vertex, v2: Vertex, weight: f32) -> Edge {
            Edge {
                from: v1,
                to: v2,
                weight
            }
        }
    }

    pub struct Graph {
        vertices: Vec<Vertex>,
        edges: Vec<Edge>
    }

    impl Graph {
        pub fn new(edges: &[Edge]) -> Graph {
            let mut set = BTreeSet::new();
            let mut vertices = Vec::new();
            let mut new_edges = Vec::new();
            
            //populate new_edges
            //get all unique values for vertex ids
            for e in edges {
                if !set.contains(&e.from) {
                    set.insert(e.from);
                }
                if !set.contains(&e.to) {
                    set.insert(e.to);
                }
            }

            //populate vertices arr
            for elem in set {
                vertices.push(elem);
            }

            Graph {
                vertices,
                edges: new_edges
            }
        }
    }
    
    /**
     * Depth-first search (DFS) executes a search through a graph
     * It's expecially useful when trying to find out information about the connectivity of the graph
     * As the name implies, DFS searches the graph by moving in a top to bottom fashion
     * The commented-out parts of the code are used for undirected graphs.
     * There will be comments next to the lines specifically for directed graphs
    */
    pub fn dfs(graph: Graph) -> Vec<Vertex> {
        //let mut current_connected_components = 0;
        //let mut connected_components_amount: Vec<i32> = Vec::new(); //make entry for each vertex
        let mut clock = 1; // part of directed graphs
        let mut visited: Vec<bool> = Vec::new();
        let mut vertex_stack: Vec<Vertex> = vec![graph.vertices[0]];
        
        for _i in &graph.vertices {
            visited.push(false);
        }
        
        //helper closure - visit the next vertex
        let mut visit = |z: usize| -> Result<usize, ()> {
            //get current vertex
            let current = graph.vertices[z];
            //connected_components_amount[z] = current_connected_components
            //find the outgoing edges of the current vertex
            let mut outgoing = graph.edges.iter().filter(|x| {x.to == current});
            let mut index: usize = 0;
            
            //find a destination vertex from outgoing edges that is unvisited
            let e = outgoing.find(|edge| {
                let v = edge.to;
                index = graph.vertices.iter().position(|&r| r == v).unwrap();
                !visited[index]
            });
            let next_vertex = match e {
                Some(eg) => eg.to,
                None => Vertex::new(-1)
            };

            //return the index of the next vertex to be processed
            if next_vertex.id > 0 {
                visited[index] = true;
                Ok(index)
            } else {
                Err(())
            }
        };

        while !vertex_stack.is_empty() {
            //current vertex being visited
            let mut current_vertex = vertex_stack[vertex_stack.len() - 1];
            let index = graph.edges.iter().position(|r| r.to == current_vertex).unwrap();

            match visit(index) {
                Ok(i) => {
                    //if vertex has an unvisited neighbor, add an unvisited neighbor
                    let mut next_vertex = graph.vertices[i];
                    vertex_stack.push(next_vertex);
                    next_vertex.pre_rank = clock; //part of directed graphs
                    clock += 1;
                },
                Err(()) => {
                    //when we reach a leaf vertex, pop from stack
                    current_vertex.post_rank = clock; //part of directed graphs
                    clock += 1;
                    vertex_stack.pop();
                }
             }
        }

        graph.vertices
    }

    /**
     * Breadth-first seach (BFS) is another way to search graphs similar to DFS. 
     * The difference between DFS and BFS is that BFS searches one complete level of vertices
     * of the graph from left to right before moving down to the next level.
     * Also unlike DFS, BFS takes both a graph and a starting vertex.
     * 
     * The output of this BFS implementation outputs an array of distances
     * between the starting vertex and every other vertex The indices of the distances array
     * match with the indices of the vertices in the graph struct.
     * 
     * BFS is better suited to search for the shortest path between two points.
     * Dijkstra's algorithm is a variation of BFS.dynamic
     */
    pub fn bfs(graph: &mut Graph, start: usize) {
        
        //Initialize data to be used
        let mut distances: Vec<i32> = Vec::new();
        for _i in 0..graph.vertices.len() - 1 {
            distances.push(i32::MAX);
        }
        distances[start] = 0;
        let mut q: Queue<usize> = Queue::new();
        
        //visit each vertex level-by-level
        while q.size() > 0 {
            let u = q.remove().unwrap();
            let outgoing_edges = graph.edges.iter().filter(|e| {e.from == graph.vertices[u as usize]});
        
            for edge in outgoing_edges {
                let v_to_index = graph.vertices.iter().position(|&r| r == edge.to).unwrap();
                
                if distances[v_to_index] == i32::MAX {
                    q.add(v_to_index).unwrap();
                    distances[v_to_index] = distances[u] + 1;
                }
            }
        }
    }

    pub fn bellman_ford() {

    }

    pub fn floyd_warshall() {

    }

    pub fn two_sat() {

    }

    pub fn kruskal_mst() {

    }
}