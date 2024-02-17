# De Novo Peptide Sequencing Graph Algorithm (coded in Rust)

A brief overview of the algorithm:
1) Extract peaks from mass spectra
2) Generate spectrum graphs with nodes (and additional node filtering)
3) Generate mass decompositions (up to 1000 Da) + Knapsack search
4) Draw edges two nodes share corresponding masses
5) Optimal path finding
6) Generate sequence candidate
7) Simulate reference spectra based on candidates using a deep-learning model
8) Score candidates based on experimental vs simulated mass spectra
