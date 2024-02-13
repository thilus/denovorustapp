struct Node {
    mass: f32,
    intensity: f32,
}

impl Node {
    fn new(mass: f32, intensity: f32) -> Self {
        Self { mass, intensity }
    }

    fn equals(&self, other: &Node) -> bool {
        self.mass == other.mass && self.intensity == other.intensity
    }

    fn get_mass(&self) -> f32 {
        self.mass
    }

    fn get_intensity(&self) -> f32 {
        self.intensity
    }
}
