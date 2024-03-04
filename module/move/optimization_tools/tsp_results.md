# Traveling Salesman Problem

## For hybrid:

 - max number of iterations: 15

 - max no improvement iterations : 10

 - improvement threshold : 0.005s

 - calculated points: 27

 - points from cache: 0

 - number of nodes: 4

 - execution time: 0.134s

 - parameters: 

```
┌─────────────┬────────┬────────┬─────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min    │ max     │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.6708 │ 0.00   │ 1.00    │ 0.18        │ 0.01     │ 15      │ 1.0000 │
│ decrease    │        │        │         │             │          │         │        │
│ coefficient │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 77     │ 10.00  │ 200.00  │ 408.47      │ 16.34    │ 15      │ 109    │
│ mutations   │        │        │         │             │          │         │        │
│ per         │        │        │         │             │          │         │        │
│ dynasty     │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 0.68   │ 0.00   │ 1.00    │ 7.13        │ 0.29     │ 15      │ 0.13   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.20   │ 0.00   │ 1.00    │ 4.95        │ 0.20     │ 15      │ 0.75   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ 0.11   │ -      │ -       │ -           │ -        │ -       │ 0.11   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 31     │ 1.00   │ 100.00  │ 64.77       │ 2.59     │ 15      │ 33     │
│ stale       │        │        │         │             │          │         │        │
│ iterations  │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 319    │ 1.00   │ 1000.00 │ 4910.37     │ 196.41   │ 15      │ 6      │
│ size        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 1269   │ 100.00 │ 2000.00 │ 3486.88     │ 139.48   │ 15      │ 582    │
│ limit       │        │        │         │             │          │         │        │
└─────────────┴────────┴────────┴─────────┴─────────────┴──────────┴─────────┴────────┘
```


#### List:


 - `max number of iterations` : limit of total iterations of optimization process, termination condition
 - `max no improvement iterations` : max amount of steps performed without detected improvement, termination condition
 - `improvement threshold` : minimal value detected as improvement in objective function result
 - `calculated points` : new calculated points that were not found in cache
 - `points from cache` : points calculated during previous optimizations and read from cache
 - `number of nodes` : number of nodes in graph representing cities from traveling salesman problem
 - `execution time` : duration of shortest found hybrid optimization process using final parameters, measured in seconds
#### Table:
 - `start` : initial value of parameter in starting point
 - `min` : lower bound of parameter
 - `max` : upper bound of parameter
 - `sum of diff` : sum of absolute differences between starting value and next value
 - `expected` : mathematical expectation of difference between starting value and next value
 - `changes` : number of successful changes of parameter value to more optimal
 - `final` : calculated value of parameter for which execution time was the lowest
## For SA:

 - max number of iterations: 15

 - max no improvement iterations : 10

 - improvement threshold : 0.005s

 - calculated points: 32

 - points from cache: 0

 - number of nodes: 4

 - execution time: 0.006s

 - parameters: 

```
┌─────────────┬────────┬────────┬─────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min    │ max     │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.0782 │ 0.00   │ 1.00    │ 0.02        │ 0.00     │ 15      │ 0.9981 │
│ decrease    │        │        │         │             │          │         │        │
│ coefficient │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 68     │ 10.00  │ 200.00  │ 675.57      │ 27.02    │ 15      │ 87     │
│ mutations   │        │        │         │             │          │         │        │
│ per         │        │        │         │             │          │         │        │
│ dynasty     │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 1.00   │ 1.00   │ 1.00    │ 0.00        │ 0.00     │ 0       │ 1.00   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.00   │ 0.00   │ 0.00    │ 0.00        │ 0.00     │ 1       │ 0.00   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ -0.00  │ -      │ -       │ -           │ -        │ -       │ 0.00   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 12     │ 1.00   │ 100.00  │ 1086.11     │ 43.44    │ 15      │ 87     │
│ stale       │        │        │         │             │          │         │        │
│ iterations  │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 1      │ 1.00   │ 1.00    │ 0.00        │ 0.00     │ 0       │ 1      │
│ size        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 776    │ 100.00 │ 5000.00 │ 40923.94    │ 1636.96  │ 15      │ 104    │
│ limit       │        │        │         │             │          │         │        │
└─────────────┴────────┴────────┴─────────┴─────────────┴──────────┴─────────┴────────┘
```


#### List:


 - `max number of iterations` : limit of total iterations of optimization process, termination condition
 - `max no improvement iterations` : max amount of steps performed without detected improvement, termination condition
 - `improvement threshold` : minimal value detected as improvement in objective function result
 - `calculated points` : new calculated points that were not found in cache
 - `points from cache` : points calculated during previous optimizations and read from cache
 - `number of nodes` : number of nodes in graph representing cities from traveling salesman problem
 - `execution time` : duration of shortest found hybrid optimization process using final parameters, measured in seconds
#### Table:
 - `start` : initial value of parameter in starting point
 - `min` : lower bound of parameter
 - `max` : upper bound of parameter
 - `sum of diff` : sum of absolute differences between starting value and next value
 - `expected` : mathematical expectation of difference between starting value and next value
 - `changes` : number of successful changes of parameter value to more optimal
 - `final` : calculated value of parameter for which execution time was the lowest
## For GA:

 - max number of iterations: 15

 - max no improvement iterations : 10

 - improvement threshold : 0.005s

 - calculated points: 23

 - points from cache: 7

 - number of nodes: 4

 - execution time: 0.141s

 - parameters: 

```
┌─────────────┬────────┬────────┬─────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min    │ max     │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.9963 │ 0.00   │ 1.00    │ 0.02        │ 0.00     │ 15      │ 1.0000 │
│ decrease    │        │        │         │             │          │         │        │
│ coefficient │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 170    │ 10.00  │ 200.00  │ 1553.22     │ 64.72    │ 15      │ 17     │
│ mutations   │        │        │         │             │          │         │        │
│ per         │        │        │         │             │          │         │        │
│ dynasty     │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 0.39   │ 0.10   │ 1.00    │ 2.66        │ 0.11     │ 15      │ 0.14   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.81   │ 0.10   │ 1.00    │ 4.37        │ 0.18     │ 15      │ 0.29   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ -0.20  │ -      │ -       │ -           │ -        │ -       │ 0.57   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 58     │ 1.00   │ 100.00  │ 641.30      │ 26.72    │ 15      │ 2      │
│ stale       │        │        │         │             │          │         │        │
│ iterations  │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 572    │ 10.00  │ 2000.00 │ 17597.22    │ 733.22   │ 15      │ 31     │
│ size        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 1824   │ 100.00 │ 2000.00 │ 12916.00    │ 538.17   │ 15      │ 355    │
│ limit       │        │        │         │             │          │         │        │
└─────────────┴────────┴────────┴─────────┴─────────────┴──────────┴─────────┴────────┘
```


#### List:


 - `max number of iterations` : limit of total iterations of optimization process, termination condition
 - `max no improvement iterations` : max amount of steps performed without detected improvement, termination condition
 - `improvement threshold` : minimal value detected as improvement in objective function result
 - `calculated points` : new calculated points that were not found in cache
 - `points from cache` : points calculated during previous optimizations and read from cache
 - `number of nodes` : number of nodes in graph representing cities from traveling salesman problem
 - `execution time` : duration of shortest found hybrid optimization process using final parameters, measured in seconds
#### Table:
 - `start` : initial value of parameter in starting point
 - `min` : lower bound of parameter
 - `max` : upper bound of parameter
 - `sum of diff` : sum of absolute differences between starting value and next value
 - `expected` : mathematical expectation of difference between starting value and next value
 - `changes` : number of successful changes of parameter value to more optimal
 - `final` : calculated value of parameter for which execution time was the lowest
## Summary:
```
┌────────┬─────────────┬───────────┬──────────┬───────────┬─────────┬────────────┬────────────┬───────────┬───────────┐
│ mode   │ temperature │ max       │ mutation │ crossover │ elitism │ max        │ population │ dynasties │ execution │
│        │ decrease    │ mutations │ rate     │ rate      │ rate    │ stale      │ size       │ limit     │ time      │
│        │ coefficient │ per       │          │           │         │ iterations │            │           │           │
│        │             │ dynasty   │          │           │         │            │            │           │           │
├────────┼─────────────┼───────────┼──────────┼───────────┼─────────┼────────────┼────────────┼───────────┼───────────┤
│ hybrid │ 1.0000      │ 109       │ 0.13     │ 0.75      │ 0.11    │ 33         │ 6          │ 582       │ 0.134s    │
├────────┼─────────────┼───────────┼──────────┼───────────┼─────────┼────────────┼────────────┼───────────┼───────────┤
│ SA     │ 0.9981      │ 87        │ 1.00     │ 0.00      │ 0.00    │ 87         │ 1          │ 104       │ 0.006s    │
├────────┼─────────────┼───────────┼──────────┼───────────┼─────────┼────────────┼────────────┼───────────┼───────────┤
│ GA     │ 1.0000      │ 17        │ 0.14     │ 0.29      │ 0.57    │ 2          │ 31         │ 355       │ 0.141s    │
└────────┴─────────────┴───────────┴──────────┴───────────┴─────────┴────────────┴────────────┴───────────┴───────────┘
```

 - `temperature decrease coefficient` : coefficient by which temperature is lowered at each iteration of optimization process
 - `max mutations per dynasty` : max number of mutations used to produce vital individual in dynasty
 - `mutation rate` : percent of individuals in population that are created using mutation
 - `crossover rate` : percent of individuals in population that are created using crossover of selected parents
 - `elitism rate` : percent of most fit individuals in population that are cloned without changes
 - sum of mutation rate, crossover rate and elitism rate always equals 1
 - `max stale iterations` : max allowed number of iterations that do not produce individuals with better fittness
 - `population size` : number of individuals in population
 - `dynasties limit` : max number of dynasties of new solutions produced during optimization process, terminates if exceeded
 - `execution time` : time spent searching for optimal solution, measured in seconds
## To run:
 - Sudoku problem:
`cargo test -- --ignored find_opt_params_sudoku`
 - Traveling salesman problem:
`cargo test -- --ignored find_opt_params_tsp`
