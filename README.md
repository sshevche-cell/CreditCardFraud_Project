# CreditCardFraud_Project
Sviatoslav Shevchenko, DS_210 Final Project


### Project Outline
In this project I aim to perform credit card fraud detection using the PageRank Algorithmn in Rust. The project is organized into three main modules: "csv_reader", "pagerank", and "main". 

#### 1. csv_reader Module
**This module handles reading the credit card dataset from a CSV file and creating a test dataset for the fraud detection algorithm.**

**Record Struct**: Represents a single record in the credit card dataset, containing time, numeric columns, amount, and class information.
**read_csv_file Function**: Reads the CSV file, parsing each record into a Record struct, and returns a vector of these records.
**create_test_dataset Function**: Takes the original dataset and selects a specified number of records for testing, ensuring a balance between non-fraudulent and fraudulent transactions.

#### 2. pagerank Module
**This module implements the PageRank algorithm, which assigns scores to each transaction node in the graph. It also provides functions for initializing PageRank scores, running the algorithm, and identifying potentially fraudulent transactions.**

**TransactionNode Struct**: Represents a node in the credit card transaction graph, storing features and PageRank scores.
**CreditCardGraph Struct**: Represents the credit card transaction graph with nodes and edges.
**initialize_pagerank Function**: Initializes the PageRank scores for each node in the graph.      
**pagerank Function**: Runs the PageRank algorithm on the graph, updating scores iteratively.        
**identify_fraudulent_transactions Function**: Identifies potentially fraudulent transactions based on a specified threshold sum.

#### 3. main Function
**The main function orchestrates the entire credit card fraud detection process.**

**Adjustable Parameters**: The user can set parameters such as the number of selected records, the threshold sum, damping factor, and maximum iterations for the PageRank algorithm.      
**Reading the Dataset**: Reads the original credit card dataset from a CSV file.        
**Creating the Test Dataset**: Utilizes the create_test_dataset function to create a test dataset, printing information about the selected records.      
**Initializing and Running PageRank**: Initializes a credit card graph, populates edges, and runs the PageRank algorithm using adjustable parameters.        
**Identifying Fraudulent Transactions**: Applies the fraud identification logic based on the calculated PageRank scores and evaluates predictions.   
**Evaluation Metrics**: The code calculates and prints precision, recall, and F1 score to evaluate the model's performance in identifying fraudulent transactions.

### Results Visualization




