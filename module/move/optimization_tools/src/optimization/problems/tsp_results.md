# Traveling Salesman Problem

Traveling salesman problem is initialized as graph with nodes representing cities and edges represent connection between cities, with edge weight as distance between cities. Every city is connected with every other city. Problem is symmetrical, so distance between cities doesn't depend on which node is starting node and which is target node.

For parameters:
 - temperature decrease coefficient : 0.999,
 - temperature increase value : 1.0,
 - ga_max_stale_iterations : 31,
 - sa_mutations_per_generation_limit : 310,
 - reset limit : 1_000,
 - elite selection rate : 0.38,
 - mutation rate : 0.22,
 - crossover_rate : 0.4,
 - population percent included in next generation : 1.0,
 - tournament_selection_pressure : 0.85,
 - tournament_selection_size : 2,


| Number of nodes in graph | Population size   | Dynasties limit   | Execution time |
| ------------------------ | ----------------- | ----------------- | -------------- |
| 4                        | 78                | 80                | 112.7s         |

For parameters ( pure simulated annealing ):
 - temperature decrease coefficient : 0.999,
 - temperature increase value : 1.0,
 - ga_max_stale_iterations : 38,
 - sa_mutations_per_generation_limit : 295,
 - reset limit : 1_000,
 - <em>elite selection rate</em> : 0,
 - <em>mutation rate</em> : 1,
 - <em>crossover_rate</em> : 0,
 - population percent included in next generation : 1.0,
 - tournament_selection_pressure : 0.85,
 - tournament_selection_size : 2,


| Number of nodes in graph | Population size   | Dynasties limit   | Execution time     |
| ------------------------ | ----------------- | ----------------- | ------------------ |
| 4                        | 1                 | 118               | 0.125s             |


For parameters ( pure genetic algorithm ):
 - temperature decrease coefficient : 1.0,
 - temperature increase value : 0.0,
 - ga_max_stale_iterations : 32,
 - sa_mutations_per_generation_limit : 307,
 - reset limit : 1_000,
 - elite selection rate : 0.3,
 - mutation rate : 0.2,
 - crossover_rate : 0.5,
 - population percent included in next generation : 1.0,
 - tournament_selection_pressure : 0.85,
 - tournament_selection_size : 2,


| Number of nodes in graph | Population size   | Dynasties limit   | Execution time    |
| ------------------------ | ----------------- | ----------------- | ----------------- |
| 4                        | 380               | 80                | 0.503s            |

