use crate::spectrum::Spectrum;
use crate::node::Node;

pub struct Graph {
    pub all_nodes: Vec<Node>,
    pub filtered_nodes: Vec<Node>,
}

impl Graph {
    // Assume implementation of methods for Graph
    pub fn generate_graph_from_spectrum(spectrum: &Spectrum) -> Self {
        let peaks = spectrum.generate_peak_vector(); // Assuming exp_spectrum and get_peaks() method exist
        
        let mut all_nodes: Vec<Node> = vec![];
        let mut filtered_nodes: Vec<Node> = vec![];

        // Add the starting node to the graph.
        all_nodes.push(Node { mz: 0.0, intensity: 0.0, charge: 2, index: 0, rank: 0 });

        // Add the spectrum peaks as nodes to the graph.
        let mut index_i: i32 = 0;
        for peak in peaks.iter() {

            let node = Node {
                mz: peak.mz,
                intensity: peak.intensity,
                charge: peak.charge,
                index: index_i,
                rank: 0,
            };
            all_nodes.push(node.clone()); // Add to all_nodes
            /*if some_condition(&node) { // Adjust this condition according to your needs
                filtered_nodes.push(node); // Add to filtered_nodes
            }*/
        }

        Graph { all_nodes, filtered_nodes }
    }
}
