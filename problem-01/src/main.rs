use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;
use std::collections::HashMap;


fn read_columns_from_file(file_path: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    // Open the file
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    // Initialize vectors to store the columns
    let mut column1 = Vec::new();
    let mut column2 = Vec::new();

    // Read the file line by line
    for line in reader.lines() {
        let line = line?; // Unwrap the result
        let numbers: Vec<&str> = line.split("   ").collect(); // Split by 4 spaces

        if numbers.len() == 2 {
            let num1: i32 = numbers[0].parse().unwrap(); // Parse the first number
            let num2: i32 = numbers[1].parse().unwrap(); // Parse the second number
            column1.push(num1);
            column2.push(num2);
        }
    }

    Ok((column1, column2)) // Return the vectors as a tuple
}

fn element_wise_difference(vec1: &[i32], vec2: &[i32]) -> Vec<i32> {
    vec1.iter()
        .zip(vec2.iter()) // Pair elements from both vectors
        .map(|(a, b)| (b - a).abs() ) // Compute the difference
        .collect() // Collect results into a new vector
}

fn count_occurrences(vec: &[i32]) -> HashMap<i32, i32> {
    let mut counts = HashMap::new();

    for &num in vec {
        *counts.entry(num).or_insert(0) += 1; // Increment the count for the number
    }

    counts
}

fn compute_similarity(vec1: &[i32], vec2: &[i32]) -> i32 {

    let v2_counts = count_occurrences(&vec2);

    let sim: Vec<i32> = vec1.iter()
                .map(|a| a*(*v2_counts.get(a).unwrap_or(&0)))
                .collect();

    sim.iter().sum()
}

fn main() -> io::Result<()> {
    let start = Instant::now(); // Start the timer

    // Path to the file
    let path = "input1.txt";

   // Call the function and get the columns
    let (mut column1, mut column2) = read_columns_from_file(path)?;

    // Print the vectors
    //println!("Column 1: {:?}", column1);
    //println!("Column 2: {:?}", column2);

    column1.sort_unstable();
    column2.sort_unstable();

    let diffs = element_wise_difference(&column1, &column2);
    //println!("Diffs: {:?}", diffs);
    let diffs_sum: i32 = diffs.iter().sum();

    let sim_score: i32 = compute_similarity(&column1, &column2);


    println!("Sum of differences: {}",diffs_sum);
    println!("Similarity Scores: {}",sim_score);
    
    let duration = start.elapsed(); // Get the elapsed time
    println!("Time elapsed: {:?}", duration);
    Ok(())
}