// csv_reader.rs

use std::fs::File;
use csv::ReaderBuilder;
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct Record {
    pub time: String,
    pub numeric_columns: Vec<f64>,
    pub amount: f64,
    pub class: String,
}

// Define the main function to read records from a CSV file
pub fn read_csv_file(path: &str, range: Option<std::ops::Range<usize>>) -> Vec<Record> {
    // Initialize an empty vector to store the parsed records
    let mut result: Vec<Record> = Vec::new();

    let file = File::open(path).expect("Could not open file");

    let mut rdr = ReaderBuilder::new().delimiter(b',').from_reader(file);

    // Iterate through each record in the CSV file
    for (index, record) in rdr.records().enumerate() {
        if let Some(ref range) = range {
            if index < range.start {
                continue; 
            }
            if index >= range.end {
                break; 
            }
        }

        // Match the result of reading a record
        match record {
            Ok(record) => {
                let time = record[0].to_string();
                let mut numeric_columns = Vec::new();

                // Iterate through numeric columns (1 to 28) and parse them as f64
                for i in 1..29 {
                    let value = record[i].parse::<f64>().expect(&format!("Error parsing column {}", i));
                    numeric_columns.push(value);
                }

                let amount = record[29].parse::<f64>().expect("Error parsing amount");
                let class = record[30].to_string();

                // Create a Record struct and push it to the result vector
                let record_struct = Record {
                    time,
                    numeric_columns,
                    amount,
                    class,
                };

                result.push(record_struct);
            }
            Err(err) => {
                eprintln!("Error reading CSV record: {}", err);
            }
        }
    }

    // Return the vector containing the parsed records
    result
}


pub fn create_test_dataset(original_dataset: &[Record], selected_records_count: usize) -> Vec<Record> {
    let mut rng = rand::thread_rng();

    // Class is the 31st column

    let non_fraudulent_records: Vec<_> = original_dataset
        .iter()
        .filter(|record| record.class == "'0'") //"'0'" represents non-fraudulent
        .cloned()
        .collect();

    let fraudulent_records: Vec<_> = original_dataset
        .iter()
        .filter(|record| record.class == "'1'") //"'1'" represents fraudulent
        .cloned()
        .collect();

    println!("Original Dataset Size: {}", original_dataset.len());
    println!("Non-Fraudulent Records: {}", non_fraudulent_records.len());
    println!("Fraudulent Records: {}", fraudulent_records.len());

    println!("Non-Fraudulent Records: {:?}", non_fraudulent_records);
    println!("Fraudulent Records: {:?}", fraudulent_records);

    // Ensure that both non-fraudulent and fraudulent records are not empty
    if non_fraudulent_records.is_empty() || fraudulent_records.is_empty() {
        panic!("Not enough data to create a non-empty test dataset");
    }

    let selected_fraudulent: Vec<_> = fraudulent_records.into_iter().collect();

    let remaining_non_fraudulent_count = selected_records_count - selected_fraudulent.len();
    let selected_non_fraudulent: Vec<_> = non_fraudulent_records.into_iter().take(remaining_non_fraudulent_count).collect();

    // Combine the selected records to form the final test dataset
    let mut test_dataset = Vec::new();
    test_dataset.extend(selected_non_fraudulent);
    test_dataset.extend(selected_fraudulent);

    // Shuffle the final test dataset
    test_dataset.shuffle(&mut rng);

    // Take only the specified number of records or all if the dataset is smaller
    test_dataset.into_iter().take(selected_records_count).collect()
}