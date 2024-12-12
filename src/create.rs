use serde::Deserialize;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Deserialize)]
pub struct PatientRecord {
    #[serde(rename = "BMI")]
    pub bmi: Option<f64>,
    #[serde(rename = "Smoking", deserialize_with = "parse_bool")]
    pub smoking: Option<bool>,
    #[serde(rename = "Stroke", deserialize_with = "parse_bool")]
    pub stroke: Option<bool>,
    #[serde(rename = "AlcoholDrinking", deserialize_with = "parse_bool")]
    pub alcohol_drinking: Option<bool>,
    #[serde(rename = "DiffWalking", deserialize_with = "parse_bool")]
    pub diff_walking: Option<bool>,
}

pub fn parse_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: String = Deserialize::deserialize(deserializer)?;
    match value.to_lowercase().as_str() {
        "true" | "1" | "yes" => Ok(Some(true)),
        "false" | "0" | "no" => Ok(Some(false)),
        _ => Ok(None),
    }
}

#[derive(Debug, Clone)]
pub struct PatientNode {
    pub description: String,
    pub bmi: f64,
    pub smoking: bool,
    pub stroke: bool,
}

pub type PatientNodes = Vec<PatientNode>;
pub type GraphEdges = HashMap<String, Vec<String>>;
pub type AdjacencyMatrix = Vec<Vec<bool>>;

#[derive(Debug)]
pub struct HealthGraph {
    num_nodes: usize,
    nodes: PatientNodes,
    edges: GraphEdges,
    adjacency_matrix: AdjacencyMatrix,
}

impl HealthGraph {
    pub fn new(
        num_nodes: usize,
        nodes: PatientNodes,
        edges: GraphEdges,
        adjacency_matrix: AdjacencyMatrix,
    ) -> Self {
        HealthGraph {
            num_nodes,
            nodes,
            edges,
            adjacency_matrix,
        }
    }

    pub fn convert_to_undirected(&mut self) -> &HealthGraph {
        self.nodes.sort_by(|a, b| a.description.cmp(&b.description));
        self
    }

    pub fn filter_by_threshold(&self, threshold: f64) -> (Vec<PatientNode>, Vec<PatientNode>) {
        let (above_threshold, below_threshold): (Vec<_>, Vec<_>) = self
            .nodes
            .clone()
            .into_iter()
            .partition(|node| node.bmi >= threshold);
        (above_threshold, below_threshold)
    }

    pub fn print_risk_assessment(&self) {
        for i in 0..self.num_nodes {
            println!("Risk assessment for: {}", self.nodes[i].description);
            self.compute_risk(i);
        }
    }

    pub fn compute_risk(&self, index: usize) {
        let mut distances: Vec<Option<u32>> = vec![None; self.num_nodes];
        distances[index] = Some(0);
        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(index);

        while let Some(node_index) = queue.pop_front() {
            for (neighbor_index, &connected) in self.adjacency_matrix[node_index].iter().enumerate() {
                if connected && distances[neighbor_index].is_none() {
                    distances[neighbor_index] = Some(distances[node_index].unwrap() + 1);
                    queue.push_back(neighbor_index);
                }
            }
        }

        for node_index in 0..self.num_nodes {
            println!(
                "Distance to {}: {:?}",
                self.nodes[node_index].description, distances[node_index]
            );
        }
    }

    pub fn print_clusters(&self) {
        let mut cluster_ids: Vec<Option<usize>> = vec![None; self.num_nodes];
        let mut cluster_count = 0;
        let mut cluster_health_data: HashMap<usize, (usize, usize)> = HashMap::new();

        for node_index in 0..self.num_nodes {
            if cluster_ids[node_index].is_none() {
                cluster_count += 1;
                self.assign_cluster(node_index, &mut cluster_ids, cluster_count);
            }
        }

        println!("Total clusters: {}", cluster_count);
        for (node_index, cluster_id) in cluster_ids.iter().enumerate() {
            println!("Node {} belongs to cluster {:?}", self.nodes[node_index].description, cluster_id);

            if let Some(id) = cluster_id {
                let entry = cluster_health_data.entry(*id).or_insert((0, 0));
                if self.nodes[node_index].smoking {
                    entry.0 += 1; // Increment smoker count
                }
                entry.1 += 1; // Increment total node count
            }
        }

        for (cluster_id, (smokers, total)) in cluster_health_data.iter() {
            println!(
                "Cluster {}: {:.2}% smokers",
                cluster_id,
                (*smokers as f64 / *total as f64) * 100.0
            );
        }
    }

    pub fn find_k_best_representatives(&self, k: usize) {
        let mut cluster_ids: Vec<Option<usize>> = vec![None; self.num_nodes];
        let mut cluster_count = 0;
        let mut cluster_representatives: HashMap<usize, Vec<(String, usize)>> = HashMap::new();

        for node_index in 0..self.num_nodes {
            if cluster_ids[node_index].is_none() {
                cluster_count += 1;
                self.assign_cluster(node_index, &mut cluster_ids, cluster_count);
            }
        }

        for (node_index, cluster_id) in cluster_ids.iter().enumerate() {
            if let Some(id) = cluster_id {
                let degree = self.adjacency_matrix[node_index]
                    .iter()
                    .filter(|&&connected| connected)
                    .count();
                cluster_representatives
                    .entry(*id)
                    .or_insert_with(Vec::new)
                    .push((self.nodes[node_index].description.clone(), degree));
            }
        }

        for (cluster_id, mut representatives) in cluster_representatives.iter_mut() {
            representatives.sort_by(|a, b| b.1.cmp(&a.1));
            println!("Cluster {} representatives:", cluster_id);
            for rep in representatives.iter().take(k) {
                println!("  Node: {}, Degree: {}", rep.0, rep.1);
            }
        }
    }

    fn assign_cluster(&self, node_index: usize, cluster_ids: &mut Vec<Option<usize>>, cluster_id: usize) {
        cluster_ids[node_index] = Some(cluster_id);
        let mut queue = VecDeque::new();
        queue.push_back(node_index);

        while let Some(current_index) = queue.pop_front() {
            for (neighbor_index, &connected) in self.adjacency_matrix[current_index].iter().enumerate() {
                if connected && cluster_ids[neighbor_index].is_none() {
                    cluster_ids[neighbor_index] = Some(cluster_id);
                    queue.push_back(neighbor_index);
                }
            }
        }
    }
}
