# Sudoku Problem

## For hybrid:

 - max number of iterations: 15

 - max no improvement iterations : 10

 - improvement threshold : 0.005s

 - calculated points: 34

 - points from cache: 13

 - level: Easy

 - execution time: 0.154s

 - parameters: 

```
┌─────────────┬────────┬────────┬─────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min    │ max     │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.8561 │ 0.00   │ 1.00    │ 0.02        │ 0.00     │ 9       │ 0.9995 │
│ decrease    │        │        │         │             │          │         │        │
│ coefficient │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 106    │ 10.00  │ 200.00  │ 295.09      │ 7.20     │ 9       │ 108    │
│ mutations   │        │        │         │             │          │         │        │
│ per         │        │        │         │             │          │         │        │
│ dynasty     │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 0.42   │ 0.00   │ 1.00    │ 1.23        │ 0.03     │ 9       │ 0.23   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.66   │ 0.00   │ 1.00    │ 1.67        │ 0.04     │ 9       │ 0.54   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ -0.09  │ -      │ -       │ -           │ -        │ -       │ 0.23   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 81     │ 1.00   │ 100.00  │ 1363.28     │ 33.25    │ 9       │ 62     │
│ stale       │        │        │         │             │          │         │        │
│ iterations  │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 116    │ 1.00   │ 1000.00 │ 9035.16     │ 220.37   │ 9       │ 3      │
│ size        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 249    │ 100.00 │ 2000.00 │ 19251.88    │ 469.56   │ 9       │ 1486   │
│ limit       │        │        │         │             │          │         │        │
└─────────────┴────────┴────────┴─────────┴─────────────┴──────────┴─────────┴────────┘
```


#### List:


 - `max number of iterations` : limit of total iterations of optimization process, termination condition
 - `max no improvement iterations` : max amount of steps performed without detected improvement, termination condition
 - `improvement threshold` : minimal value detected as improvement in objective function result
 - `calculated points` : new calculated points that were not found in cache
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

 - max number of iterations: 15

 - max no improvement iterations : 10

 - improvement threshold : 0.005s

 - calculated points: 10

 - points from cache: 12

 - level: Easy

 - execution time: 0.019s

 - parameters: 

```
┌─────────────┬────────┬────────┬─────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min    │ max     │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.8244 │ 0.00   │ 1.00    │ 0.48        │ 0.03     │ 12      │ 0.9554 │
│ decrease    │        │        │         │             │          │         │        │
│ coefficient │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 157    │ 10.00  │ 200.00  │ 261.00      │ 18.64    │ 12      │ 116    │
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
│ max         │ 67     │ 1.00   │ 100.00  │ 214.24      │ 15.30    │ 12      │ 39     │
│ stale       │        │        │         │             │          │         │        │
│ iterations  │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 1      │ 1.00   │ 1.00    │ 0.00        │ 0.00     │ 0       │ 1      │
│ size        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 3455   │ 100.00 │ 5000.00 │ 13134.94    │ 938.21   │ 12      │ 1646   │
│ limit       │        │        │         │             │          │         │        │
└─────────────┴────────┴────────┴─────────┴─────────────┴──────────┴─────────┴────────┘
```


#### List:


 - `max number of iterations` : limit of total iterations of optimization process, termination condition
 - `max no improvement iterations` : max amount of steps performed without detected improvement, termination condition
 - `improvement threshold` : minimal value detected as improvement in objective function result
 - `calculated points` : new calculated points that were not found in cache
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

 - max number of iterations: 15

 - max no improvement iterations : 10

 - improvement threshold : 0.005s

 - calculated points: 37

 - points from cache: 9

 - level: Easy

 - execution time: 0.338s

 - parameters: 

```
┌─────────────┬────────┬────────┬─────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min    │ max     │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.5685 │ 0.00   │ 1.00    │ 0.34        │ 0.01     │ 13      │ 0.9994 │
│ decrease    │        │        │         │             │          │         │        │
│ coefficient │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 23     │ 10.00  │ 200.00  │ 581.71      │ 14.54    │ 13      │ 109    │
│ mutations   │        │        │         │             │          │         │        │
│ per         │        │        │         │             │          │         │        │
│ dynasty     │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 0.12   │ 0.10   │ 1.00    │ 1.96        │ 0.05     │ 13      │ 0.31   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.21   │ 0.10   │ 1.00    │ 4.17        │ 0.10     │ 13      │ 0.62   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ 0.67   │ -      │ -       │ -           │ -        │ -       │ 0.07   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 5      │ 1.00   │ 100.00  │ 181.55      │ 4.54     │ 13      │ 34     │
│ stale       │        │        │         │             │          │         │        │
│ iterations  │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 1110   │ 10.00  │ 2000.00 │ 11558.92    │ 288.97   │ 13      │ 100    │
│ size        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 520    │ 100.00 │ 2000.00 │ 4552.06     │ 113.80   │ 13      │ 926    │
│ limit       │        │        │         │             │          │         │        │
└─────────────┴────────┴────────┴─────────┴─────────────┴──────────┴─────────┴────────┘
```


#### List:


 - `max number of iterations` : limit of total iterations of optimization process, termination condition
 - `max no improvement iterations` : max amount of steps performed without detected improvement, termination condition
 - `improvement threshold` : minimal value detected as improvement in objective function result
 - `calculated points` : new calculated points that were not found in cache
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
│ hybrid │ 0.9995      │ 108       │ 0.23     │ 0.54      │ 0.23    │ 62         │ 3          │ 1486      │ 0.154s    │
├────────┼─────────────┼───────────┼──────────┼───────────┼─────────┼────────────┼────────────┼───────────┼───────────┤
│ SA     │ 0.9554      │ 116       │ 1.00     │ 0.00      │ 0.00    │ 39         │ 1          │ 1646      │ 0.019s    │
├────────┼─────────────┼───────────┼──────────┼───────────┼─────────┼────────────┼────────────┼───────────┼───────────┤
│ GA     │ 0.9994      │ 109       │ 0.31     │ 0.62      │ 0.07    │ 34         │ 100        │ 926       │ 0.338s    │
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
