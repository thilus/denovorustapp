// tests/peak_test.rs

// Import the necessary items from your main code (peak.rs)
use testlib::Peak;

#[test]
fn test_peak_new() {
    // Test the Peak::new method
    let peak = Peak::new(100.0, 50.0, 2);
    assert_eq!(peak.mz, 100.0);
    assert_eq!(peak.intensity, 50.0);
    assert_eq!(peak.charge, 2);
}

// Add more test functions for other methods as needed
#[test]
fn test_peak_creation() {
    let peak = Peak::new(100.0, 50.0, 2);
    assert_eq!(peak.mz, 100.0);
    assert_eq!(peak.intensity, 50.0);
    assert_eq!(peak.charge, 2);
}

#[test]
fn test_peak_cloning() {
    let peak = Peak::new(100.0, 50.0, 2);
    let cloned_peak = peak.clone();
    assert_eq!(peak, cloned_peak);
}

#[test]
fn test_peak_comparison() {
    let peak1 = Peak::new(100.0, 50.0, 2);
    let peak2 = Peak::new(150.0, 60.0, 3);
    assert_eq!(peak1.compare_to(&peak2), std::cmp::Ordering::Less);
}
#[test]
fn test_peak_complement_mass_calculation() {
    let peak = Peak::new(100.0, 50.0, 2);
    println!("peak compl mass: {}", peak.get_complement_mass(1000.0));
    assert_eq!(peak.get_complement_mass(1000.0), 1000.0 - peak.get_mass());
}

#[test]
fn test_peak_to_string() {
    let peak = Peak::new(100.0, 50.0, 2);
    assert_eq!(peak.to_string(), "100 50");
}