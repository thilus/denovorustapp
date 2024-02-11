use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_mgf_file(file_input: &str) {
    
    if let Ok(file) = File::open(file_input) {
        let reader = BufReader::new(file);

        let mut current_section = String::new();
        let mut header_lines = String::new();
        let mut current_section_data = String::new();

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("BEGIN IONS") {
                    // Start a new section
                    current_section = line;
                    header_lines.clear();
                    current_section_data.clear();
                } else if line.starts_with("END IONS") {
                    // Process the section
                    process_section(&current_section, &header_lines, &current_section_data);
                } else if line.contains('=') {
                    // Accumulate header lines
                    header_lines.push_str(&line);
                    header_lines.push('\n');
                } else {
                    // Accumulate data for the current section
                    current_section_data.push_str(&line);
                    current_section_data.push('\n');
                }
            }
        }
    } else {
        println!("Failed to open the file {}", file_input);
    }
}

fn process_section(header: &str, header_lines: &str, data: &str) {
    println!("{}", header);

    // Parse lines in the header
    let mut header_info: HashMap<&str, &str> = HashMap::new();
    for line in header_lines.lines() {
        if let Some(index) = line.find('=') {
            let (key, value) = line.split_at(index);
            header_info.insert(key.trim(), value.trim_start_matches('='));
        }
    }

    // Print parsed header information
    for (key, value) in &header_info {
        println!("{}: {}", key, value);
    }

    let mut mz_list: Vec<f64> = Vec::new();
    let mut intensity_list: Vec<f64> = Vec::new();
    //let mut data_line: String = String::new();
    for data_line in data.lines() {
        let parts: Vec<&str> = data_line.split_whitespace().collect();    
        let mz = parts[0].parse::<f64>().unwrap();
        let intensity = parts[1].parse::<f64>().unwrap();
        println!("m/z: {}, int: {}", mz, intensity);
        mz_list.push(mz);
        intensity_list.push(intensity);
    }
        
}




fn main() {
    let infile = "test.mgf";
    read_mgf_file(infile);
}


