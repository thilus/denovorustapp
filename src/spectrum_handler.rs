use crate::spectrum::Spectrum;
use std::collections::HashSet;

pub struct SpectrumHandler;

impl SpectrumHandler {
    
    /// Coarse-grains the masses (m/z) of the input spectrum to ensure no peaks are within 0.01 Da of each other.
    pub fn coarse_grain_spectrum(spectrum: &Spectrum) -> Spectrum {
        let mut coarse_grained_spectrum = Spectrum::new();
        let mut seen_mz: HashSet<i32> = HashSet::new();
        for (mz, intensity) in spectrum.mz_vec.iter().zip(spectrum.intensity_vec.iter()) {
            let coarse_mz = (mz * 100.0).round() as i32;
            if !seen_mz.contains(&coarse_mz) {
                coarse_grained_spectrum.mz_vec.push(*mz);
                coarse_grained_spectrum.intensity_vec.push(*intensity);
                seen_mz.insert(coarse_mz);
            }
        }

        coarse_grained_spectrum
    }
}
