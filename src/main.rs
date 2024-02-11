mod spectrum;
use spectrum::Spectrum;

mod mass;
use std::fs::File;
use std::io::{BufRead, BufReader};



fn read_mgf_file(file_input: &str) {
    
    if let Ok(file) = File::open(file_input) {
        let reader = BufReader::new(file);

        let mut spectra: Vec<Spectrum> = Vec::new();

        for line in reader.lines() {
            let mut current_spectrum = Spectrum::new();
            if let Ok(line) = line {
                if line.starts_with("BEGIN IONS") {
                    // Initialize a new spectrum                    
                } else if line.starts_with("END IONS") {
                    // Push the current spectrum to the vector
                    spectra.push(current_spectrum);
                } else if line.contains('=') {
                    // Parse header information
                    if line.starts_with("TITLE") {
                        current_spectrum.title = line.split('=').nth(1).unwrap_or("").trim().to_string();
                    } else if line.starts_with("CHARGE") {
                        current_spectrum.charge = line.split('=').nth(1).unwrap_or("").trim().to_string();
                    } else if line.starts_with("PEPMASS") {
                        current_spectrum.pepmass = line.split('=').nth(1).unwrap_or("").trim().to_string();
                    } else if line.starts_with("RTINSECONDS") {
                        current_spectrum.rtinseconds = line.split('=').nth(1).unwrap_or("").trim().to_string();
                    } else if line.starts_with("SCANS") {
                        current_spectrum.scan_number = line.split('=').nth(1).unwrap_or("0").trim().parse().unwrap_or(0);
                    } else if line.starts_with("PEPTIDE") {
                        current_spectrum.peptide = line.split('=').nth(1).unwrap_or("").trim().to_string();
                    } 
                } else {
                    // Parse mz and intensity values and directly insert them into current_spectrum
                    if let Some((mz_list, intensity_list)) = parse_mz_intensities(&line) {
                        current_spectrum.mz_list.extend(mz_list);
                        current_spectrum.intensity_list.extend(intensity_list);
                    }
                }
            }
        }
    } else {
        println!("Failed to open the file {}", file_input);
    }
}

fn parse_mz_intensities(data: &str) -> Option<(Vec<f64>, Vec<f64>)> {
    let mut mz_list: Vec<f64> = Vec::new();
    let mut intensity_list: Vec<f64> = Vec::new();
    for data_line in data.lines() {
        let parts: Vec<&str> = data_line.split_whitespace().collect();
        if let Some(mz_str) = parts.get(0) {
            if let Some(intensity_str) = parts.get(1) {
                if let Ok(mz) = mz_str.parse::<f64>() {
                    if let Ok(intensity) = intensity_str.parse::<f64>() {
                        mz_list.push(mz);
                        intensity_list.push(intensity);
                        println!("m/z: {}, int: {}", mz, intensity);
                    }
                }
            }
        }
    }
    if !mz_list.is_empty() && !intensity_list.is_empty() {
        Some((mz_list, intensity_list))
    } else {
        None
    }
}

fn main() {
    let infile = "test.mgf";
    read_mgf_file(infile);
   
    // Example usage of the mass calculation:
    let sequence = "PEPTIDE";
    let mass = mass::MassCalculator::calc_monoisotopic_mass(sequence, None, None, None);
    println!("Mass of sequence {}: {}", sequence, mass);

    
}


