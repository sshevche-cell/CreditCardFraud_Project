// pagerank.rs
pub struct TransactionNode {
    pub features: Vec<f64>,
    pub pagerank_score: f64,
}

pub struct CreditCardGraph {
    pub nodes: Vec<TransactionNode>,
    pub edges: Vec<Vec<usize>>, // Adjacency list representation
}

pub fn initialize_pagerank(graph: &mut CreditCardGraph) {
    let nodes_len = graph.nodes.len();
    for node in &mut graph.nodes {
        node.pagerank_score = 1.0 / nodes_len as f64;
    }
}

pub fn pagerank(graph: &mut CreditCardGraph, damping_factor: f64, max_iterations: usize) {
    let mut iter_count = 0;
    let mut prev_scores: Vec<f64> = vec![0.0; graph.nodes.len()];

    while iter_count < max_iterations {
        // Update PageRank scores
        for i in 0..graph.nodes.len() {
            let mut new_score = (1.0 - damping_factor) / graph.nodes.len() as f64;

            for &j in &graph.edges[i] {
                new_score += damping_factor * prev_scores[j] / graph.edges[j].len() as f64;
            }

            graph.nodes[i].pagerank_score = new_score;
        }

        // Update previous scores for the next iteration
        prev_scores = graph.nodes.iter().map(|node| node.pagerank_score).collect();

        iter_count += 1;
    }
}

pub fn identify_fraudulent_transactions(graph: &CreditCardGraph, threshold_sum: f64) -> Vec<usize> {
    
    let fraudulent_indices: Vec<usize> = graph.nodes.iter()
        .enumerate()
        .filter(|(_, node)| node.features.iter().sum::<f64>() > threshold_sum)
        .map(|(index, _)| index)
        .collect();

    fraudulent_indices
}


pub fn evaluate_predictions(predicted: &[usize], actual_labels: &[bool]) {
    println!("Actual Labels: {:?}", actual_labels);
    println!("Predicted Indices: {:?}", predicted);

    let true_positives = predicted.iter().filter(|&&index| actual_labels.get(index).map_or(false, |&label| label)).count();
    let false_positives = predicted.iter().filter(|&&index| !actual_labels.get(index).map_or(false, |&label| label)).count();
    let false_negatives = actual_labels.iter().filter(|&&label| !predicted.contains(&(label as usize))).count();

    let precision = if true_positives + false_positives == 0 {
        0.0
    } else {
        true_positives as f64 / (true_positives + false_positives) as f64
    };

    let recall = if true_positives + false_negatives == 0 {
        0.0
    } else {
        true_positives as f64 / (true_positives + false_negatives) as f64
    };

    let f1_score = if precision + recall == 0.0 {
        0.0
    } else {
        2.0 * (precision * recall) / (precision + recall)
    };

    println!("Precision: {:.2}", precision);
    println!("Recall: {:.2}", recall);
    println!("F1 Score: {:.2}", f1_score);
}