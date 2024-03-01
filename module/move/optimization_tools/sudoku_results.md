# Sudoku Problem

## For hybrid:

 - execution time: 0.311s

 - level: Easy

 - parameters: 

```
┌─────────────┬────────┬────────┬─────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min    │ max     │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.8561 │ 0.00   │ 1.00    │ 0.93        │ 0.02     │ 6       │ 0.9787 │
│ decrease    │        │        │         │             │          │         │        │
│ coefficient │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 106    │ 10.00  │ 200.00  │ 318.95      │ 6.38     │ 6       │ 107    │
│ mutations   │        │        │         │             │          │         │        │
│ per         │        │        │         │             │          │         │        │
│ dynasty     │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 0.42   │ 0.00   │ 1.00    │ 2.60        │ 0.05     │ 6       │ 0.31   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.66   │ 0.00   │ 1.00    │ 3.93        │ 0.08     │ 6       │ 0.58   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ -0.09  │ -      │ -       │ -           │ -        │ -       │ 0.11   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 81     │ 1.00   │ 100.00  │ 474.07      │ 9.48     │ 6       │ 38     │
│ stale       │        │        │         │             │          │         │        │
│ iterations  │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 116    │ 1.00   │ 1000.00 │ 9216.57     │ 184.33   │ 6       │ 77     │
│ size        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 249    │ 100.00 │ 2000.00 │ 2423.08     │ 48.46    │ 6       │ 984    │
│ limit       │        │        │         │             │          │         │        │
└─────────────┴────────┴────────┴─────────┴─────────────┴──────────┴─────────┴────────┘
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
┌─────────────┬────────┬────────┬─────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min    │ max     │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.0660 │ 0.00   │ 1.00    │ 3.08        │ 0.06     │ 6       │ 0.9657 │
│ decrease    │        │        │         │             │          │         │        │
│ coefficient │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 108    │ 10.00  │ 200.00  │ 126.76      │ 2.49     │ 6       │ 102    │
│ mutations   │        │        │         │             │          │         │        │
│ per         │        │        │         │             │          │         │        │
│ dynasty     │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 1.00   │ 1.00   │ 1.00    │ 0.00        │ 0.00     │ 0       │ 1.00   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.00   │ 0.00   │ 0.00    │ 0.00        │ 0.00     │ 0       │ 0.00   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ -0.00  │ -      │ -       │ -           │ -        │ -       │ 0.00   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 47     │ 1.00   │ 100.00  │ 89.91       │ 1.76     │ 6       │ 30     │
│ stale       │        │        │         │             │          │         │        │
│ iterations  │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 1      │ 1.00   │ 1.00    │ 0.00        │ 0.00     │ 0       │ 1      │
│ size        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 4974   │ 100.00 │ 5000.00 │ 21180.01    │ 415.29   │ 6       │ 1216   │
│ limit       │        │        │         │             │          │         │        │
└─────────────┴────────┴────────┴─────────┴─────────────┴──────────┴─────────┴────────┘
```


 - `start` : initial value of parameter in starting point
 - `min` : lower bound of parameter
 - `max` : upper bound of parameter
 - `sum of diff` : sum of absolute differences between starting value and next value
 - `expected` : mathematical expectation of difference between starting value and next value
 - `changes` : number of successful changes of parameter value to more optimal
 - `final` : calculated value of parameter for which execution time was the lowest
## For GA:

 - execution time: 0.264s

 - level: Easy

 - parameters: 

```
┌─────────────┬────────┬────────┬─────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min    │ max     │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.4043 │ 0.00   │ 1.00    │ 0.51        │ 0.03     │ 10      │ 1.0000 │
│ decrease    │        │        │         │             │          │         │        │
│ coefficient │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 37     │ 10.00  │ 200.00  │ 335.93      │ 21.00    │ 10      │ 118    │
│ mutations   │        │        │         │             │          │         │        │
│ per         │        │        │         │             │          │         │        │
│ dynasty     │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 0.24   │ 0.10   │ 1.00    │ 0.56        │ 0.03     │ 10      │ 0.17   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.94   │ 0.10   │ 1.00    │ 2.44        │ 0.15     │ 10      │ 0.74   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ -0.18  │ -      │ -       │ -           │ -        │ -       │ 0.08   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 30     │ 1.00   │ 100.00  │ 25.45       │ 1.59     │ 10      │ 32     │
│ stale       │        │        │         │             │          │         │        │
│ iterations  │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 1102   │ 10.00  │ 2000.00 │ 8803.52     │ 550.22   │ 10      │ 77     │
│ size        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 439    │ 100.00 │ 2000.00 │ 3596.94     │ 224.81   │ 10      │ 1221   │
│ limit       │        │        │         │             │          │         │        │
└─────────────┴────────┴────────┴─────────┴─────────────┴──────────┴─────────┴────────┘
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
│ hybrid │ 0.9787      │ 107       │ 0.31     │ 0.58      │ 0.11    │ 38         │ 77         │ 984       │ 0.311s    │
├────────┼─────────────┼───────────┼──────────┼───────────┼─────────┼────────────┼────────────┼───────────┼───────────┤
│ SA     │ 0.9657      │ 102       │ 1.00     │ 0.00      │ 0.00    │ 30         │ 1          │ 1216      │ 0.034s    │
├────────┼─────────────┼───────────┼──────────┼───────────┼─────────┼────────────┼────────────┼───────────┼───────────┤
│ GA     │ 1.0000      │ 118       │ 0.17     │ 0.74      │ 0.08    │ 32         │ 77         │ 1221      │ 0.264s    │
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
