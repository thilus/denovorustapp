mod spectrum;
use spectrum::Spectrum;
mod graph;
mod node;
mod composition;
use composition::AminoAcidSequence;
use composition::generate_seqmz_candidates;
use graph::Graph;
mod mass;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fs;

fn read_mgf_file_and_return_spectra(file_input: &str) -> Vec<Spectrum> {
    // Attempt to get the full path
    match fs::canonicalize(file_input) {
        Ok(full_path) => {
            // Full path retrieved successfully
            println!("Full path of the file: {}", full_path.to_string_lossy());
        }
        Err(e) => {
            // Error occurred while retrieving the full path
            eprintln!("Error getting full path: {}", e);
        }
    }

    let mut spectra: Vec<Spectrum> = Vec::new();

    if let Ok(file) = File::open(file_input) {
        let reader = BufReader::new(file);

        let mut current_spectrum = Spectrum::new();

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.starts_with("BEGIN IONS") {
                    current_spectrum = Spectrum::new(); // Initialize a new spectrum
                } else if line.starts_with("END IONS") {
                    // Push the current spectrum to the vector
                    spectra.push(current_spectrum.clone()); // Clone the spectrum to avoid moving
                } else if line.contains('=') {
                    // Parse header information
                    if line.starts_with("TITLE") {
                        current_spectrum.title = line.split('=').nth(1).unwrap_or("").trim().to_string();
                    } else if line.starts_with("CHARGE") {
                        current_spectrum.charge = line.split('=').nth(1).and_then(|s| s.trim().parse().ok()).unwrap_or(0); // Default value if parsing fails
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
                        current_spectrum.mz_vec.extend(mz_list);
                        current_spectrum.intensity_vec.extend(intensity_list);
                    }
                }
            }
        }
    } else {
        println!("Failed to open the file {}", file_input);
    }

    spectra
}

fn parse_mz_intensities(data: &str) -> Option<(Vec<f32>, Vec<f32>)> {
    let mut mz_list: Vec<f32> = Vec::new();
    let mut intensity_list: Vec<f32> = Vec::new();
    for data_line in data.lines() {
        let parts: Vec<&str> = data_line.split_whitespace().collect();
        if let Some(mz_str) = parts.get(0) {
            if let Some(intensity_str) = parts.get(1) {
                if let Ok(mz) = mz_str.parse::<f32>() {
                    if let Ok(intensity) = intensity_str.parse::<f32>() {
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
    // Mass generator test:

    let res_seq = AminoAcidSequence::new();
    let aalist = res_seq.aa_residual_composition.keys().collect::<Vec<_>>();
    // Iterate over the vector and print out the values
    for item in &aalist {
        let mass = res_seq.aa_residual_composition[&aalist[0]].mass();
        println!("AA {:?} {}", item, mass);
        
    }

    let max_mass = 200.0;
    let aalist_startindex = 0;
    //let result = generate_seqmz_candidates(&res_seq, max_mass, aalist_startindex);

    // Print the result
    //println!("Result: {:?}", result);

    let infile = "test.mgf";
    let spectra = read_mgf_file_and_return_spectra(infile);

    // Example usage of the mass calculation:
    let sequence = "PEPTIDE";
    let mass = mass::MassCalculator::calc_monoisotopic_mass(sequence, None, None, None);
    println!("Mass of sequence {}: {}", sequence, mass);

    // Print the spectra
    for spectrum in &spectra {
        let graph = Graph::generate_graph_from_spectrum(&spectrum); 
        println!("no. nodes: {}", graph.all_nodes.len());       
    }
}
