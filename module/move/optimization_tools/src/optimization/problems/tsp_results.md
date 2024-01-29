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
 - population percent included in next generation : 1.0,


| Number of nodes in graph | Population number | Generation number | Execution time |
| ------------------------ | ----------------- | ----------------- | -------------- |
| 4                        | 10                | 10                | 15 ms          |
| 10                       | 100               | 1000              | 11500 ms       |
| 15                       | 200               | 1500              | 36000 ms       |
| 15                       | 100               | 3000              | 35400 ms       |

For parameters ( pure simulated annealing ):
 - temperature decrease coefficient : 0.999,
 - temperature increase value : 1.0,
 - ga_max_stale_iterations : 20,
 - sa_mutations_per_generation_limit : 2_000,
 - reset limit : 1_000,
 - elite selection rate : 1,
 - mutation rate : 1,
 - population percent included in next generation : 1.0,


| Number of nodes in graph | Population number | Generation number | Execution time             |
| ------------------------ | ----------------- | ----------------- | -------------------------- |
| 4                        | 1                 | 1000              | Optimal Solution not found |
| 10                       | 1                 | 10000             | Optimal Solution not found |
| 15                       | 1                 | 1000000           | Optimal Solution not found |


For parameters ( pure genetic algorithm ):
 - temperature decrease coefficient : 1.0,
 - temperature increase value : 0.0,
 - ga_max_stale_iterations : 20,
 - sa_mutations_per_generation_limit : 2_000,
 - reset limit : 1_000,
 - elite selection rate : 0.25,
 - mutation rate : 0.5,
 - population percent included in next generation : 1.0,


| Number of nodes in graph | Population number | Generation number | Execution time    |
| ------------------------ | ----------------- | ----------------- | ----------------- |
| 4                        | 100               | 100               | 1000 ms           |
| 10                       | 100               | 1000              | 11000 ms          |
| 15                       | 100               | 5000              | 60550 ms          |