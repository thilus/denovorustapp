use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Composition {
    pub composition: HashMap<char, i32>,
    pub atom_mass: HashMap<char, f64>,
    pub mass: f64,
}

impl Composition {
    pub fn new(class_input: &str) -> Self {
        let mut composition = HashMap::new();
        let mut atom_mass = HashMap::new();
        atom_mass.insert('H', 1.00782503223);
        atom_mass.insert('C', 12.0);
        atom_mass.insert('N', 14.00307400443);
        atom_mass.insert('O', 15.99491461957);
        atom_mass.insert('P', 30.97376199842);
        atom_mass.insert('S', 31.9720711744);

        if class_input.chars().all(|c| c.is_uppercase()) {
            let iter = if class_input.starts_with('-') {
                class_input[1..].chars()
            } else {
                class_input.chars()
            };
            composition = iter.filter(|c| c.is_alphabetic())
                .map(|c| (c, class_input.matches(c).count() as i32))
                .collect();
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
            ("A", "C3H5ON"),
            ("R", "C6H12ON4"),
            ("N", "C4H6O2N2"),
            ("D", "C4H5O3N"),
            ("M", "C3H5ONS-H+C2H4ON"),
            ("E", "C5H7O3N"),
            ("Q", "C5H8O2N2"),
            ("G", "C2H3ON"),
            ("H", "C6H7ON3"),
            ("I", "C6H11ON"),
            ("K", "C6H12ON2"),
            ("M", "C5H9ONS"),
            ("M", "C5H9ONS+O"),
            ("F", "C9H9ON"),
            ("P", "C5H7ON"),
            ("S", "C3H5O2N"),
            ("T", "C4H7O2N"),
            ("W", "C11H10ON2"),
            ("Y", "C9H9O2N"),
            ("V", "C5H9ON"),
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
    // Iterate over the vector and print out the values
    for item in &aalist {
        println!("{:?}", item);
    }
    let mut i = 0;

    if aalist.len() - 1 == aalist_startindex {
        loop {
            let mass = if i == 0 { 0.0 } else { res_seq.aa_residual_composition[&aalist[aalist_startindex]].mass() };
            let mass_remain = max_mass - mass;
            if mass_remain < 0.0 {
                break;
            } 
            print!("mass: {}", mass);
            result.push(mass);
            i += 1;
        }
    } else {
        loop {
            let mass = if i == 0 { 0.0 } else { res_seq.aa_residual_composition[&aalist[aalist_startindex]].mass() };
            let mass_remain = max_mass - mass;
            if mass_remain < 0.0 {
                break;
            }
            for r in generate_seqmz_candidates(res_seq, mass_remain, aalist_startindex + 1) {
                result.push(mass + r);
            }
            i += 1;
        }
    }

    result
}