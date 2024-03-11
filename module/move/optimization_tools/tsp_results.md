# Traveling Salesman Problem

## For hybrid:

 - max number of iterations: 100

 - max no improvement iterations : 10

 - improvement threshold : 0.005s

 - calculated points: 124 from 133

 - points from cache: 9 from 133

 - number of nodes: 4

 - execution time: 0.008s

 - parameters: 

```
┌─────────────┬────────┬────────┬─────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min    │ max     │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.7726 │ 0.00   │ 1.00    │ 28.88       │ 0.21     │ 74      │ 0.7349 │
│ decrease    │        │        │         │             │          │         │        │
│ coefficient │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 14     │ 10.00  │ 200.00  │ 6917.13     │ 49.76    │ 74      │ 33     │
│ mutations   │        │        │         │             │          │         │        │
│ per         │        │        │         │             │          │         │        │
│ dynasty     │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 0.00   │ 0.00   │ 1.00    │ 23.18       │ 0.17     │ 74      │ 0.13   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.63   │ 0.00   │ 1.00    │ 40.81       │ 0.29     │ 74      │ 0.86   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ 0.37   │ -      │ -       │ -           │ -        │ -       │ 0.01   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 58     │ 1.00   │ 100.00  │ 3695.03     │ 26.58    │ 74      │ 62     │
│ stale       │        │        │         │             │          │         │        │
│ iterations  │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 674    │ 1.00   │ 1000.00 │ 46923.94    │ 337.58   │ 74      │ 1      │
│ size        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 824    │ 100.00 │ 2000.00 │ 79548.00    │ 572.29   │ 74      │ 138    │
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

 - max number of iterations: 100

 - max no improvement iterations : 10

 - improvement threshold : 0.005s

 - calculated points: 16 from 26

 - points from cache: 10 from 26

 - number of nodes: 4

 - execution time: 0.007s

 - parameters: 

```
┌─────────────┬────────┬────────┬─────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min    │ max     │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.4533 │ 0.00   │ 1.00    │ 0.28        │ 0.01     │ 12      │ 0.9997 │
│ decrease    │        │        │         │             │          │         │        │
│ coefficient │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 54     │ 10.00  │ 200.00  │ 397.21      │ 20.91    │ 12      │ 120    │
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
│ max         │ 91     │ 1.00   │ 100.00  │ 920.69      │ 48.46    │ 12      │ 87     │
│ stale       │        │        │         │             │          │         │        │
│ iterations  │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 1      │ 1.00   │ 1.00    │ 0.00        │ 0.00     │ 0       │ 1      │
│ size        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 2849   │ 100.00 │ 5000.00 │ 35258.61    │ 1855.72  │ 12      │ 117    │
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

 - max number of iterations: 100

 - max no improvement iterations : 10

 - improvement threshold : 0.005s

 - calculated points: 40 from 67

 - points from cache: 27 from 67

 - number of nodes: 4

 - execution time: 0.033s

 - parameters: 

```
┌─────────────┬────────┬────────┬─────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min    │ max     │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.9963 │ 0.00   │ 1.00    │ 0.05        │ 0.00     │ 35      │ 1.0000 │
│ decrease    │        │        │         │             │          │         │        │
│ coefficient │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 170    │ 10.00  │ 200.00  │ 4452.25     │ 71.81    │ 35      │ 18     │
│ mutations   │        │        │         │             │          │         │        │
│ per         │        │        │         │             │          │         │        │
│ dynasty     │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 0.39   │ 0.10   │ 1.00    │ 7.29        │ 0.12     │ 35      │ 0.13   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.81   │ 0.10   │ 1.00    │ 10.88       │ 0.18     │ 35      │ 0.29   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ -0.20  │ -      │ -       │ -           │ -        │ -       │ 0.58   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 58     │ 1.00   │ 100.00  │ 1560.73     │ 25.17    │ 35      │ 28     │
│ stale       │        │        │         │             │          │         │        │
│ iterations  │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 572    │ 10.00  │ 2000.00 │ 44693.82    │ 720.87   │ 35      │ 19     │
│ size        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 1824   │ 100.00 │ 2000.00 │ 43273.64    │ 697.96   │ 35      │ 123    │
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
│ hybrid │ 0.7349      │ 33        │ 0.13     │ 0.86      │ 0.01    │ 62         │ 1          │ 138       │ 0.008s    │
├────────┼─────────────┼───────────┼──────────┼───────────┼─────────┼────────────┼────────────┼───────────┼───────────┤
│ SA     │ 0.9997      │ 120       │ 1.00     │ 0.00      │ 0.00    │ 87         │ 1          │ 117       │ 0.007s    │
├────────┼─────────────┼───────────┼──────────┼───────────┼─────────┼────────────┼────────────┼───────────┼───────────┤
│ GA     │ 1.0000      │ 18        │ 0.13     │ 0.29      │ 0.58    │ 28         │ 19         │ 123       │ 0.033s    │
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
