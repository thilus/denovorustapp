use std::fmt;
mod composition;
use composition::Composition;
pub struct Peak {
    pub mz: f32,
    pub intensity: f32,
    pub charge: i32,
}

impl Peak {    
    pub fn new(mz: f32, intensity: f32, charge: i32) -> Self {
        Self {
            mz,
            intensity,
            charge,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            mz: self.mz,
            intensity: self.intensity,
            charge: self.charge,
        }
    }

    pub fn compare_to(&self, other: &Self) -> std::cmp::Ordering {
        match self.mz.partial_cmp(&other.mz).unwrap_or(std::cmp::Ordering::Equal) {
            std::cmp::Ordering::Equal => self.intensity.partial_cmp(&other.intensity).unwrap_or(std::cmp::Ordering::Equal),
            ordering => ordering,
        }
    }

    pub fn duplicate(&self, offset: f32) -> Self {
        let mz_offset = offset / self.charge as f32;
        Self::new(self.mz + mz_offset, self.intensity, self.charge)
    }

    pub fn equals(&self, other: &Self) -> bool {
        (self.mz - other.mz).abs() < std::f32::EPSILON && (self.intensity - other.intensity).abs() < std::f32::EPSILON
    }

    pub fn get_mass(&self) -> f32 {
        self.mz - Composition::new("X").mass as f32 * self.charge as f32
    }

    pub fn get_complement_mass(&self, parent_mass: f32) -> f32 {
        println!("parent mass: {}", parent_mass);
        println!("self mass: {}", self.get_mass());
        parent_mass - self.get_mass()
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", self.mz, self.intensity)
    }
}

impl fmt::Debug for Peak {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Peak")
            .field("mz", &self.mz)
            .field("intensity", &self.intensity)
            .field("charge", &self.charge)

            .finish()
    }
}

impl PartialEq for Peak {
    fn eq(&self, other: &Self) -> bool {
        // Compare mz and intensity within a small epsilon
        (self.mz - other.mz).abs() < std::f32::EPSILON
            && (self.intensity - other.intensity).abs() < std::f32::EPSILON
    }
}
