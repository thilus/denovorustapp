use std::vec::Vec;

use testlib::Peak;

pub struct Spectrum {
    pub title: String,
    pub charge: i32,
    pub pepmass: String,
    pub rtinseconds: String,
    pub scan_number: i32,
    pub peptide: String, 
    //total_peak_intensity: f64,
    //maximal_peak_mass_to_consider: f64,
    //minimal_peak_mass: f64,
    //maximal_peak_mass: f64,
    pub mz_vec: Vec<f32>,
    pub intensity_vec: Vec<f32>,
}

impl Spectrum {
    pub fn new() -> Self {
        Spectrum {
            title: String::new(),
            charge: 2,
            pepmass: String::new(),
            rtinseconds: String::new(),
            scan_number: 0,
            peptide: String::new(),
            //total_peak_intensity: 0.0,
            //maximal_peak_mass_to_consider: 0.0,
            //minimal_peak_mass: 0.0,
            //maximal_peak_mass: 0.0,
            mz_vec: Vec::new(),
            intensity_vec: Vec::new(),
        }
    }
    
    pub fn size(&self) -> usize {
        self.mz_vec.len()
    }

    pub fn generate_peak_vector(&self) -> Vec<Peak> {
        // Generate the vector of Peak objects by iterating through mz and intensity vectors
        let mut peaks: Vec<Peak> = Vec::new();
        
        for i in 0..self.mz_vec.len() {
            let mz: f32 = self.mz_vec[i];
            let intensity: f32 = self.intensity_vec[i];
            let charge: i32 = 2;
            peaks.push(Peak {mz, intensity, charge});
        }

        peaks
    } 
/*     pub fn find_peak_with_max_intensity(&self, expected_mass: f64, tolerance: f64) -> f64 {
        // Implementation
        0.0
    } */
}

impl Clone for Spectrum {
    fn clone(&self) -> Self {
        Spectrum {
            title: self.title.clone(),
            charge: self.charge,
            pepmass: self.pepmass.clone(),
            rtinseconds: self.rtinseconds.clone(),
            scan_number: self.scan_number,
            peptide: self.peptide.clone(),
            mz_vec: self.mz_vec.clone(),
            intensity_vec: self.intensity_vec.clone(),
        }
    }
}


