use std::io::{BufRead, Seek, SeekFrom};

impl IOUtil {
    fn parse_spectrum(&mut self, location: u64) -> (f64, f64, String, String, Vec<f64>, Vec<f64>) {
        self.input_handle.seek(SeekFrom::Start(location)).unwrap();
        let mut line = String::new();
        self.input_handle.read_line(&mut line).unwrap();
        assert!(line.contains("BEGIN IONS"), "Error: wrong input BEGIN IONS");
        let (precursor_mz, charge, scan, raw_sequence) = self.parse_spectrum_header();
        let (mz_list, intensity_list) = self.parse_spectrum_ions();
        (precursor_mz, charge, scan, raw_sequence, mz_list, intensity_list)
    }

    fn parse_spectrum_header(&mut self) -> (f64, f64, String, String) {
        let mut line = String::new();
        self.input_handle.read_line(&mut line).unwrap();
        assert!(line.contains("TITLE="), "Error: wrong input TITLE");
        // parse other header lines similarly
        // header PEPMASS
        // header CHARGE
        // header SCANS
        // header RTINSECONDS
        // header SEQ
        // Return parsed values
        (precursor_mz, charge, scan, raw_sequence)
    }

    fn parse_spectrum_ions(&mut self) -> (Vec<f64>, Vec<f64>) {
        let mut mz_list = Vec::new();
        let mut intensity_list = Vec::new();
        let mut line = String::new();
        loop {
            self.input_handle.read_line(&mut line).unwrap();
            if line.contains("END IONS") {
                break;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            let mz = parts[0].parse::<f64>().unwrap();
            let intensity = parts[1].parse::<f64>().unwrap();
            // TODO: MZ_MAX needs to be used!
            if mz <= self.MZ_MAX {
                mz_list.push(mz);
                intensity_list.push(intensity);
            }
            line.clear();
        }
        (mz_list, intensity_list)
    }
}
