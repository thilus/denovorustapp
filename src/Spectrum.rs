use std::vec::Vec;

pub struct Spectrum {
    pub title: String,
    pub charge: String,
    pub pepmass: String,
    pub rtinseconds: String,
    pub scan_number: i32,
    pub peptide: String, 
    //total_peak_intensity: f64,
    //maximal_peak_mass_to_consider: f64,
    //minimal_peak_mass: f64,
    //maximal_peak_mass: f64,
    pub mz_list: Vec<f64>,
    pub intensity_list: Vec<f64>,
}

impl Spectrum {
    pub fn new() -> Self {
        Spectrum {
            title: String::new(),
            charge: String::new(),
            pepmass: String::new(),
            rtinseconds: String::new(),
            scan_number: 0,
            peptide: String::new(),
            //total_peak_intensity: 0.0,
            //maximal_peak_mass_to_consider: 0.0,
            //minimal_peak_mass: 0.0,
            //maximal_peak_mass: 0.0,
            mz_list: Vec::new(),
            intensity_list: Vec::new(),
        }
    }
    
/*     pub fn find_peak_with_max_intensity(&self, expected_mass: f64, tolerance: f64) -> f64 {
        // Implementation
        0.0
    } */
}


