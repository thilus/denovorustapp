use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
pub struct Node {
    pub(crate) mz: f32,
    pub(crate) intensity: f32,
    pub(crate) charge: i32,
    pub(crate) index: i32,
    pub(crate) rank: i32,
}

impl Node {
    pub fn new(mz: f32, intensity: f32, charge: i32) -> Self {
        Self {
            mz,
            intensity,
            charge,
            index: -1,
            rank: -1,
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            mz: self.mz,
            intensity: self.intensity,
            charge: self.charge,
            index: self.index,
            rank: self.rank,
        }
    }

    
    pub fn compare_by_mz(a: &Node, b: &Node) -> Ordering {
        a.mz.partial_cmp(&b.mz).unwrap_or(Ordering::Equal)
    }

    fn duplicate(&self, offset: f32) -> Self {
        let mz_offset = offset / self.charge as f32;
        Self::new(self.mz + mz_offset, self.intensity, self.charge)
    }

    fn equals(&self, other: &Self) -> bool {
        (self.mz - other.mz).abs() < std::f32::EPSILON && (self.intensity - other.intensity).abs() < std::f32::EPSILON
    }

    /*fn get_mass(&self) -> f32 {
        (self.mz - crate::composition::Composition) * self.charge as f32
    }*/

    /*fn get_complement_mass(&self, parent_mass: f32) -> f32 {
        parent_mass - self.get_mass()
    }*/

    fn to_string(&self) -> String {
        format!("{} {}", self.mz, self.intensity)
    }
}
