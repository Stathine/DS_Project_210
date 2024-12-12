mod create;
mod adjacent;

use create::Graph;
use adjacent::{createadj, recommend};
use std::{collections::HashMap, error::Error};
use std::fs::File;

pub type Nodes = Vec<(String, (f64, f64, f64, f64, f64, f64, bool))>;

fn read(path: &str) -> Result<Nodes, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut nodes: Nodes = Vec::new();

    for (index, record) in rdr.records().enumerate() {
        let record = record?;
        let id = format!("Patient_{}", index);

        // Parse all fields
        let heart_rate: f64 = record.get(7).unwrap_or("0").parse()?;  // thalach
        let chest_pain: f64 = record.get(2).unwrap_or("0").parse()?; // cp
        let cholesterol: f64 = record.get(4).unwrap_or("0").parse()?; // chol
        let oldpeak: f64 = record.get(6).unwrap_or("0").parse()?;    // oldpeak
        let ca: f64 = record.get(8).unwrap_or("0").parse()?;         // ca
        let target: f64 = record.get(9).unwrap_or("0").parse()?;     // target
        let extra_value: f64 = record.get(11).unwrap_or("0").parse()?; // Extra numeric value
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
    let path = "heart_reduced_with_ptID.csv";
    let nodes = read(path).expect("Couldn't read the dataset!");
    let n = nodes.len();

    let threshold = 0.45; // Adjust as needxed
    let (adj_map, adj_matrix) = createadj(nodes.clone(), threshold, n);

    // Convert nodes to a HashMap for easy lookup
    let node_map: HashMap<String, (f64, f64, f64, f64, f64, f64, bool)> = nodes
        .into_iter()
        .map(|(id, attributes)| (id, attributes))
        .collect();

    let mut graph = Graph::new(n, node_map, adj_map, adj_matrix);
    graph.undirected();

    graph.analyze_neighborhoods();
    graph.distances();
    graph.components();

    let mut positive = graph.high_risk_pts(); // Get the list of positive patients
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

    let patient_id = "Patient_20".to_string(); // Focus only on Patient 20
    match graph.predict_angina(&patient_id) {
        Some(true) => println!("{} is likely to have exercise-induced angina.", patient_id),
        Some(false) => println!("{} is unlikely to have exercise-induced angina.", patient_id),
        None => println!("{} has no neighbors to make a prediction.", patient_id),
    }
}
