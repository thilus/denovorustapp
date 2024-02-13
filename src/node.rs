#[derive(Debug, PartialEq)]
struct Peak {
    mz: f32,
    intensity: f32,
    charge: i32,
    index: i32,
    rank: i32,
}

impl Peak {
    fn new(mz: f32, intensity: f32, charge: i32) -> Self {
        Self {
            mz,
            intensity,
            charge,
            index: -1,
            rank: -1,
        }
    }

    fn clone(&self) -> Self {
        Self {
            mz: self.mz,
            intensity: self.intensity,
            charge: self.charge,
            index: self.index,
            rank: self.rank,
        }
    }

    fn compare_to(&self, other: &Self) -> std::cmp::Ordering {
        match self.mz.partial_cmp(&other.mz).unwrap_or(std::cmp::Ordering::Equal) {
            std::cmp::Ordering::Equal => self.intensity.partial_cmp(&other.intensity).unwrap_or(std::cmp::Ordering::Equal),
            ordering => ordering,
        }
    }

    fn duplicate(&self, offset: f32) -> Self {
        let mz_offset = offset / self.charge as f32;
        Self::new(self.mz + mz_offset, self.intensity, self.charge)
    }

    fn equals(&self, other: &Self) -> bool {
        (self.mz - other.mz).abs() < std::f32::EPSILON && (self.intensity - other.intensity).abs() < std::f32::EPSILON
    }

    fn get_mass(&self) -> f32 {
        (self.mz - crate::Composition::PROTON) * self.charge as f32
    }

    fn get_complement_mass(&self, parent_mass: f32) -> f32 {
        parent_mass - self.get_mass()
    }

    fn to_string(&self) -> String {
        format!("{} {}", self.mz, self.intensity)
    }
}
