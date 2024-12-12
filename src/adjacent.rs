use std::collections::HashMap;
use crate::PatientNode;

pub type PatientNodes = Vec<PatientNode>;
pub type GraphEdges = HashMap<String, Vec<String>>;
pub type AdjacencyMatrix = Vec<Vec<bool>>;

pub fn build_adjacency(
    nodes: PatientNodes,
    threshold: f64,
    weights: (f64, f64, f64), // (BMI weight, Smoking weight, Stroke weight)
) -> (GraphEdges, AdjacencyMatrix) {
    let num_nodes = nodes.len();
    let mut edges = HashMap::new();
    let mut matrix: AdjacencyMatrix = vec![vec![false; num_nodes]; num_nodes];

    for i in 0..num_nodes {
        for j in 0..num_nodes {
            if i == j {
                matrix[i][j] = true;
                continue;
            }

            let node_i = &nodes[i];
            let node_j = &nodes[j];

            let bmi_diff = (node_i.bmi - node_j.bmi).abs() * weights.0;
            let smoking_sim = if node_i.smoking == node_j.smoking {
                weights.1
            } else {
                0.0
            };
            let stroke_sim = if node_i.stroke == node_j.stroke {
                weights.2
            } else {
                0.0
            };

            let similarity_score = bmi_diff - smoking_sim - stroke_sim;

            if similarity_score <= threshold {
                edges
                    .entry(node_i.description.clone())
                    .or_insert_with(Vec::new)
                    .push(node_j.description.clone());
                matrix[i][j] = true;
            }
        }
    }

    (edges, matrix)
}
