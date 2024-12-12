mod create;
mod adjacent;

use create::Graph;
use adjacent::{createadj, recommend};
use std::{collections::HashMap, error::Error};
use std::fs::File;

pub type Nodes = Vec<(String, (f64, f64, f64, bool))>;

fn read(path: &str) -> Result<Nodes, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut nodes: Nodes = Vec::new();

    for (index, record) in rdr.records().enumerate() {
        let record = record?;
        let id = format!("Patient_{}", index);

        let heart_rate: f64 = record.get(7).unwrap_or("0").parse()?; // Adjust index for dataset column
        let chest_pain: f64 = record.get(2).unwrap_or("0").parse()?;
        let cholesterol: f64 = record.get(4).unwrap_or("0").parse()?;
        let angina: bool = record.get(10).unwrap_or("0").parse::<i32>()? == 1;

        nodes.push((id, (heart_rate, chest_pain, cholesterol, angina)));
    }

    Ok(nodes)
}

fn main() {
    let path = "heart_reduced_with_ptID.csv";
    let nodes = read(path).expect("Couldn't read the dataset!");
    let n = nodes.len();

    let threshold = 0.34; // Adjust as needed
    let (adj_map, adj_matrix) = createadj(nodes.clone(), threshold, n);

    // Convert nodes to a HashMap for easy lookup
    let node_map: HashMap<String, (f64, f64, f64, bool)> = nodes
        .into_iter()
        .map(|(id, attributes)| (id, attributes))
        .collect();

    let mut graph = Graph::new(n, node_map, adj_map, adj_matrix);
    graph.undirected();


    // Perform other graph-based operations
    let mut positive = graph.dailyexpect(); // Get the list of positive patients
    positive.sort_by_key(|id| id.trim_start_matches("Patient_").parse::<usize>().unwrap());
    println!("Positive patients based on neighbor angina rate: {:?}", positive);
    graph.analyze_neighborhoods();
    graph.portfolio();
    graph.groups();

    // Predict angina for a specific patient
    //let patient_id = "Patient_42".to_string(); // Example patient
    //match graph.predict_angina(&patient_id) {
        //Some(true) => println!(" {} is likely to have exercise-induced angina.", patient_id),
        //Some(false) => println!(" {} is unlikely to have exercise-induced angina.", patient_id),
        //None => println!(" {} has no neighbors to make a prediction.", patient_id),
    //}

    let patients = vec![
    3, 5, 7, 9, 10, 12, 13, 16, 17, 18, 19, 20, 22, 24, 29, 30, 33, 34, 35, 36, 37, 40, 42, 43, 46,
    47, 49, 54, 59, 67, 68, 69, 72, 74, 78, 79, 83, 86, 87,
    ];

    for patient_number in patients {
        let patient_id = format!("Patient_{}", patient_number); // Format each patient ID
        match graph.predict_angina(&patient_id) {
            Some(true) => println!("{} is likely to have exercise-induced angina.", patient_id),
            Some(false) => println!("{} is unlikely to have exercise-induced angina.", patient_id),
            None => println!("{} has no neighbors to make a prediction.", patient_id),
        }
    }

    let density = graph.edge_density();
    println!("Edge Density: {:.2}", density);

    if let Some(avg_length) = graph.average_path_length() {
        println!("Average Path Length: {:.2}", avg_length);
    } else {
        println!("Graph has no paths.");
    }

    let clustering = graph.clustering_coefficient();
    println!("Clustering Coefficient: {:.2}", clustering);

}
