use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::fmt;

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
pub struct AminoAcidSequence {
    pub aa_residual_composition: HashMap<char, Composition>,
}

impl AminoAcidSequence {
    pub fn new() -> AminoAcidSequence {
        let mut aa_residual_composition = HashMap::new();
        let compositions = vec![
            ("A", "C3H7O2N1"),
            ("R", "C6H14N4O2"),
            ("N", "C4H8O3N2"),
            ("D", "C4H7O4N1"),
            ("C", "C3H7O2N1S1"),
            ("E", "C5H9O4N1"),
            ("Q", "C5H10O3N2"),
            ("G", "C2H5O2N1"),
            ("H", "C6H9O2N3"),
            ("I", "C6H13O2N1"),
            ("K", "C6H14O2N2"),
            ("L", "C6H13O2N1"),
            ("M", "C5H11O2N1S1"),
            ("F", "C9H11O2N1"),
            ("P", "C5H9O2N1"),
            ("S", "C3H7O3N1"),
            ("T", "C4H9O3N1"),
            ("W", "C11H12O2N2"),
            ("Y", "C9H11O3N1"),
            ("V", "C5H11O2N1"),
        ];

        for (key, value) in compositions {
            aa_residual_composition.insert(key.chars().next().unwrap(), Composition::new(value)); 
        }

        AminoAcidSequence {
            aa_residual_composition,
        }
    }


}
pub fn generate_seqmz_candidates(res_seq: &AminoAcidSequence, max_mass: f64, aalist_startindex: usize) -> Vec<f64> {
    let mut result = Vec::new();
    let aalist = res_seq.aa_residual_composition.keys().collect::<Vec<_>>();
    
    let mut i = aalist_startindex;
    println!("test");
    if aalist.len() - 1 == aalist_startindex {
        loop {
            println!("calc: mass {}", res_seq.aa_residual_composition[&aalist[i]].mass());
            // TODO: multiple the chars by index i! account for this in the aminoacidseq
            let mass = if i < aalist.len() { res_seq.aa_residual_composition[&aalist[i]].mass() } else { 0.0 };
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
            let mass = if i < aalist.len() { res_seq.aa_residual_composition[&aalist[aalist_startindex]].mass() } else { 0.0 };
            let mass_remain = 1000.0 - mass;
            if mass_remain < 0.0 {
                break;
            }
            for r in generate_seqmz_candidates(res_seq, mass_remain, aalist_startindex + 1) {
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
    let mut masses = HashSet::new();
    generate_masses_recursive(amino_acids, &mut masses, max_mass, 0.0);
    masses
}

// Recursive function to generate masses
fn generate_masses_recursive(
    amino_acids: &[AminoAcid],
    masses: &mut HashSet<Float>,
    max_mass: f64,
    current_mass: f64,
) {
    println!("mass: {}, max_mass: {}", current_mass, max_mass);
    if current_mass < max_mass {
        println!("mass: {}", current_mass);
        masses.insert(Float(current_mass));
        for amino_acid in amino_acids {
            let new_mass = current_mass + amino_acid.mass.0; // Unwrap the Float
            println!("Adding Amino Acid: {}, New Mass: {:.4}", amino_acid.symbol, new_mass);
            generate_masses_recursive(amino_acids, masses, max_mass, new_mass);
        }
    }    
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
