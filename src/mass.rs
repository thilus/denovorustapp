extern crate lazy_static;

use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref STD_AA_MASS: HashMap<char, f64> = {
        let mut std_aa_mass = HashMap::new();
        std_aa_mass.insert('G', 57.02146372057);
        std_aa_mass.insert('A', 71.03711378471);
        std_aa_mass.insert('S', 87.03202840427001);
        std_aa_mass.insert('P', 97.05276384885);
        std_aa_mass.insert('V', 99.06841391299);
        std_aa_mass.insert('T', 101.04767846841);
        std_aa_mass.insert('C', 103.00918478471);
        std_aa_mass.insert('L', 113.08406397713001);
        std_aa_mass.insert('I', 113.08406397713001);
        std_aa_mass.insert('J', 113.08406397713001);
        std_aa_mass.insert('N', 114.04292744114001);
        std_aa_mass.insert('D', 115.02694302383001);
        std_aa_mass.insert('Q', 128.05857750527997);
        std_aa_mass.insert('K', 128.09496301399997);
        std_aa_mass.insert('E', 129.04259308796998);
        std_aa_mass.insert('M', 131.04048491299);
        std_aa_mass.insert('H', 137.05891185845002);
        std_aa_mass.insert('F', 147.06841391298997);
        std_aa_mass.insert('U', 150.95363508471);
        std_aa_mass.insert('R', 156.10111102359997);
        std_aa_mass.insert('Y', 163.06332853254997);
        std_aa_mass.insert('W', 186.07931294985997);
        std_aa_mass.insert('O', 237.14772686284996);
        std_aa_mass
    };
}

pub struct MassCalculator;

impl MassCalculator {
    pub fn calc_monoisotopic_mass(sequence: &str, _ion_type: Option<&str>, charge: Option<i32>, aa_mass: Option<&HashMap<char, f64>>) -> f64 {
        let aa_mass = aa_mass.unwrap_or_else(|| &*STD_AA_MASS);
        let mut mass = sequence.chars().map(|c| aa_mass.get(&c).unwrap_or_else(|| panic!("No mass data for residue: {}", c))).sum::<f64>();

        let mass_data_h = 1.00782503207; // was 1.00782503207 * 2.0 before!
        let mass_data_o = 15.99491461957;
        mass += mass_data_h * 3.0 + mass_data_o; // was mass_data_h * 2.0 + mass_data_o before!

/*         if let Some(ion_type) = ion_type {
            // Implement ion_comp here
            // let ion_comp = kwargs.get("ion_comp", &std_ion_comp)[ion_type];
            // Implement mass_data here
            // mass += ion_comp.iter().map(|(element, num)| mass_data[element] * num).sum::<f64>();
            // Placeholder code for the sake of completion
            unimplemented!();
        } */

        if let Some(charge) = charge {
            mass = (mass + 1.00782503207 * charge as f64) / charge as f64; // mass_data["H+"][0][0] * charge
        }

        mass
    }
}


