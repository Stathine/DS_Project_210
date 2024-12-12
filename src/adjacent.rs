use std::collections::HashMap;

pub type Nodes = Vec<(String, (f64, f64, f64, f64, f64, f64, bool))>;
pub type Edges = HashMap<String, Vec<String>>;
pub type Matrix = Vec<Vec<bool>>;

pub fn createadj(nodes: Nodes, threshold: f64, n: usize) -> (Edges, Matrix) {
    let mut edges = HashMap::new();
    let mut matrix: Vec<Vec<bool>> = vec![vec![false; n]; n];
    let max_heart_rate = nodes.iter().map(|(_, (hr, _, _, _, _, _, _))| *hr).fold(0.0, f64::max);
    let max_chest_pain = nodes.iter().map(|(_, (_, cp, _, _, _, _, _))| *cp).fold(0.0, f64::max);
    let max_cholesterol = nodes.iter().map(|(_, (_, _, chol, _, _, _, _))| *chol).fold(0.0, f64::max);
    let max_oldpeak = nodes.iter().map(|(_, (_, _, _, oldpeak, _, _, _))| *oldpeak).fold(0.0, f64::max);
    let max_ca = nodes.iter().map(|(_, (_, _, _, _, ca, _, _))| *ca).fold(0.0, f64::max);
    let max_target = nodes.iter().map(|(_, (_, _, _, _, _, target, _))| *target).fold(0.0, f64::max);
    for (i, (id1, (hr1, cp1, chol1, oldpeak1, ca1, target1, _))) in nodes.iter().enumerate() {
        for (j, (id2, (hr2, cp2, chol2, oldpeak2, ca2, target2, _))) in nodes.iter().enumerate() {
            if i == j {
                matrix[i][j] = true;
                continue;
            }
            let hr_normalized = (hr1 - hr2).abs() / max_heart_rate;
            let cp_normalized = (cp1 - cp2).abs() / max_chest_pain;
            let chol_normalized = (chol1 - chol2).abs() / max_cholesterol;
            let oldpeak_normalized = (oldpeak1 - oldpeak2).abs() / max_oldpeak;
            let ca_normalized = (ca1 - ca2).abs() / max_ca;
            let target_normalized = (target1 - target2).abs() / max_target;
            let distance = (hr_normalized.powi(2)
                + cp_normalized.powi(2)
                + chol_normalized.powi(2)
                + oldpeak_normalized.powi(2)
                + ca_normalized.powi(2)
                + target_normalized.powi(2))
            .sqrt();
            if distance <= threshold {
                edges.entry(id1.clone())
                    .or_insert_with(Vec::new)
                    .push(id2.clone());
                matrix[i][j] = true;
            }
        }
    }
    (edges, matrix)
}

pub fn recommend(edges: Edges, nodes: &HashMap<String, (f64, f64, f64, f64, f64, f64, bool)>) -> HashMap<String, bool> {
    let mut recommendations = HashMap::new();
    for (node, neighbors) in edges.iter() {
        let angina_neighbors = neighbors
            .iter()
            .filter(|neighbor| nodes.get(*neighbor).map_or(false, |(_, _, _, _, _, _, angina)| *angina))
            .count();

        let angina_ratio = angina_neighbors as f64 / neighbors.len() as f64;
        recommendations.insert(node.clone(), angina_ratio > 0.5);
    }
    recommendations
}



