use std::vec::Vec;
use testlib::Peak;

/// Represents a spectrum containing mass-to-charge (m/z) values and intensities of peaks.
pub struct Spectrum {
    /// Title of the spectrum.
    pub title: String,
    /// Charge of the spectrum.
    pub charge: i32,
    /// Precursor mass.
    pub pepmass: String,
    /// Retention time in seconds.
    pub rtinseconds: String,
    /// Scan number.
    pub scan_number: i32,
    /// Peptide sequence associated with the spectrum (optional: if ground truth sequence is provided).
    pub peptide: String,
    /// Vector of mass-to-charge (m/z) values.
    pub mz_vec: Vec<f32>,
    /// Vector of intensities corresponding to the m/z values.
    pub intensity_vec: Vec<f32>,
}

impl Spectrum {
    /// Creates a new `Spectrum` instance with default values.
    pub fn new() -> Self {
        Spectrum {
            title: String::new(),
            charge: 2,
            pepmass: String::new(),
            rtinseconds: String::new(),
            scan_number: 0,
            peptide: String::new(),
            mz_vec: Vec::new(),
            intensity_vec: Vec::new(),
        }
    }
    
    /// Returns the size of the spectrum (number of peaks).
    pub fn size(&self) -> usize {
        self.mz_vec.len()
    }

    /// Generates a vector of `Peak` objects based on the m/z and intensity vectors.
    pub fn generate_peak_vector(&self) -> Vec<Peak> {
        let mut peaks: Vec<Peak> = Vec::new();
        
        for i in 0..self.mz_vec.len() {
            let mz: f32 = self.mz_vec[i];
            let intensity: f32 = self.intensity_vec[i];
            let charge: i32 = 2;
            peaks.push(Peak { mz, intensity, charge });
        }

        peaks
    }
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
