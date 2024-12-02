use std::fs::File;
use std::io::{self, BufRead};

fn read_rows_as_vectors(file_path: &str) -> io::Result<Vec<Vec<i32>>> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut rows = Vec::new();

    for line in reader.lines() {
        let line = line?; // Unwrap the result of reading a line
        let numbers: Vec<i32> = line
            .split_whitespace() // Split the line by spaces
            .map(|s| s.parse::<i32>().unwrap()) // Parse each substring into an integer
            .collect(); // Collect the numbers into a vector

        rows.push(numbers); // Add the vector to the list of rows
    }

    Ok(rows)
}


fn stepwise_diffs(vec1: &Vec<i32>) -> Vec<i32>{
    vec1.windows(2).map(|s| s[1] - s[0]).collect()
}

fn validate_vector(vec: &[i32]) -> i64 {
    if vec.is_empty() {
        return 1; // Consider an empty vector as valid
    }

    let first_sign = vec[0].is_positive(); // Determine the sign of the first element

    let is_valid = vec.iter()
        .fold(
            (true, i32::MAX, i32::MIN), // (valid, min_abs, max_abs)
            |(valid, min_abs, max_abs), &x| {
                let abs_value = x.abs();
                (
                    valid
                        && abs_value >= 1
                        && abs_value <= 3 // Check bounds
                        && x.is_positive() == first_sign, // Check sign consistency
                    min_abs.min(abs_value), // Update min absolute value
                    max_abs.max(abs_value), // Update max absolute value
                )
            },
        )
        .0;
    if is_valid {
        return 1;
    }

    else {
        return 0;
    }
}

fn drop_one<T: Clone>(v: &[T]) -> Vec<Vec<T>> {
    (0..v.len())
        .map(|i| {
            // Create a new vector by excluding the value at index `i`
            let mut new_vec = v.to_vec();
            new_vec.remove(i);
            new_vec
        })
        .collect()
}

fn map_and_check(vv: &[Vec<i32>], f: impl Fn(&Vec<i32>) -> i64) -> i64 {
    if vv.iter().any(|v| f(v) == 1) {
        1
    } else {
        0
    }
}

fn main() -> io::Result<()> {
    let file_path = "input1.txt"; // Replace with your file path

    let rows = read_rows_as_vectors(file_path)?;
    let mut safe_vals: i64 = 0;
    let mut already_safe: i64 = 0;
    for (_i, row) in rows.iter().enumerate() {
        
        already_safe = validate_vector(&stepwise_diffs(&row));
        if already_safe == 0 {
            let one_removed = drop_one(&row);
            let diffs_or: Vec<Vec<i32>> = one_removed.iter().map(|a| stepwise_diffs(&a)).collect();
            already_safe = map_and_check(&diffs_or, |arg0: &Vec<i32>| validate_vector(arg0));
            // println!("{}", already_safe);
        }
        safe_vals += already_safe
    }
    println!("{}", safe_vals);
    Ok(())
}
