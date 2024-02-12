<!-- {{# generate.module_header{} #}} -->

# Module :: optimization_tools
[![experimental](https://raster.shields.io/static/v1?label=stability&message=experimental&color=orange&logoColor=eee)](https://github.com/emersion/stability-badges#experimental) [![rust-status](https://github.com/Wandalen/wTools/actions/workflows/ModuleGraphsToolsPush.yml/badge.svg)](https://github.com/Wandalen/wTools/actions/workflows/ModuleGraphsToolsPush.yml) [![docs.rs](https://img.shields.io/docsrs/optimization_tools?color=e3e8f0&logo=docs.rs)](https://docs.rs/optimization_tools) [![Open in Gitpod](https://raster.shields.io/static/v1?label=try&message=online&color=eee&logo=gitpod&logoColor=eee)](https://gitpod.io/#RUN_PATH=.,SAMPLE_FILE=sample%2Frust%2Foptimization_tools_trivial_sample%2Fsrc%2Fmain.rs,RUN_POSTFIX=--example%20optimization_tools_trivial_sample/https://github.com/Wandalen/wTools) [![discord](https://img.shields.io/discord/872391416519737405?color=eee&logo=discord&logoColor=eee&label=ask)](https://discord.gg/m3YfbXpUUY)

# Hybrid optimization using Simulated Annealing and Genetic Algorithm

## Simulated Annealing

Simulated Annealing is a probabilistic optimization algorithm inspired by the annealing process in metallurgy. It is often used to find near-optimal solutions to combinatorial optimization problems. Simulated Annealing is known for its ability to escape local optima and explore a broader solution space, making it suitable for complex optimization problems, like solving of sudoku puzzle.

Simulated Annealing starts with an initial solution and iteratively explores neighboring solutions. It accepts better solutions with certainty but occasionally accepts worse solutions with a decreasing probability over time. The algorithm maintains a temperature parameter that controls the likelihood of accepting worse solutions. As the algorithm progresses, the temperature decreases, and the acceptance of worse solutions becomes less probable.

### Resources:
   - [Video explanation](https://www.youtube.com/watch?v=21EDdFVMz8I)
   - [Wikipedia page](https://en.wikipedia.org/wiki/Simulated_annealing)

## Genetic Algorithm

A genetic algorithm (GA) is an optimization technique inspired by the principles of natural selection and genetics. It begins with a population of candidate solutions, randomly generated. Each candidate solution is evaluated using a fitness function that quantifies how well it solves the problem at hand. Solutions with higher fitness values are considered better.

To produce new population genetic operators are used: selection, crossover, mutation and elitism. 
- Mutation introduces random changes (mutations) to some individuals to maintain diversity in the population and prevent premature convergence.
- Some individuals are replaced by offspring. First parent individuals are selected from the population based on their fitness. Individuals with higher fitness have a higher chance of being selected. Than selected individuals create offspring using crossover operator, which performs recombination of their genetic material. This mimics the mating process in natural genetics.
- Some most fit individuals(elites) are cloned to new population without changes.

These operations are performed repeatedly for a certain number of generations or until a termination condition is met (e.g., reaching a satisfactory solution).

The algorithm returns the best solution found in the final population, which represents an optimal or near-optimal solution to the problem.

### Resources:
   - [Video explanation](https://www.youtube.com/watch?v=S8LdYxA5-8U)
   - [Wikipedia page](https://en.wikipedia.org/wiki/Genetic_algorithm)

## Problems

#### Sudoku Solving

Sudoku is a classic number puzzle game that involves filling a 9x9 grid with digits. The puzzle begins with some cells already filled with numbers, and the goal is to complete the grid following specific rules - every row, column and 3x3 block must contain unique digits from 1 to 9.

#### Traveling Salesman Problem

The Traveling Salesman Problem (TSP) is a classic optimization problem where the goal is, with given set of cities and the distances between each pair of cities, find the shortest possible tour that visits each city exactly once and returns to the starting city. The tour must form a closed loop, returning to the starting city.

### Results

- [Sudoku Solving](sudoku_results.md)
- [Traveling Salesman](tsp_results.md)

## Finding optimal parameters for Hybrid Optimizer using Nelder-Mead algorithm

### About Nelder-Mead algorithm:

The Nelder-Mead method, also known as the downhill simplex method, is an optimization algorithm used to find the minimum of a function in a multidimensional space. It is useful for optimizing functions that are not well-behaved or have unknown derivatives. It has several stages:

- Simplex Initialization:
Calculate initial simplex, which is a geometric shape formed by n+1 points in an n-dimensional space. These points represent candidate solutions.

- Reflection, Expansion, Contraction, and Shrinkage:
During each iteration, the method evaluates the function at each simplex point. Based on these evaluations, it performs operations like reflection, expansion, contraction, or shrinkage to adjust the simplex.

- Updating Simplex:
Depending on the evaluation results, the simplex is updated to move towards the optimum. This process continues iteratively until a termination criterion is met.

- Termination:
Termination criteria includes reaching a maximum number of iterations or achieving a desired level of accuracy. The algorithm outputs the best point found, which corresponds to the minimum of the function.

### Illustration:

<img src="https://upload.wikimedia.org/wikipedia/commons/d/de/Nelder-Mead_Himmelblau.gif" width="400" height="400" />

### More:

 - [Video explanation](https://www.youtube.com/watch?v=-GWze-wtu60)
 - [Wikipedia page](https://en.wikipedia.org/wiki/Nelder%E2%80%93Mead_method)


