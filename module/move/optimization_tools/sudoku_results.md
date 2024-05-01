# Sudoku Problem

## For hybrid:

 - max number of iterations: 50

 - max no improvement iterations : 10

 - improvement threshold : 0.005s

 - termination reason: NoImprovement

 - iterations number: 48

 - resumed after stale: 8

 - points from cache: 43/133

 - level: Easy

 - execution time: 0.117s

 - parameters: 

```
┌─────────────┬────────┬────────┬─────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min    │ max     │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.4043 │ 0.00   │ 1.00    │ 0.10        │ 0.00     │ 41      │ 1.0000 │
│ decrease    │        │        │         │             │          │         │        │
│ coefficient │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 37     │ 10.00  │ 200.00  │ 8265.03     │ 65.08    │ 41      │ 177    │
│ mutations   │        │        │         │             │          │         │        │
│ per         │        │        │         │             │          │         │        │
│ dynasty     │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 0.16   │ 0.00   │ 1.00    │ 17.64       │ 0.14     │ 41      │ 0.41   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.93   │ 0.00   │ 1.00    │ 42.41       │ 0.33     │ 41      │ 0.10   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ -0.09  │ -      │ -       │ -           │ -        │ -       │ 0.49   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 30     │ 1.00   │ 100.00  │ 160.48      │ 1.26     │ 41      │ 31     │
│ stale       │        │        │         │             │          │         │        │
│ iterations  │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 549    │ 1.00   │ 1000.00 │ 33602.75    │ 264.59   │ 41      │ 11     │
│ size        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 439    │ 100.00 │ 2000.00 │ 58761.38    │ 462.69   │ 41      │ 1521   │
│ limit       │        │        │         │             │          │         │        │
└─────────────┴────────┴────────┴─────────┴─────────────┴──────────┴─────────┴────────┘
```


#### List:


 - `max number of iterations` : limit of total iterations of optimization process, termination condition
 - `max no improvement iterations` : max amount of steps performed without detected improvement, termination condition
 - `improvement threshold` : minimal value detected as improvement in objective function result
 - `termination reason` : the reason why optimization process was stopped
 - `iterations number` : actual number of iterations performed during optimization
 - `resumed after stale` : how many times optimization progress was resumed after some iterations without improvement
 - `points from cache` : points calculated during previous optimizations and read from cache
 - `level` : sudoku board difficulty level
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

 - max number of iterations: 50

 - max no improvement iterations : 10

 - improvement threshold : 0.005s

 - termination reason: NoImprovement

 - iterations number: 12

 - resumed after stale: 1

 - points from cache: 31/32

 - level: Easy

 - execution time: 0.026s

 - parameters: 

```
┌─────────────┬────────┬────────┬─────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min    │ max     │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.8244 │ 0.00   │ 1.00    │ 0.83        │ 0.03     │ 11      │ 0.9554 │
│ decrease    │        │        │         │             │          │         │        │
│ coefficient │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 157    │ 10.00  │ 200.00  │ 423.98      │ 17.67    │ 11      │ 116    │
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
│ max         │ 67     │ 1.00   │ 100.00  │ 265.64      │ 11.07    │ 11      │ 39     │
│ stale       │        │        │         │             │          │         │        │
│ iterations  │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 1      │ 1.00   │ 1.00    │ 0.00        │ 0.00     │ 0       │ 1      │
│ size        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 3455   │ 100.00 │ 5000.00 │ 17618.46    │ 734.10   │ 11      │ 1646   │
│ limit       │        │        │         │             │          │         │        │
└─────────────┴────────┴────────┴─────────┴─────────────┴──────────┴─────────┴────────┘
```


#### List:


 - `max number of iterations` : limit of total iterations of optimization process, termination condition
 - `max no improvement iterations` : max amount of steps performed without detected improvement, termination condition
 - `improvement threshold` : minimal value detected as improvement in objective function result
 - `termination reason` : the reason why optimization process was stopped
 - `iterations number` : actual number of iterations performed during optimization
 - `resumed after stale` : how many times optimization progress was resumed after some iterations without improvement
 - `points from cache` : points calculated during previous optimizations and read from cache
 - `level` : sudoku board difficulty level
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

 - max number of iterations: 50

 - max no improvement iterations : 10

 - improvement threshold : 0.005s

 - termination reason: NoImprovement

 - iterations number: 30

 - resumed after stale: 4

 - points from cache: 87/93

 - level: Easy

 - execution time: 0.175s

 - parameters: 

```
┌─────────────┬────────┬────────┬─────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min    │ max     │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.3698 │ 0.00   │ 1.00    │ 4.51        │ 0.05     │ 25      │ 0.9432 │
│ decrease    │        │        │         │             │          │         │        │
│ coefficient │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 108    │ 10.00  │ 200.00  │ 751.96      │ 8.74     │ 25      │ 109    │
│ mutations   │        │        │         │             │          │         │        │
│ per         │        │        │         │             │          │         │        │
│ dynasty     │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 0.22   │ 0.10   │ 1.00    │ 4.71        │ 0.05     │ 25      │ 0.32   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.16   │ 0.10   │ 1.00    │ 3.75        │ 0.04     │ 25      │ 0.54   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ 0.61   │ -      │ -       │ -           │ -        │ -       │ 0.15   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 61     │ 1.00   │ 100.00  │ 523.70      │ 6.09     │ 25      │ 35     │
│ stale       │        │        │         │             │          │         │        │
│ iterations  │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 1743   │ 10.00  │ 2000.00 │ 29942.40    │ 348.17   │ 25      │ 12     │
│ size        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 1626   │ 100.00 │ 2000.00 │ 10424.65    │ 121.22   │ 25      │ 1092   │
│ limit       │        │        │         │             │          │         │        │
└─────────────┴────────┴────────┴─────────┴─────────────┴──────────┴─────────┴────────┘
```


#### List:


 - `max number of iterations` : limit of total iterations of optimization process, termination condition
 - `max no improvement iterations` : max amount of steps performed without detected improvement, termination condition
 - `improvement threshold` : minimal value detected as improvement in objective function result
 - `termination reason` : the reason why optimization process was stopped
 - `iterations number` : actual number of iterations performed during optimization
 - `resumed after stale` : how many times optimization progress was resumed after some iterations without improvement
 - `points from cache` : points calculated during previous optimizations and read from cache
 - `level` : sudoku board difficulty level
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
│ hybrid │ 1.0000      │ 177       │ 0.41     │ 0.10      │ 0.49    │ 31         │ 11         │ 1521      │ 0.117s    │
├────────┼─────────────┼───────────┼──────────┼───────────┼─────────┼────────────┼────────────┼───────────┼───────────┤
│ SA     │ 0.9554      │ 116       │ 1.00     │ 0.00      │ 0.00    │ 39         │ 1          │ 1646      │ 0.026s    │
├────────┼─────────────┼───────────┼──────────┼───────────┼─────────┼────────────┼────────────┼───────────┼───────────┤
│ GA     │ 0.9432      │ 109       │ 0.32     │ 0.54      │ 0.15    │ 35         │ 12         │ 1092      │ 0.175s    │
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
