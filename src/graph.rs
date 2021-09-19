
    extern crate queues;
    extern crate petgraph;
    extern crate priority_queue;
    use queues::*;
    use std::cmp::{Ord, Eq, PartialEq, PartialOrd};
    use petgraph::unionfind::UnionFind;
    use petgraph::matrix_graph::IndexType;
    use priority_queue::PriorityQueue;
    use std::hash::Hash;

    //Answer element for the 2-SAT algorithm
    pub struct Answer {
        boolean: bool,
        vertex: Vertex
    }

    pub trait Complement {
        fn new_complement(id:i32) -> Vec<Vertex> {
            if id == 0 {
                panic!("Id cannot equal 0");
            }

            let first = Vertex {
                id,
                pre_rank: 0,
                post_rank: 0,
                scc_num: 0
            };
            let second = Vertex {
                id: -id,
                pre_rank: 0,
                post_rank: 0,
                scc_num: 0
            };

            vec![first, second]
        }
    }
    
    #[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Default, Hash)]
    pub struct Vertex {
        pub id: i32,
        pre_rank: i32,
        post_rank: i32,
        scc_num: i32
    }

    impl Complement for Vertex {}

    impl Vertex {
        pub fn new(num: i32) -> Self {
            Vertex {
                id: num,
                pre_rank: 0,
                post_rank: 0,
                scc_num: 0
            }
        }
    }

    #[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Default)]
    pub struct Edge {
        pub from: Vertex,
        pub to: Vertex,
        pub weight: i32
    }

    impl Edge {
        pub fn new(v1: Vertex, v2: Vertex, weight: i32) -> Self {
            Edge {
                from: v1,
                to: v2,
                weight
            }
        }
    }

    pub struct Graph {
        pub vertices: Vec<Vertex>,
        pub edges: Vec<Edge>
    }

    impl Graph {
        pub fn new(edges: Vec<Edge>, vertices: Vec<Vertex>) -> Self {
            Graph {
                vertices,
                edges
            }
        }
    }

    pub struct CNF {
        formula: Vec<Vec<Vertex>>
    }

    impl CNF {
        pub fn new(formula: Vec<Vec<Vertex>>) -> Self {
            formula.iter().for_each(|pair| {
                if pair.len() > 2 {
                    panic!("Complement subarrays must not be greater than 2 in length.");
                }
            });
            
            CNF {
                formula
            }
        }
    }
    
    /**
     * Depth-first search (DFS) executes a search through a graph
     * It's expecially useful when trying to find out information about the connectivity of the graph
     * As the name implies, DFS searches the graph by moving in a top to bottom fashion
     * The commented-out parts of the code are used for undirected graphs.
     * There will be comments next to the lines specifically for directed graphs
     * 
     * This current implmentation of DFS returns vertices with pre-order and post-order 
     * rankings, denoting the relative order in which they were first visited (pre-order) 
     * and in which all their child vertices were finished being visited (post-order)  
    */
    pub fn dfs(graph: Graph) -> Vec<Vertex> {
        let mut clock = 1;
        let mut vertex_stack: Vec<Vertex> = vec![];
        
        //helper closure - visit the next vertex
        let visit = |z: usize| -> Result<usize, ()> {
            //get current vertex
            let current = graph.vertices[z];
 
            //find the outgoing edges of the current vertex
            let mut outgoing = graph.edges.iter().filter(|x| {x.to == current});
            
            //find a destination vertex from outgoing edges that is unvisited
            let e = outgoing.find(|edge| {edge.to.pre_rank == 0});
            let next_vertex = match e {
                Some(eg) => eg.to,
                None => Vertex::new(-1)
            };

            //return the index of the next vertex to be processed
            if next_vertex.id > 0 {
                let index = graph.vertices.iter().position(|&r| r == next_vertex).unwrap();
                Ok(index)
            } else {
                Err(())
            }
        };

        for k in 0..graph.vertices.len() - 1 {
            if graph.vertices[k].pre_rank == 0 {        //if this vertex hasn't been visited yet, 
                vertex_stack.push(graph.vertices[k]);   //then visit all accessible vertices

                while !vertex_stack.is_empty() {        //explore accessible vertices
                    //current vertex being visited
                    let mut current_vertex = vertex_stack[vertex_stack.len() - 1]; //end of stack
                    let index = graph.edges.iter().position(|r| r.to == current_vertex).unwrap();
                    if current_vertex.pre_rank == 0 {
                        current_vertex.pre_rank = clock;
                        clock += 1;
                    }
                   
                    match visit(index) {
                        Ok(i) => {
                            //if vertex has an unvisited neighbor, add an unvisited neighbor
                            let next_vertex = graph.vertices[i];
                            vertex_stack.push(next_vertex);
                        },
                        Err(()) => {
                            //when a leaf vertex is reached, assign its post-order rank 
                            //and pop from stack
                            current_vertex.post_rank = clock;
                            clock += 1;
                            vertex_stack.pop();
                        }
                     }
                }
            }
        }
        graph.vertices
    }

    /**
     * An implementation of DFS for undirected graphs.
     * This implementation is used to determine the SCC membership of all vertices.
     * The definition for SCCs is located in the function description for find_scc
     */
    pub fn dfs_undirected(graph: Graph) -> Vec<Vertex> {
        let mut current_connected_components = 0;
        let mut vertex_stack: Vec<Vertex> = vec![];
        
        //helper closure - visit the next vertex
        let visit = |z: usize| -> Result<usize, ()> {
            //get current vertex
            let current = graph.vertices[z];
 
            //find the outgoing edges of the current vertex
            let mut outgoing = graph.edges.iter().filter(|x| {x.to == current});
            
            //find a destination vertex from outgoing edges that is unvisited
            let e = outgoing.find(|edge| {edge.to.scc_num == 0});
            let next_vertex = match e {
                Some(eg) => eg.to,
                None => Vertex::new(-1)
            };

            //return the index of the next vertex to be processed
            if next_vertex.id > 0 {
                let index = graph.vertices.iter().position(|&r| r == next_vertex).unwrap();
                Ok(index)
            } else {
                Err(())
            }
        };

        for k in 0..graph.vertices.len() - 1 {
            if graph.vertices[k].scc_num == 0 {         //if this vertex hasn't been visited yet, 
                vertex_stack.push(graph.vertices[k]);   //then visit all accessible vertices
                current_connected_components += 1;      //marks the start of a new SCC

                while !vertex_stack.is_empty() {        //explore accessible vertices
                    //current vertex being visited
                    let mut current_vertex = vertex_stack[vertex_stack.len() - 1]; //end of stack
                    let index = graph.edges.iter().position(|r| r.to == current_vertex).unwrap();
                    if current_vertex.scc_num == 0 {
                        current_vertex.scc_num = current_connected_components;
                    }
                   
                    match visit(index) {
                        Ok(i) => {
                            //if vertex has an unvisited neighbor, add an unvisited neighbor
                            let next_vertex = graph.vertices[i];
                            vertex_stack.push(next_vertex);
                        },
                        Err(()) => {
                            //when a leaf vertex is reached, pop from stack
                            vertex_stack.pop();
                        }
                     }
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
     * Dijkstra's algorithm is a variation of BFS
     */
    pub fn bfs(graph: &mut Graph, start: usize) -> Vec<i32> {
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
        distances
    }

    /**
     * An implementation of Dijkstra's shortest path algorithm
     * Input: Graph(vertices, edges w/ weights)
     * Output: shortest paths
     * 
     * As mentioned prviously, this algorithm assumes no negative weight values.
     * For the sake of optimizing for simplicity, I used a pre-made priority queue 
     * implementation. However, this implementation prioritizes maximum values. This
     * means that I needed to adapt my implementation to fit this constraint. 
     * The negative values used in this implementation, therefore, are used as a work-around.
     */
    pub fn dijkstra(graph: &Graph, start: Vertex) -> Vec<Option<Vertex>> {
        let mut dist = vec![];
        let mut prev: Vec<Option<Vertex>> = Vec::new();

        for i in 0..graph.vertices.len() - 1 {
            prev[i] = None;
            if graph.vertices[i] == start {
                dist[i] = 0;  
            } else {
                dist[i] = i32::MIN;
            }
        }
        let mut pq: PriorityQueue<Vertex, i32> = PriorityQueue::new();
        for v in 0..graph.vertices.len() - 1 {
            pq.push(graph.vertices[v], dist[v]);
        }

        while !pq.is_empty() {
            let (u, _val) = pq.pop().unwrap();

            for edge in graph.edges.iter().filter(|j|{j.from == u}) {
                let j = graph.vertices.iter().position(|v| {*v == edge.from}).unwrap();
                let k = graph.vertices.iter().position(|v| {*v == edge.to}).unwrap();

                if dist[k] < dist[j] - edge.weight {
                    dist[k] =  dist[j] - edge.weight;
                    prev[k] = Some(graph.vertices[j]);
                    pq.change_priority(&graph.vertices[k], dist[k]);
                }
            }
        }
        prev
    }

    /**
     * Dijkstra's algorithm is used under the assumption that all edge weights are positive.
     * If the weights are negative, however, that algorithm can't be used reliably.
     * The Bellman-Ford algorithm, like Dijstra's solves the shortest path problem.
     * However, This solution does not assume non-negative weight values.
     * If all weight values are indeed positive, though, Dijkstra's is faster than this algorithm.
     * Bellman-Ford solves for the path between a given starting vertex all all other vertices
     * 
     * Runtime: O(nm) where n = # vertices m = # edges
     */
    pub fn bellman_ford(graph: Graph, start: usize) -> Vec<i32> {
        let mut distances: Vec<Vec<i32>> = Vec::new();
        let mut answers: Vec<i32> = Vec::new();

        //populate starting values for distances array
        //and base cases for distances[0][v]
        for _e in 0..graph.edges.len() - 1 {
            let mut row: Vec<i32> = Vec::new();
            for _v in 0..graph.vertices.len() - 1 {
                row.push(i32::MAX);
            } 
            distances.push(row);
        }

        //base case for start vertex
        distances[0][start] = 0;

        for e in 1..graph.edges.len() - 1 {
            for z in 0..graph.vertices.len() - 1 {
                distances[e][z] = distances[e -1][z];
                let incoming_edges = graph.edges.iter().filter(|edge| {edge.to == graph.vertices[z]});
                //for all edges y -> z
                for ie in incoming_edges {
                    let y = graph.vertices.iter().position(|&r| r == ie.from).unwrap();
                    if distances[e][z] > distances[e-1][y] + (ie.weight as i32) {
                        distances[e][z] = distances[e-1][y] + (ie.weight as i32);
                    }
                }
            }
        }

        //populate the answers array
        for col in 0..distances[graph.edges.len() - 1].len() - 1 {
            answers.push(distances[graph.edges.len() - 1][col]);
        }
        answers
    }


    /**
     * Floyd-Warshall is similar to the Bellman-Ford algorithm above in that it can be
     * used as a fallback from Dijkstra's to find the shortest path where edges may be negative. 
     * The distinctive featur of Floyd-Warshall is that it solves for all vertex pairs.
     * Where Bellman-Ford solves for a single starting vertex, this algorithm solves for all
     * vertices as starting points to all vertices as endpoints.
     * 
     * This distinction makes Floyd-Warshall a better option if searching for negative weight cycles
     * in a graph as Bellman-Ford would only be able to find a negative weight cycle if it was
     * accessible from the starting point.
     * 
     * Runtime: O(n^3)
     */
    pub fn floyd_warshall(graph: Graph) -> Vec<Vec<i32>> {
        let mut distances: Vec<Vec<Vec<i32>>> = Vec::new();
        let mut answers: Vec<Vec<i32>> = Vec::new();

        for s in 0..graph.vertices.len() - 1 {
            for t in 0..graph.vertices.len() - 1 {
                let s_vertex = graph.vertices[s];
                let t_vertex = graph.vertices[t];
                let edge = graph.edges.iter().find(|e| {e.from == s_vertex && e.to == t_vertex});
                match edge {
                    Some(ed) => distances[0][s][t] = ed.weight as i32,
                    None =>  distances[0][s][t] = i32::MAX
                }
            }
        }

        for i in 0..graph.vertices.len() - 1 {
            for s in 0..graph.vertices.len() - 1 {
                for t in 0..graph.vertices.len() - 1 {
                    let nonexistent_path_value = distances[i-1][s][t];
                    let existing_path_value = distances[i-1][s][i] + distances[i-1][i][t];
                
                    //If the path exists, take that path value
                    if nonexistent_path_value <= existing_path_value { 
                        distances[i][s][t] = nonexistent_path_value;
                    } else {
                        distances[i][s][t] = existing_path_value;
                    }
                }
            }
        }
        for a in 0..graph.vertices.len() - 1 {
            for b in 0..graph.vertices.len() - 1 {
                answers[a][b] = distances[graph.vertices.len() - 1][a][b];
            }
        }

        answers
    }

    /**
     * This algorithm is used to find the SCCs of a directed graph
     * An SCC(strongly-connected component) is a cluster of vertices where:
     * All member vertices are accessible to one another
     * 
     * The meta-graph of a directed graph with SCCs is an undirected graph
     * This implementation returns a list of vertices with an assigned SCC number.
     * All vertices with the same SCC number are in the same SCC.
     */
    pub fn find_scc(graph: Graph) -> Vec<Vertex> {
        let reversed_edges = graph.edges.iter().map(|edge| {
            let to = edge.to;
            let from = edge.from;
            Edge::new(to, from, edge.weight)
        }).collect::<Vec<Edge>>();

        //Get post-order rank of reversed graph
        let reversed_graph = Graph::new(reversed_edges, graph.vertices);
        let mut ranked_vertices = dfs(reversed_graph);
        //sort vertices in descending order
        ranked_vertices.sort();  
        ranked_vertices.reverse();

        //run dfs (for undirected graphs) on original graph with ordered vertices 
        //return vertices with scc number
        let ordered_v_graph = Graph::new(graph.edges, ranked_vertices);
        dfs_undirected(ordered_v_graph)    
    }

    /**
     * Satisfiability (or SAT for short) begs the general question 
     * of whether or not a set of boolean variables in an expression can be assigned 
     * values in such a way to produce "True" as a result.
     * 
     * General notation and definition notes:
     * conjunctive normal form (CNF) - a formula for writing expressions for boolean logic.
     * The following is an example of CNF:
     * (x || x2) && (x3 || x1) && (x2 || x4) && (x3)
     * Notice that the parenthesis sets (from herein referred to as clauses) 
     * only contain OR gates, and AND gates are located between clauses.
     * Also notice that there are no more than two variables inside each clause.
     * This specific example is therefore a 2-SAT problem
     * 
     * For this specific implementation, each complement pair is given the same id
     * in absolute value; however, one value is positive, and its complement is negative.
     * The absolute value is used to identify them as a pair, and the sign is used to 
     * indicate them as complements.
     * 
     * This is an algorithm for solving the k-SAT problem where k = 2
     * This algorithm assumes ALL clauses are EXACTLY 2 variables in length
     * The Satisfiability problem (SAT for short) is the following:
     * 
     * Given a formula f in conjunctive normal form (CNF) with n variables and m clauses
     */
    pub fn two_sat(cnf: CNF) -> Option<Vec<Answer>> {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut edges: Vec<Edge> = Vec::new();
        let mut booleans: Vec<Answer> = vec![];

        //populate the vertices for the graph
        //populate the answers array
        cnf.formula.iter().for_each(|pair| {
             vertices.push(pair[0]);
             vertices.push(pair[1]);
             booleans.push(Answer{boolean: false, vertex: pair[0]});
             booleans.push(Answer{boolean: false, vertex: pair[1]});
        });

        //populate the edges for the graph
        //For clause (a || b), the edges are a -> -b and b -> -a
        cnf.formula.iter().for_each(|pair| {
            let first = pair[0];
            let second = pair[1];

            let from_first = vertices.iter().find(|vertex| {
                vertex.id.abs() == second.id.abs() && vertex.id.is_negative() != first.id.is_negative()
            }).unwrap();

            let from_second = vertices.iter().find(|vertex| {
                vertex.id.abs() == first.id.abs() && vertex.id.is_negative() != second.id.is_negative()
            }).unwrap();

            let edge_first = Edge {from: first, to: *from_first, weight: 0};
            let edge_second = Edge {from: second, to: *from_second, weight: 0};

            edges.push(edge_first);
            edges.push(edge_second);
        });

        let g = Graph::new(edges, vertices);
        let scc_vertices = find_scc(g);     //find scc groupings

        //Go through each SCC from sink to source
        //Satisfy all of the variables in each SCC
        let num_sccs = scc_vertices[0].scc_num;

        for i in 1..num_sccs {
            let current_scc = scc_vertices.iter().filter(|v| {v.scc_num == i});
            current_scc.for_each(|vertex| {
                let j = booleans.iter().position(|b| {b.vertex == *vertex}).unwrap();
                if vertex.id > 0 {
                    booleans[j].boolean = true;
                }
            });
        }

        //Evaluate the solvability of the formula
        let mut is_solvable = true;
        booleans.iter().for_each(|bl| {
            if (bl.vertex.id > 0 && !bl.boolean) || (bl.vertex.id < 0 && bl.boolean) {
                is_solvable = false;
            }
        });

        if is_solvable {
            Some(booleans)
        } else {
            Option::None
        }
    }

    /**
     * Kruskal's algorithm is used to find a minimum spanning tree (MST) 
     * in a given undirected graph. A minimum spanning tree is essentially
     * the largest tree that can be created while minimizing for 
     * the sum of the edge weights used
     * 
     * For the sake of optimizing for simplicity, I've chosen for this implementation
     * to assume a directed graph as input, which will be treated as an undirected graph
     * within the implementation.
     * 
     * The output is an array of all edges to be included in the MST
     */
    pub fn kruskal_mst(graph: &Graph) -> Vec<Edge> {
        let mut edges: Vec<Edge> = Vec::new();
        let mut answers: Vec<Edge> = Vec::new();
        graph.edges.iter().for_each(|elem| {edges.push(*elem)});

        edges.sort_by(|a, b| {a.weight.cmp(&b.weight)}); //sort edges in ascending order
        let mut uf: UnionFind<usize> = UnionFind::new(graph.vertices.len());
        
        //determine whether edge vertices have been added to the same root
        //if they have been added to the same root, that means that adding that
        //the current edge to the MST will cause a cycle, which isn't allowed in a tree
        for e in edges {
            let to_index = graph.vertices.iter().position(|y| {*y == e.to}).unwrap();
            let from_index = graph.vertices.iter().position(|x| {*x == e.from}).unwrap();

            let to_root = uf.find(to_index);
            let from_root = uf.find(from_index);

            if to_root != from_root {
                answers.push(e);
                uf.union(from_root, to_root);
            }
        }

        answers
    }

    pub fn page_rank() {

    }