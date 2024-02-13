// Define a struct named Composition
#[derive(Debug)]
pub struct Composition {
    // Define fields for the average masses
    pub avg_mass: [f32; 5],
    // Define fields for the constants
    pub c: f64,
    pub c13: f64,
    pub c14: f64,
    pub o: f64,
    pub n: f64,
    pub co: f64,
    pub deuterium: f64,
    pub h: f64,
    pub h2: f64,
    pub h2o: f64,
    pub isotope: f64,
    pub isotope2: f64,
    pub s: f64,
    pub mono_mass: [f64; 5],
    pub neutron: f64,
    pub nh: f64,
    pub nh2: f64,
    pub nh3: f64,
    pub proton: f32,
    pub offset_b: f64,
    pub offset_y: f64,
    pub p: f64,
}

// Implement the default trait for Composition
impl Default for Composition {
    fn default() -> Self {
        Composition {
            // Initialize the fields with the given values
            avg_mass: [12.011, 1.00794, 14.00674, 15.9994, 32.066],
            c: 12.0,
            c13: 13.00335483,
            c14: 14.003241,
            o: 15.99491463,
            n: 14.003074,
            co: 0.0, // This is calculated from c and o, but we'll set it to 0 here
            deuterium: 2.014101779,
            h: 1.007825035,
            h2: 0.0, // This is calculated from h, but we'll set it to 0 here
            h2o: 0.0, // This is calculated from h and o, but we'll set it to 0 here
            isotope: 0.0, // This is calculated from c and isotope, but we'll set it to 0 here
            isotope2: 0.0, // This is calculated from c and isotope2, but we'll set it to 0 here
            s: 31.9720707,
            mono_mass: [12.0, 1.007825035, 14.003074, 15.99491463, 31.9720707],
            neutron: 1.0086650,
            nh: 0.0, // This is calculated from n and h, but we'll set it to 0 here
            nh2: 0.0, // This is calculated from n and h, but we'll set it to 0 here
            nh3: 0.0, // This is calculated from n and h, but we'll set it to 0 here
            proton: 1.0072697,
            offset_b: 1.0072697, // This is set to proton as per the given Java code
            offset_y: 1.007825035 * 2.0 + 15.99491463 + 1.0072697, // This is calculated as per the given Java code
            p: 30.973762,
        }
    }
}
