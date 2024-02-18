# De Novo Peptide Sequencing Graph Algorithm (coded in Rust)

A brief overview of the algorithm:
1) Extract peaks from mass spectra [works!].
2) Generate spectrum graphs with nodes (and additional node filtering) [works].
3) Generate mass decompositions (up to 1000 Da) + Knapsack search [to be implemented].
4) Draw the edges of two nodes that share corresponding masses [to be implemented].
5) Perform optimal path finding [to be implemented].
6) Generate sequence candidates [to be implemented].
7) Simulate reference spectra based on candidates using a deep-learning model [to be implemented].
8) Score candidates based on experimental vs simulated mass spectra [to be implemented].
