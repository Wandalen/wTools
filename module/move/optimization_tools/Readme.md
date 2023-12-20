<!-- {{# generate.module_header{} #}} -->

# Module :: optimization_tools
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleGraphsToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleGraphsToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/optimization_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/optimization_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Foptimization_tools_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20optimization_tools_trivial_sample/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

Playground for experimenting with optimization algorithms.

### Basic use-case

<!-- {{# generate.module_sample{} #}} -->

```rust
```

## Solving sudoku puzzles using Simulated Annealing algorithm

Simulated Annealing is a probabilistic optimization algorithm inspired by the annealing process in metallurgy. It is often used to find near-optimal solutions to combinatorial optimization problems. Simulated Annealing is known for its ability to escape local optima and explore a broader solution space, making it suitable for complex optimization problems, like solving of sudoku puzzle.

Simulated Annealing starts with an initial solution and iteratively explores neighboring solutions. It accepts better solutions with certainty but occasionally accepts worse solutions with a decreasing probability over time. The algorithm maintains a temperature parameter that controls the likelihood of accepting worse solutions. As the algorithm progresses, the temperature decreases, and the acceptance of worse solutions becomes less probable.

#### Sudoku

Sudoku is a classic number puzzle game that involves filling a 9x9 grid with digits. The puzzle begins with some cells already filled with numbers, and the goal is to complete the grid following specific rules - every row, column and 3x3 block must contain unique digits from 1 to 9.

### To add to your project

```bash
cargo add optimization_tools
```

### Try out from the repository

``` shell test
git clone https://github.com/Wandalen/wTools
cd wTools
cd examples/optimization_tools_trivial_sample
cargo run
```
