
---

<div align="center">
<pre>
 _____ _                     _    ______     _      
/  __ \ |                   | |   | ___ \   (_)     
| /  \/ | ___  ___  ___  ___| |_  | |_/ /_ _ _ _ __ 
| |   | |/ _ \/ __|/ _ \/ __| __| |  __/ _` | | '__|
| \__/\ | (_) \__ \  __/\__ \ |_  | | | (_| | | |   
 \____/_|\___/|___/\___||___/\__| \_|  \__,_|_|_|   
</pre>

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg)](https://www.rust-lang.org/)
[![cargo](https://img.shields.io/badge/Cargo-1.76.0-darkred.svg
)](https://crates.io/)

</div>

Given a set of Points, the [**Closest Pair Problem**](https://en.wikipedia.org/wiki/Closest_pair_of_points_problem) asks for two Points, that are closest to another, i.e. minimal euclidean distance.

<p align="center" width="100%">
    <img src="misc/example.png" width=50%><br>
    <em>An example instance with solution.</em>
</p>

Disclaimer: Implemented algorithms work for 2-dimensional points, but could be expanded to work for n-dimensional points as well.

### Algorithms

3 algorithms were implemented in Rust:

| Algorithm        | Description   |
|------------------------------|---------------|
| brute_force                  | A simple $\mathcal{O}(n^2)$ algorithm that computes all pair distances and returns the smallest one.        |
| deterministic                | A deterministc $\mathcal{O}(n\;\log\; n)$ algorithm that uses a divide and conquer approach (for further details see [here](https://www.geeksforgeeks.org/closest-pair-of-points-using-divide-and-conquer-algorithm/)).        |
| randomized                   | A randomized algorithm that runs in $\mathcal{O}(n)$ in expectation [[Rabin 1976](https://en.wikipedia.org/wiki/Michael_O._Rabin)] . |

### Usage

    cargo run --bin closest_point <algorithm> <point1> <point2> <point3> ...

or

    cargo run --bin closest_point <algorithm> <pathToFile>

Where any point has to consist of two comma separated decimal numbers.<br>
Furthermore points in the file must be separated by new lines.

Due to the nature of the randomized algorithm, it will only work for non-negative points. This could be fixed by rearanging all points, then arrange them back after computation.

### Comparison

All algorithms are successfully able to compute the correct solution.<br>
But they differ in runtime. This can be seen in the following plot:

<p align="center" width="100%">
    <img src="misc/plot.png" width=100%><br>
    <em>Runtime comparison between the algorithms.</em>
</p>

### Discussion

Observing the results, the randomized algorithm exhibits a widely distributed runtime, with an average growth that appears linear.
Despite its expected speed advantage among the three, the deterministic algorithm proves faster for our specific instances.
This is probably caused by the grid generation of the randomized algorithm as it produces a lot of overhead.
As expected, the brute force algorithm demonstrates a significantly faster growth rate compared to both other algorithms.

Overall, for shown instances sizes, the deterministic algorithm appears to be the optimal choice. While in theory, the randomized algorithm is expected to eventually outperform, this might only hold true for unreasonably large instances.