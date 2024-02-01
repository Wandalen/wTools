# Sudoku Problem

For parameters:
 - temperature decrease coefficient : 0.999,
 - temperature increase value : 1.0,
 - ga_max_stale_iterations : 20,
 - sa_mutations_per_generation_limit : 2_000,
 - reset limit : 1_000,
 - elite selection rate : 0.25,
 - mutation rate : 0.5,
 - crossover rate : 0.25,
 - population percent included in next generation : 1.0,
 - tournament selection_pressure : 0.85,
 - tournament selection_size : 2,


| Level | Population size   | Dynasies limit    | Execution time |
| ----- | ----------------- | ----------------- | -------------- |
| Medium| 1400              | 100               | 31 ms          |


For parameters ( pure simulated annealing ):
 - temperature decrease coefficient : 0.999,
 - temperature increase value : 1.0,
 - ga_max_stale_iterations : 20,
 - sa_mutations_per_generation_limit : 2_000,
 - reset limit : 1_000,
 - <em>crossover rate</em> : 0.0,
 - <em>elite selection rate</em> : 0.0,
 - <em>mutation rate</em> : 1,
 - population percent included in next generation : 1.0,
 - tournament selection_pressure : 0.85,
 - tournament selection_size : 2,


| Level  | <em>Population size</em> | Dynasties limit | Execution time             |
| ------ | ------------------------ | --------------- | -------------------------- |
| Medium | 1                        | 100000          | Optimal Solution not found |


For parameters ( pure genetic algorithm ):
 - temperature decrease coefficient : 0.999,
 - temperature increase value : 1.0,
 - ga_max_stale iterations : 20,
 - sa_mutations per_generation_limit : 2_000,
 - reset limit : 1_000,
 - crossover rate : 0.5,
 - elite selection rate : 0.2,
 - mutation rate : 0.3,
 - population percent included in next generation : 1.0,
 - tournament selection_pressure : 0.85,
 - tournament selection_size : 2,


| Level  | Population size | Dynasties limit   | Execution time    |
| ------ | --------------- | ----------------- | ----------------- |
| Medium | 5000            | 100               | 210 s           |


