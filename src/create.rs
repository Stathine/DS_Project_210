use std::collections::{HashMap, VecDeque};

pub type Edges = HashMap<String, Vec<String>>;
pub type Matrix = Vec<Vec<bool>>;

pub struct Graph {
    n: usize,
    nodes: HashMap<String, (f64, f64, f64, bool)>, 
    adj_map: Edges,
    adj_matrix: Matrix,
}

impl Graph {
    pub fn new(
        n: usize,
        nodes: HashMap<String, (f64, f64, f64, bool)>,
        adj_map: Edges,
        adj_matrix: Matrix,
    ) -> Self {
        Graph {
            n,
            nodes,
            adj_map,
            adj_matrix,
        }
    }

    pub fn undirected(&mut self) -> &Graph {
        self
    }

    pub fn dailyexpect(&self) -> Vec<String> {
        let mut positive = Vec::new();

        for (node, neighbors) in self.adj_map.iter() {
            let total_neighbors = neighbors.len();

            if total_neighbors == 0 {
                continue; // Skip patients with no neighbors
            }

            let angina_neighbors = neighbors
                .iter()
                .filter(|&neighbor| {
                    self.nodes.get(neighbor).map_or(false, |(_, _, _, angina)| *angina)
                })
                .count();

            let angina_rate = angina_neighbors as f64 / total_neighbors as f64;

            if angina_rate >= 0.7 {
                positive.push(node.clone());
            }
        }

        positive
    }

    pub fn portfolio(&self) {
        for node in self.nodes.keys() {
            println!("Distance from {}", node);
            self.risk(node);
        }
    }

    pub fn risk(&self, node: &String) {
        let mut distances: HashMap<String, Option<u32>> = self
            .nodes
            .keys()
            .map(|k| (k.clone(), None))
            .collect();
        distances.insert(node.clone(), Some(0));

        let mut queue = VecDeque::new();
        queue.push_back(node.clone());

        while let Some(v) = queue.pop_front() {
            if let Some(neighbors) = self.adj_map.get(&v) {
                for neighbor in neighbors {
                    if distances[neighbor].is_none() {
                        distances.insert(neighbor.clone(), Some(distances[&v].unwrap() + 1));
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        for (neighbor, distance) in distances.iter() {
            if let Some(d) = distance {
                println!("{}: {}", neighbor, d);
            }
        }
    }

    // new 
    pub fn analyze_neighborhoods(&self) {
        for (node, neighbors) in self.adj_map.iter() {
            println!("Node: {}, Immediate Neighbors: {:?}", node, neighbors);
        }
    }

    //new 
    pub fn edge_density(&self) -> f64 {
        let n = self.n;
        if n < 2 {
            return 0.0; // No edges possible for fewer than 2 nodes
        }

        // Count actual edges (undirected, so count each edge only once)
        let edge_count: usize = self.adj_map.values().map(|neighbors| neighbors.len()).sum();
        let actual_edges = edge_count / 2; // Each edge is counted twice in adj_map

        // Maximum possible edges in an undirected graph
        let max_edges = n * (n - 1) / 2;

        actual_edges as f64 / max_edges as f64
    }

    // new 
    pub fn average_path_length(&self) -> Option<f64> {
        let mut total_distance = 0;
        let mut pair_count = 0;

        for node in self.nodes.keys() {
            let distances = self.shortest_paths(node);
            for distance in distances.values() {
                if let Some(d) = distance {
                    total_distance += *d as usize;
                    pair_count += 1;
                }
            }
        }

        if pair_count == 0 {
            return None; // No paths in the graph
        }

        Some(total_distance as f64 / pair_count as f64)
    }

    fn shortest_paths(&self, start: &String) -> HashMap<String, Option<u32>> {
        let mut distances: HashMap<String, Option<u32>> = self
            .nodes
            .keys()
            .map(|node| (node.clone(), None))
            .collect();
        distances.insert(start.clone(), Some(0));

        let mut queue = VecDeque::new();
        queue.push_back(start.clone());

        while let Some(current) = queue.pop_front() {
            if let Some(neighbors) = self.adj_map.get(&current) {
                for neighbor in neighbors {
                    if distances[neighbor].is_none() {
                        distances.insert(neighbor.clone(), Some(distances[&current].unwrap() + 1));
                        queue.push_back(neighbor.clone());
                    }
                }
            }
        }

        distances
    }

    // new 
    pub fn clustering_coefficient(&self) -> f64 {
        let mut total_coefficient = 0.0;

        for (node, neighbors) in self.adj_map.iter() {
            let neighbor_count = neighbors.len();
            if neighbor_count < 2 {
                continue; // No possible connections between neighbors
            }

            // Count connections between neighbors
            let mut connections = 0;
            for i in 0..neighbor_count {
                for j in (i + 1)..neighbor_count {
                    if self.adj_map.get(&neighbors[i]).unwrap_or(&vec![]).contains(&neighbors[j]) {
                        connections += 1;
                    }
                }
            }

            let max_connections = neighbor_count * (neighbor_count - 1) / 2;
            total_coefficient += connections as f64 / max_connections as f64;
        }

        total_coefficient / self.n as f64
    }

    pub fn groups(&self) {
        let mut visited = HashMap::new();
        let mut count = 0;

        for node in self.nodes.keys() {
            if visited.get(node).is_none() {
                count += 1;
                let size = self.find_components(node, &mut visited, count);
                println!("Component {}: {} nodes", count, size);
            }
        }

        println!("{} components found", count);
    }

    fn find_components(
        &self,
        node: &String,
        visited: &mut HashMap<String, usize>,
        count: usize,
    ) -> usize {
        let mut queue = VecDeque::new();
        let mut size = 0;
        queue.push_back(node.clone());

        while let Some(v) = queue.pop_front() {
            if visited.insert(v.clone(), count).is_none() {
                size += 1;
                if let Some(neighbors) = self.adj_map.get(&v) {
                    for neighbor in neighbors {
                        if visited.get(neighbor).is_none() {
                            queue.push_back(neighbor.clone());
                        }
                    }
                }
            }
        }

        size
    }

    pub fn predict_angina(&self, patient: &String) -> Option<bool> {
        let neighbors = self.adj_map.get(patient)?;
        let angina_count = neighbors
            .iter()
            .filter(|neighbor| {
                self.nodes
                    .get(*neighbor)
                    .map_or(false, |(_, _, _, angina)| *angina)
            })
            .count();
    
        let total_neighbors = neighbors.len();
    
        if total_neighbors == 0 {
            return None;
        }
    
        let angina_ratio = angina_count as f64 / total_neighbors as f64;
    
        // Debugging: Print details
        println!(
            "Patient: {}, Neighbors: {:?}, Angina Count: {}, Total Neighbors: {}, Angina Ratio: {:.2}",
            patient, neighbors, angina_count, total_neighbors, angina_ratio
        );
    
        Some(angina_ratio > 0.4)
    }
    
}
