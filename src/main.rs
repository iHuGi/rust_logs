use std::fs;
use std::io::Error;

/// Filters the input text and returns a collection of lines that begin with "ERROR".
fn extract_errors(text: &str) -> Vec<String> {
    let lines = text.split("\n");
    let mut results = vec![];

    for line in lines {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }
    
    results
}

/// Filters the input text and returns a collection of lines that begin with "WARNING".
fn extract_warnings(text: &str) -> Vec<String> {
    let lines = text.split("\n");
    let mut results = vec![];

    for line in lines {
        if line.starts_with("WARNING") {
            results.push(line.to_string());
        }
    }
    
    results
}

fn main() -> Result<(), Error> {
    // Read the entire log file into memory
    let text = fs::read_to_string("logs.txt")?;
    
    // Process and save error logs
    let error_logs = extract_errors(text.as_str());
    fs::write("errors.txt", error_logs.join("\n"))?;

    // Process and save warning logs
    let warnings_logs = extract_warnings(text.as_str());
    fs::write("warnings.txt", warnings_logs.join("\n"))?;

    // Print the success message to the console
    println!("Files were created successfully!");

    // Return success if all file operations complete without errors
    Ok(())
}

//fn main() {
//
//    let text = fs::read_to_string("logs.txt")
//        .expect("Failed to read logs.txt");
//
//    let error_logs = extract_errors(text.as_str());
//    fs::write("errors.txt", error_logs.join("\n"))
//        .expect("Failed to write errors.txt");
//
//    let warnings_logs = extract_warnings(text.as_str());
//     fs::write("warnings.txt", warnings_logs.join("\n"))
//        .expect("Failed to write warnings.txt");
//}


//fn main() {
//    match fs::read_to_string("logs.txt") {
//        Ok(file_content) => {
//            let error_logs = extract_errors(&file_content.as_str());
//            
//            match fs::write("errors.txt", error_logs.join("\n")) {
//                Ok(..) => println!("Wrote errors.txt"),
//                Err(write_error) => {
//                    println!("Writing of errors.txt failed: {}", write_error)
//                }
//            }
//        } 
//        Err(read_error) => {
//            println!("Failed to read file {}", read_error);
//        }
//    }
//}