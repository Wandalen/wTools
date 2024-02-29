# Sudoku Problem

## For hybrid:

 - execution time: 0.379s

 - level: Easy

 - parameters: 

```
┌─────────────┬────────┬─────────┬────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min     │ max    │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.8561 │ 1.00    │ 0.00   │ 0.31        │ 0.01     │ 9       │ 0.9787 │
│ decrease    │        │         │        │             │          │         │        │
│ coefficient │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 106    │ 200.00  │ 10.00  │ 127.60      │ 5.80     │ 9       │ 107    │
│ mutations   │        │         │        │             │          │         │        │
│ per         │        │         │        │             │          │         │        │
│ dynasty     │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 0.42   │ 1.00    │ 0.00   │ 1.26        │ 0.06     │ 9       │ 0.31   │
│ rate        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.66   │ 1.00    │ 0.00   │ 1.68        │ 0.08     │ 9       │ 0.58   │
│ rate        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ -0.09  │ -       │ -      │ -           │ -        │ -       │ 0.11   │
│ rate        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 81     │ 100.00  │ 1.00   │ 285.33      │ 12.97    │ 9       │ 38     │
│ stale       │        │         │        │             │          │         │        │
│ iterations  │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 116    │ 1000.00 │ 1.00   │ 3293.07     │ 149.68   │ 9       │ 77     │
│ size        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 249    │ 2000.00 │ 100.00 │ 3707.31     │ 168.51   │ 9       │ 984    │
│ limit       │        │         │        │             │          │         │        │
└─────────────┴────────┴─────────┴────────┴─────────────┴──────────┴─────────┴────────┘
```


 - `start` : initial value of parameter in starting point
 - `min` : lower bound of parameter
 - `max` : upper bound of parameter
 - `sum of diff` : sum of absolute differences between starting value and next value
 - `expected` : mathematical expectation of difference between starting value and next value
 - `changes` : number of successful changes of parameter value to more optimal
 - `final` : calculated value of parameter for which execution time was the lowest
## For SA:

 - execution time: 0.034s

 - level: Easy

 - parameters: 

```
┌─────────────┬────────┬─────────┬────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min     │ max    │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.0660 │ 1.00    │ 0.00   │ 3.08        │ 0.06     │ 6       │ 0.9657 │
│ decrease    │        │         │        │             │          │         │        │
│ coefficient │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 108    │ 200.00  │ 10.00  │ 126.76      │ 2.49     │ 6       │ 102    │
│ mutations   │        │         │        │             │          │         │        │
│ per         │        │         │        │             │          │         │        │
│ dynasty     │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 1.00   │ 1.00    │ 1.00   │ 0.00        │ 0.00     │ 0       │ 1.00   │
│ rate        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.00   │ 0.00    │ 0.00   │ 0.00        │ 0.00     │ 0       │ 0.00   │
│ rate        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ -0.00  │ -       │ -      │ -           │ -        │ -       │ 0.00   │
│ rate        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 47     │ 100.00  │ 1.00   │ 89.91       │ 1.76     │ 6       │ 30     │
│ stale       │        │         │        │             │          │         │        │
│ iterations  │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 1      │ 1.00    │ 1.00   │ 0.00        │ 0.00     │ 0       │ 1      │
│ size        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 4974   │ 5000.00 │ 100.00 │ 21180.01    │ 415.29   │ 6       │ 1216   │
│ limit       │        │         │        │             │          │         │        │
└─────────────┴────────┴─────────┴────────┴─────────────┴──────────┴─────────┴────────┘
```


 - `start` : initial value of parameter in starting point
 - `min` : lower bound of parameter
 - `max` : upper bound of parameter
 - `sum of diff` : sum of absolute differences between starting value and next value
 - `expected` : mathematical expectation of difference between starting value and next value
 - `changes` : number of successful changes of parameter value to more optimal
 - `final` : calculated value of parameter for which execution time was the lowest
## For GA:

 - execution time: 0.337s

 - level: Easy

 - parameters: 

```
┌─────────────┬────────┬─────────┬────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min     │ max    │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.3986 │ 1.00    │ 0.00   │ 2.96        │ 0.20     │ 10      │ 0.8275 │
│ decrease    │        │         │        │             │          │         │        │
│ coefficient │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 18     │ 200.00  │ 10.00  │ 444.27      │ 29.62    │ 10      │ 82     │
│ mutations   │        │         │        │             │          │         │        │
│ per         │        │         │        │             │          │         │        │
│ dynasty     │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 0.28   │ 1.00    │ 0.10   │ 0.47        │ 0.03     │ 10      │ 0.29   │
│ rate        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.61   │ 1.00    │ 0.10   │ 0.90        │ 0.06     │ 10      │ 0.59   │
│ rate        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ 0.11   │ -       │ -      │ -           │ -        │ -       │ 0.12   │
│ rate        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 64     │ 100.00  │ 1.00   │ 217.68      │ 14.51    │ 10      │ 41     │
│ stale       │        │         │        │             │          │         │        │
│ iterations  │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 143    │ 2000.00 │ 10.00  │ 3469.32     │ 231.29   │ 10      │ 55     │
│ size        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 1423   │ 2000.00 │ 100.00 │ 3913.95     │ 260.93   │ 10      │ 1206   │
│ limit       │        │         │        │             │          │         │        │
└─────────────┴────────┴─────────┴────────┴─────────────┴──────────┴─────────┴────────┘
```


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
│ hybrid │ 0.9787      │ 107       │ 0.31     │ 0.58      │ 0.11    │ 38         │ 77         │ 984       │ 0.379s    │
├────────┼─────────────┼───────────┼──────────┼───────────┼─────────┼────────────┼────────────┼───────────┼───────────┤
│ SA     │ 0.9657      │ 102       │ 1.00     │ 0.00      │ 0.00    │ 30         │ 1          │ 1216      │ 0.034s    │
├────────┼─────────────┼───────────┼──────────┼───────────┼─────────┼────────────┼────────────┼───────────┼───────────┤
│ GA     │ 0.8275      │ 82        │ 0.29     │ 0.59      │ 0.12    │ 41         │ 55         │ 1206      │ 0.337s    │
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
