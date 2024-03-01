use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Composition {
    pub composition: HashMap<char, i32>,
    pub atom_mass: HashMap<char, f64>,
    pub mass: f64,
}

impl Composition {
    pub fn new(class_input: &str) -> Self {
        let mut composition = HashMap::with_capacity(class_input.len());
        let mut atom_mass = HashMap::new();
        atom_mass.insert('H', 1.00782503223);
        atom_mass.insert('X', 1.007276466621);        
        atom_mass.insert('C', 12.0);
        atom_mass.insert('N', 14.00307400443);
        atom_mass.insert('O', 15.99491461957);
        atom_mass.insert('P', 30.97376199842);
        atom_mass.insert('S', 31.9720711744);

        if class_input.chars().all(|c: char| c.is_uppercase() || c.is_numeric()) {            
            let mut iter = class_input.chars().peekable();
            let mut current_atom = iter.next().unwrap();
            let mut count_str = String::new();            
            while let Some(c) = iter.next() {               
                if c.is_numeric() {
                    count_str.push(c);           
                } else {
                    if !count_str.is_empty() {
                        let count = count_str.parse().unwrap_or(1);                        
                        composition.insert(current_atom, count);
                        current_atom = c;
                        count_str.clear();
                    }                   
                }
            }        
            composition.insert(current_atom, count_str.parse().unwrap_or(1));
        } else {            
            composition.insert(class_input.chars().next().unwrap(), 1);
        }

        let mass = composition.iter()
            .map(|(&k, &v)| v as f64 * atom_mass[&k])
            .sum();

        Composition {
            composition,
            atom_mass,
            mass,
        }
    }

    pub fn comp2formula(&self) -> String {
        self.composition.iter()
            .map(|(&k, &v)| format!("{}{}", k, v))
            .collect()
    }

    pub fn output_neutron() -> f64 {
        1.00866491595
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }
}

impl PartialEq for Composition {
    fn eq(&self, other: &Self) -> bool {
        self.composition == other.composition
    }
}

impl PartialOrd for Composition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.mass.partial_cmp(&other.mass)
    }
}

impl Eq for Composition {}

impl Ord for Composition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Display for Composition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Composition({:?})", self.composition)
    }
}

#[derive(Debug, Clone)]
pub struct AminoAcidSet {
    pub aa_residual_composition: HashMap<char, Composition>,
}

impl AminoAcidSet {
    pub fn new() -> AminoAcidSet {
        // Creates a mutable hash map is creates which stores the composition of the amino acids
        let mut aa_residual_composition = HashMap::new();
        // Defines a vector of tuples: each tuple contains an amino acid symbol and its composition
        let compositions = vec![
            ("A", "C3H5O1N1"),
            ("R", "C6H12N4O1"),
            ("N", "C4H6O2N2"),
            ("D", "C4H5O3N1"),
            ("C", "C3H5O1N1S1"),
            ("E", "C5H7O3N1"),
            ("Q", "C5H8O2N2"),
            ("G", "C2H3O1N1"),
            ("H", "C6H7O1N3"),
            ("I", "C6H11O1N1"),
            ("K", "C6H12O1N2"),
            ("L", "C6H11O1N1"),
            ("M", "C5H9O1N1S1"),
            ("F", "C9H9O1N1"),
            ("P", "C5H7O1N1"),
            ("S", "C3H5O2N1"),
            ("T", "C4H7O2N1"),
            ("W", "C11H10O1N2"),
            ("Y", "C9H9O2N1"),
            ("V", "C5H9O1N1"),
        ];

        for (key, value) in compositions {
            aa_residual_composition.insert(key.chars().next().unwrap(), Composition::new(value)); 
        }

        AminoAcidSet {
            aa_residual_composition,
        }
    }

}

pub fn generate_seqmz_candidates(aa_set: &AminoAcidSet, max_mass: f64, aalist_startindex: usize) -> Vec<f64> {
    let mut result = Vec::new();
    let aalist = aa_set.aa_residual_composition.keys().collect::<Vec<_>>();
    
    let mut i = aalist_startindex;  
    if aalist.len() - 1 == aalist_startindex {
        loop {
            println!("calc: mass {}", aa_set.aa_residual_composition[&aalist[i]].mass());
            // TODO: multiple the chars by index i! account for this in the aminoacidseq
            let mass = if i < aalist.len() { aa_set.aa_residual_composition[&aalist[i]].mass() } else { 0.0 };
            let mass_remain = 1000.0 - mass;
            println!("mass remain: {}", mass_remain);
            if mass_remain < 0.0 {
                break;
            } 
            println!("mass: {}", mass);
            result.push(mass);
            i += 1;
        }
    } else {
        i = 0;
        loop {
            let mass = if i < aalist.len() { aa_set.aa_residual_composition[&aalist[aalist_startindex]].mass() } else { 0.0 };
            let mass_remain = 1000.0 - mass;
            if mass_remain < 0.0 {
                break;
            }
            for r in generate_seqmz_candidates(aa_set, mass_remain, aalist_startindex + 1) {
                result.push(mass + r);
                println!("mass + r: {}", mass + r);
            }
            i += 1;
        }
    }

    result
}

// Newtype wrapper around f64 implementing Eq and Hash
#[derive(Debug, Copy, Clone)]
pub struct Float(f64);

impl Eq for Float {}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 || (self.0 - other.0).abs() < f64::EPSILON
    }
}

impl Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the bits representation of the float
        state.write_u64(self.0.to_bits());
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Define a struct to represent an amino acid with its mass
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct AminoAcid {
    pub symbol: &'static str,
    pub mass: Float, // Use the Float wrapper type
}

impl AminoAcid {
    pub fn new(symbol: &'static str, mass: f64) -> Self {
        AminoAcid { symbol, mass: Float(mass) } // Wrap the mass in Float
    }
}

// Function to generate all possible masses from a set of amino acids
pub fn generate_masses(amino_acids: &[AminoAcid], max_mass: f64) -> HashSet<Float> {
    if let Some(masses) = load_generated_masses() {
        return masses;
    }

    let mut masses = HashSet::new();
    let mut stack = Vec::new();

    // Initialize the stack with initial state
    stack.push((0, 0.0));

    while let Some((index, current_mass)) = stack.pop() {
        if current_mass < max_mass {
            masses.insert(Float(current_mass));

            // Iterate over the remaining amino acids
            for i in index..amino_acids.len() {
                let new_mass = current_mass + amino_acids[i].mass.0;
                println!("Adding Amino Acid: {}, New Mass: {:.4}", amino_acids[i].symbol, new_mass);
                stack.push((i, new_mass));
            }
        }
    }
    dump_generated_masses(&masses);
    masses
}

// Function to dump generated masses into a file
fn dump_generated_masses(masses: &HashSet<Float>) {
    if let Ok(mut file) = File::create("generated_masses.txt") {
        let mut writer = BufWriter::new(&mut file);
        for mass in masses {
            writeln!(writer, "{}", mass.0).unwrap();
        }
    }
}

// Function to load generated masses from a file
fn load_generated_masses() -> Option<HashSet<Float>> {
    let path = Path::new("generated_masses.txt");
    if let Ok(file) = File::open(path) {
        let reader = BufReader::new(file);
        let mut masses = HashSet::new();
        for line in reader.lines() {
            if let Ok(mass_str) = line {
                if let Ok(mass_value) = mass_str.parse::<f64>() {
                    masses.insert(Float(mass_value));
                }
            }
        }
        return Some(masses);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_with_uppercase_input() {
        // Test creating a Composition with uppercase input
        let class_input = "C6H14N4O2"; // Example input
        let composition = Composition::new(class_input);
        // Iterate over the keys and values
        for (key, value) in &composition.composition {
            println!("Atom: {} | Count: {}", key, value);
        }

        // Verify that the composition contains the expected atoms and counts
        let mut expected_composition = HashMap::new();
        expected_composition.insert('C', 6);
        expected_composition.insert('H', 14);
        expected_composition.insert('O', 2);
        expected_composition.insert('N', 4);
        assert_eq!(composition.composition, expected_composition);

        // Verify the calculated mass (you can adjust the expected value based on your actual data)
        assert_eq!(composition.mass(), 174.11167570807999, "Unexpected mass for C6H14N4O2");
    }

    #[test]
    fn test_new_with_single_atom() {
        // Test creating a Composition with a single atom
        let class_input = "C"; // Example input
        let composition = Composition::new(class_input);

        // Verify that the composition contains only the specified atom
        let expected_composition: HashMap<char, i32> = [('C', 1)].iter().cloned().collect();
        assert_eq!(composition.composition, expected_composition);

        // Verify the calculated mass (you can adjust the expected value based on your actual data)
        assert_eq!(composition.mass(), 12.0, "Unexpected mass for C");
    }

     
        
}
