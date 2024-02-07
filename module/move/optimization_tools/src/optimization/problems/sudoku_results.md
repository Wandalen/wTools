# Sudoku Problem

For parameters:
 - temperature decrease coefficient : 0.999,
 - temperature increase value : 1.0,
 - max_stale_iterations : 30,
 - sa_mutations_per_dynasty_limit : 320,
 - reset limit : 1_000,
 - elite selection rate : 0.5,
 - mutation rate : 0.15,
 - crossover rate : 0.35,
 - population percent included in next generation : 1.0,
 - tournament selection_pressure : 0.85,
 - tournament selection_size : 2,


| Level  | Population size   | Dynasies limit    | Execution time |
| ------ | ----------------- | ----------------- | -------------- |
| Easy   | 262               | 1000              | 0.9s           |


For parameters ( pure simulated annealing ):
 - temperature decrease coefficient : 0.999,
 - temperature increase value : 1.0,
 - max_stale_iterations : 32,
 - sa_mutations_per_generation_limit : 310,
 - reset limit : 1_000,
 - <em>crossover rate</em> : 0.0,
 - <em>elite selection rate</em> : 0.0,
 - <em>mutation rate</em> : 1,
 - population percent included in next generation : 1.0,
 - tournament selection_pressure : 0.85,
 - tournament selection_size : 2,


| Level  | <em>Population size</em> | Dynasties limit | Execution time      |
| ------ | ------------------------ | --------------- | ------------------- |
| Easy   | 1                        | 200             | 0.3s                |


For parameters ( pure genetic algorithm ):
 - temperature decrease coefficient : 0.999,
 - temperature increase value : 1.0,
 - ga_max_stale iterations : 30,
 - sa_mutations per_generation_limit : 310,
 - reset limit : 1_000,
 - crossover rate : 0.45,
 - elite selection rate : 0.2,
 - mutation rate : 0.35,
 - population percent included in next generation : 1.0,
 - tournament selection_pressure : 0.85,
 - tournament selection_size : 2,


| Level  | Population size | Dynasties limit   | Execution time    |
| ------ | --------------- | ----------------- | ----------------- |
| Easy   | 237             | 1080              | 0.8s              |


