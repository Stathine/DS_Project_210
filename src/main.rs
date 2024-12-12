use std::{error::Error, collections::HashMap};
use create::{PatientRecord, parse_bool, HealthGraph, PatientNode};

mod adjacent;
mod create;

pub type PatientNodes = Vec<PatientNode>;
pub type GraphEdges = HashMap<String, Vec<String>>;
pub type AdjacencyMatrix = Vec<Vec<bool>>;

fn read_csv(path: &str) -> Result<PatientNodes, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let mut nodes: PatientNodes = Vec::new();

    for result in rdr.deserialize() {
        let record: PatientRecord = result?;
        let node = PatientNode {
            description: format!(
                "BMI: {:.1}, Smoking: {:?}, Stroke: {:?}, Alcohol Drinking: {:?}, Difficulty Walking: {:?}",
                record.bmi.unwrap_or(0.0),
                record.smoking.unwrap_or(false),
                record.stroke.unwrap_or(false),
                record.alcohol_drinking.unwrap_or(false),
                record.diff_walking.unwrap_or(false)
            ),
            bmi: record.bmi.unwrap_or(0.0),
            smoking: record.smoking.unwrap_or(false),
            stroke: record.stroke.unwrap_or(false),
        };
        nodes.push(node);
    }

    Ok(nodes)
}

fn main() {
    let dataset_path = "heart_2020_reduced_100.csv";
    let patient_nodes = read_csv(dataset_path).expect("Couldn't read the dataset!");
    let similarity_weights = (1.0, 0.5, 0.3); // Weights: BMI, Smoking, Stroke
    let similarity_threshold = 5.0; // Similarity threshold

    let (edges, adjacency_matrix) =
        adjacent::build_adjacency(patient_nodes.clone(), similarity_threshold, similarity_weights);

    let mut health_graph =
        HealthGraph::new(patient_nodes.len(), patient_nodes.clone(), edges.clone(), adjacency_matrix.clone());
    let health_graph = health_graph.convert_to_undirected();

    let (positive_cases, negative_cases) = health_graph.filter_by_threshold(25.0);

    println!("Positive cases: {:?}", positive_cases);
    println!("Negative cases: {:?}", negative_cases);

    health_graph.print_risk_assessment();
    health_graph.print_clusters();
    health_graph.find_k_best_representatives(3); // Find top 3 representatives in each cluster
}
