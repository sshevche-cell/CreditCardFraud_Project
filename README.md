# CreditCardFraud_Project
Sviatoslav Shevchenko, DS_210 Final Project


### Project Outline

In this project I aim to perform credit card fraud detection using the PageRank Algorithmn in Rust. The project is organized into three main modules: "csv_reader", "pagerank", and "main".

#### Dataset 
The project utilizes a challenging dataset, described by the authors as “unbalanced” due to only 0.172% of the data comprising of fraudulent credit card transactions. Additionally, this dataset has more than 280,000 individual transactions, making it the largest dataset I worked with until now. Most of the features of the transactions are transformed (to numerical values) because of PCA (Principal Component Analysis) due to confidentiality concerns, this would most likely require me to test for multiple features and compare them, only allowing me to suspect what the features from V1 to V28 mean. The only features not transformed are Time, Amount and Class (fraudulent or not).      
link: https://www.kaggle.com/datasets/joebeachcapital/credit-card-fraud

#### 1. csv_reader Module
**This module handles reading the credit card dataset from a CSV file and creating a test dataset for the fraud detection algorithm.**

**Record Struct**:    Represents a single record in the credit card dataset, containing time, numeric columns, amount, and class information.
**read_csv_file Function**:    Reads the CSV file, parsing each record into a Record struct, and returns a vector of these records.
**create_test_dataset Function**:    Takes the original dataset and selects a specified number of records for testing, ensuring a balance between non-fraudulent and fraudulent transactions.

#### 2. pagerank Module
**This module implements the PageRank algorithm, which assigns scores to each transaction node in the graph. It also provides functions for initializing PageRank scores, running the algorithm, and identifying potentially fraudulent transactions.**

**TransactionNode Struct**:    Represents a node in the credit card transaction graph, storing features and PageRank scores.
**CreditCardGraph Struct**:    Represents the credit card transaction graph with nodes and edges.
**initialize_pagerank Function**:    Initializes the PageRank scores for each node in the graph.      
**pagerank Function**:    Runs the PageRank algorithm on the graph, updating scores iteratively.        
**identify_fraudulent_transactions Function**:    Identifies potentially fraudulent transactions based on a specified threshold sum.

#### 3. main Function
**The main function orchestrates the entire credit card fraud detection process.**

**Adjustable Parameters**:    The user can set parameters such as the number of selected records, the threshold sum, damping factor, and maximum iterations for the PageRank algorithm.      
**Reading the Dataset**:    Reads the original credit card dataset from a CSV file.        
**Creating the Test Dataset**:    Utilizes the create_test_dataset function to create a test dataset, printing information about the selected records.      
**Initializing and Running PageRank**:    Initializes a credit card graph, populates edges, and runs the PageRank algorithm using adjustable parameters.        
**Identifying Fraudulent Transactions**:    Applies the fraud identification logic based on the calculated PageRank scores and evaluates predictions.   
**Evaluation Metrics**:    The code calculates and prints precision, recall, and F1 score to evaluate the model's performance in identifying fraudulent transactions.
**Test CSV Reading**:    Validate CSV file reading.

#### 4. Tests
**The main function orchestrates the entire credit card fraud detection process.**
**Test CSV Reading**:    Validate CSV file reading.
**Test Fraudulent Identification**:    Verify accuracy in identifying fraud.


#### Results Visualization:
You can run the project from the main function, it has some adjustable parameters that the user can adjust before running main. I have provided screenshots of an example result output in this repository. Mind that the code might need some time to run due to teh size of the dataset. 


#### Limitations and Future Directions 
Most limitations of this project stem from teh daatset size, which was quite difficult to work with, especially due to teh confidential PCA information. The code most likley can be optimized to incraese accuracy, however it would require techniques that are perhaps out of the scope of this class. I believe that graph analysis is not teh best methodology to go about this dataset, with regression being the more obvious choice, graph analysis nevertheless could be utilized as well, but at a higher complexity in order to get better results. 

