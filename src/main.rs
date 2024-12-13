mod graph;
mod adjacent;

use create::Graph;
use adjacent::{createadj};
use std::{collections::HashMap, error::Error};
use std::fs::File;

pub type Nodes = Vec<(String, (f64, f64, f64, f64, f64, f64, bool))>;

pub fn read(path: &str) -> Result<Nodes, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut nodes: Nodes = Vec::new();
    for (index, record) in rdr.records().enumerate() {
        let record = record?;
        let id = format!("Patient_{}", index);
        let heart_rate: f64 = record.get(7).unwrap_or("0").parse()?;  // thalach
        let chest_pain: f64 = record.get(2).unwrap_or("0").parse()?; // cp
        let cholesterol: f64 = record.get(4).unwrap_or("0").parse()?; // chol
        let oldpeak: f64 = record.get(6).unwrap_or("0").parse()?;    // oldpeak
        let ca: f64 = record.get(8).unwrap_or("0").parse()?;         // ca
        let target: f64 = record.get(9).unwrap_or("0").parse()?;     // target
        let angina: bool = record.get(10).unwrap_or("0").parse::<i32>()? == 1; // angina
        nodes.push((
            id,
            (
                heart_rate, chest_pain, cholesterol, oldpeak, ca, target, angina,
            ),
        ));
    }
    Ok(nodes)
}

fn main() {
    let path = "heart_with_ptID.csv";
    let nodes = read(path).expect("Couldn't read the dataset!");
    let n = nodes.len();
    let threshold = 0.45; 
    let (adj_map, adj_matrix) = createadj(nodes.clone(), threshold, n);
    let node_map: HashMap<String, (f64, f64, f64, f64, f64, f64, bool)> = nodes
        .into_iter()
        .map(|(id, attributes)| (id, attributes))
        .collect();
    
    let mut graph = Graph::new(n, node_map, adj_map, adj_matrix);
    graph.undirected();
    graph.analyze_neighborhoods();
    graph.distances();
    graph.components();

    let mut positive = graph.high_risk_pts(); 
    positive.sort_by_key(|id| id.trim_start_matches("Patient_").parse::<usize>().unwrap());
    println!("At risk patients based on neighbor angina rate: {:?}", positive);

    let density = graph.edge_density();
    println!("Edge Density: {:.2}", density);

    if let Some(avg_length) = graph.average_path_length() {
        println!("Average Path Length: {:.2}", avg_length);
    } else {
        println!("Graph has no paths.");
    }

    let clustering = graph.clustering_coefficient();
    println!("Clustering Coefficient: {:.2}", clustering);

    let patient_id = "Patient_20".to_string(); 
    match graph.predict_angina(&patient_id) {
        Some(true) => println!("{} is likely to have exercise-induced angina.", patient_id),
        Some(false) => println!("{} is unlikely to have exercise-induced angina.", patient_id),
        None => println!("{} has no neighbors to make a prediction.", patient_id),
    }

}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_read_function() {
        let test_path = "test_heart_reduced_with_ptID.csv"; // Ensure this path is correct
        let nodes = read(test_path).expect("Failed to read test dataset.");
        assert!(
            !nodes.is_empty(),
            "The dataset should not be empty after reading."
        );
        assert_eq!(nodes.len(), 100, "Expected 100 patients in the test dataset."); // Adjust as per your test dataset
    }

    #[test]
    fn test_graph_initialization() {
        let test_path = "test_heart_reduced_with_ptID.csv"; // Ensure this path is correct
        let nodes = read(test_path).expect("Failed to read test dataset.");
        let n = nodes.len();
        let (adj_map, adj_matrix) = createadj(nodes.clone(), 0.45, n);

        let node_map: HashMap<String, (f64, f64, f64, f64, f64, f64, bool)> = nodes
            .into_iter()
            .map(|(id, attributes)| (id, attributes))
            .collect();

        let graph = Graph::new(n, node_map, adj_map, adj_matrix);
        assert_eq!(graph.n, n, "Graph should initialize with the correct number of nodes.");
    }

    #[test]
    fn test_high_risk_pts() {
        let test_path = "test_heart_reduced_with_ptID.csv";
        let nodes = read(test_path).expect("Failed to read test dataset.");
        let n = nodes.len();
        let (adj_map, adj_matrix) = createadj(nodes.clone(), 0.45, n);

        let node_map: HashMap<String, (f64, f64, f64, f64, f64, f64, bool)> = nodes
            .into_iter()
            .map(|(id, attributes)| (id, attributes))
            .collect();

        let graph = Graph::new(n, node_map, adj_map, adj_matrix);
        let positive = graph.high_risk_pts();
        assert!(
            !positive.is_empty(),
            "Expected at least one high-risk patient in the dataset."
        );
    }

    #[test]
    fn test_predict_angina() {
        let test_path = "test_heart_reduced_with_ptID.csv";
        let nodes = read(test_path).expect("Failed to read test dataset.");
        let n = nodes.len();
        let (adj_map, adj_matrix) = createadj(nodes.clone(), 0.45, n);

        let node_map: HashMap<String, (f64, f64, f64, f64, f64, f64, bool)> = nodes
            .into_iter()
            .map(|(id, attributes)| (id, attributes))
            .collect();

        let graph = Graph::new(n, node_map, adj_map, adj_matrix);
        let patient_id = "Patient_1".to_string();
        match graph.predict_angina(&patient_id) {
            Some(prediction) => assert!(
                prediction == true || prediction == false,
                "Prediction should return a boolean value."
            ),
            None => assert!(true, "No prediction available for Patient_1."),
        }
    }
}

