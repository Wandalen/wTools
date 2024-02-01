# Traveling Salesman Problem

Traveling salesman problem is initialized as graph with nodes representing cities and edges represent connection between cities, with edge weight as distance between cities. Every city is connected with every other city. Problem is symmetrical, so distance between cities doesn't depend on which node is starting node and which is target node.

For parameters:
 - temperature decrease coefficient : 0.999,
 - temperature increase value : 1.0,
 - ga_max_stale_iterations : 20,
 - sa_mutations_per_generation_limit : 2_000,
 - reset limit : 1_000,
 - elite selection rate : 0.25,
 - mutation rate : 0.5,
 - crossover_rate : 0.25,
 - population percent included in next generation : 1.0,
 - tournament_selection_pressure : 0.85,
 - tournament_selection_size : 2,


| Number of nodes in graph | Population size   | Dynasties limit   | Execution time |
| ------------------------ | ----------------- | ----------------- | -------------- |
| 10                       | 1000              | 800               | 117 s          |

For parameters ( pure simulated annealing ):
 - temperature decrease coefficient : 0.999,
 - temperature increase value : 1.0,
 - ga_max_stale_iterations : 20,
 - sa_mutations_per_generation_limit : 2_000,
 - reset limit : 1_000,
 - <em>elite selection rate</em> : 0,
 - <em>mutation rate</em> : 1,
 - <em>crossover_rate</em> : 0,
 - population percent included in next generation : 1.0,
 - tournament_selection_pressure : 0.85,
 - tournament_selection_size : 2,


| Number of nodes in graph | Population size   | Dynasties limit   | Execution time             |
| ------------------------ | ----------------- | ----------------- | -------------------------- |
| 10                       | 1                 | 10000             | Optimal Solution not found |


For parameters ( pure genetic algorithm ):
 - temperature decrease coefficient : 1.0,
 - temperature increase value : 0.0,
 - ga_max_stale_iterations : 20,
 - sa_mutations_per_generation_limit : 2_000,
 - reset limit : 1_000,
 - elite selection rate : 0.25,
 - mutation rate : 0.5,
 - crossover_rate : 0.25,
 - population percent included in next generation : 1.0,
 - tournament_selection_pressure : 0.85,
 - tournament_selection_size : 2,


| Number of nodes in graph | Population size   | Dynasties limit   | Execution time    |
| ------------------------ | ----------------- | ----------------- | ----------------- |
| 10                       | 100               | 1000              | 11000 ms          |

