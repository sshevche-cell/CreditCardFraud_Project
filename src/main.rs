// main.rs

mod csv_reader;
mod pagerank;

use std::path::Path;
use csv_reader::{read_csv_file, create_test_dataset};
use pagerank::{CreditCardGraph, initialize_pagerank, pagerank, identify_fraudulent_transactions, evaluate_predictions};
use crate::pagerank::TransactionNode;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_reading() {
        let path = "test_data.csv"; 
        let expected_record_count = 42;

        // Check if the file exists before attempting to read it
        if Path::new(path).exists() {
            let records = read_csv_file(path, None);

            assert_eq!(records.len(), expected_record_count);

        } else {
            println!("Test data file not found at path: {}", path);
        }
    }

    #[test]
    fn test_fraudulent_identification() {
        // Create a test CreditCardGraph with known fraudulent transactions
        let graph = CreditCardGraph {
            nodes: vec![
                TransactionNode { features: vec![1.0, 2.0], pagerank_score: 0.1 },
            ],
            edges: vec![
                vec![1],
            ],
        };

        // Identify fraudulent transactions
        let fraudulent_transactions = identify_fraudulent_transactions(&graph, 0.5); 

        let expected_fraudulent_count = 1;

        // Ensure that the number of identified fraudulent transactions is as expected
        assert_eq!(fraudulent_transactions.len(), expected_fraudulent_count);

        // Ensure that specific nodes are identified as fraudulent
        assert!(fraudulent_transactions.contains(&0)); 
        }
    }

fn main() {
    let original_dataset = read_csv_file("creditcard_csv.csv", None);

    let selected_records_count = 4000; // Set your desired size of the test_dataset
    let threshold_sum = 5.0; // Set your desired threshold
    let damping_factor = 0.95; // Set your desired damping factor
    let max_iterations = 300; // Set your desired max iterations

    // Create the test dataset
    let test_dataset = create_test_dataset(&original_dataset, selected_records_count);

    // Print only 10 rows of the test dataset if it's not empty
    if !test_dataset.is_empty() {
        for record in &test_dataset[..10] {
            println!(
                "Time: [{}], Numeric Columns: [{}], Amount: {}, Class: {}",
                record.time,
                record.numeric_columns.iter().map(|&x| x.to_string()).collect::<Vec<String>>().join(", "),
                record.amount,
                record.class
            );
            println!();
        }
    } else {
        println!("Test dataset is empty.");
    }

    // Count and print the number of records in the test dataset
    let test_dataset_count = test_dataset.len();
    println!("Number of records in the test dataset: {}", test_dataset_count);

    // Use the test dataset to create a CreditCardGraph and apply PageRank algorithm
    let mut graph = CreditCardGraph {
        nodes: test_dataset.iter().take(selected_records_count).map(|record| TransactionNode {
            features: record.numeric_columns.clone(),
            pagerank_score: 0.0,
        }).collect(),
        edges: Vec::new(),
    };

    // Populate the edges vector
    for i in 0..graph.nodes.len() {
        let mut edges_for_node = Vec::new();
        for j in 0..graph.nodes.len() {
            if i != j {
                edges_for_node.push(j);
            }
        }
        graph.edges.push(edges_for_node);
    }

    let actual_labels: Vec<bool> = test_dataset.iter().map(|record| record.class == "1").collect();

    // Initialize and run PageRank algorithm with adjustable parameters
    initialize_pagerank(&mut graph);
    pagerank(&mut graph, damping_factor, max_iterations);

    // Identify fraudulent transactions and evaluate predictions
    let fraudulent_transactions = identify_fraudulent_transactions(&graph, threshold_sum);
    evaluate_predictions(&fraudulent_transactions, &actual_labels);
}