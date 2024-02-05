# Sudoku Problem

For parameters:
 - temperature decrease coefficient : 0.999,
 - temperature increase value : 1.0,
 - max_stale_iterations : 30,
 - sa_mutations_per_dynasty_limit : 300,
 - reset limit : 1_000,
 - elite selection rate : 0.25,
 - mutation rate : 0.25,
 - crossover rate : 0.5,
 - population percent included in next generation : 1.0,
 - tournament selection_pressure : 0.85,
 - tournament selection_size : 2,


| Level  | Population size   | Dynasies limit    | Execution time |
| ------ | ----------------- | ----------------- | -------------- |
| Medium | 500               | 1000              | 122.86s        |


For parameters ( pure simulated annealing ):
 - temperature decrease coefficient : 0.999,
 - temperature increase value : 1.0,
 - max_stale_iterations : 30,
 - sa_mutations_per_generation_limit : 300,
 - reset limit : 1_000,
 - <em>crossover rate</em> : 0.0,
 - <em>elite selection rate</em> : 0.0,
 - <em>mutation rate</em> : 1,
 - population percent included in next generation : 1.0,
 - tournament selection_pressure : 0.85,
 - tournament selection_size : 2,


| Level  | <em>Population size</em> | Dynasties limit | Execution time      |
| ------ | ------------------------ | --------------- | ------------------- |
| Medium | 1                        | 30000           | 7.13s               |


For parameters ( pure genetic algorithm ):
 - temperature decrease coefficient : 0.999,
 - temperature increase value : 1.0,
 - ga_max_stale iterations : 30,
 - sa_mutations per_generation_limit : 300,
 - reset limit : 1_000,
 - crossover rate : 0.5,
 - elite selection rate : 0.2,
 - mutation rate : 0.3,
 - population percent included in next generation : 1.0,
 - tournament selection_pressure : 0.85,
 - tournament selection_size : 2,


| Level  | Population size | Dynasties limit   | Execution time    |
| ------ | --------------- | ----------------- | ----------------- |
| Medium | 5000            | 100               | 110.12s           |


