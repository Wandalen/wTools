# Traveling Salesman Problem

## For hybrid:

 - execution time: 0.193s

 - number of nodes: 4

 - parameters: 

```
┌─────────────┬────────┬────────┬─────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min    │ max     │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.1471 │ 0.00   │ 1.00    │ 0.65        │ 0.04     │ 10      │ 0.9999 │
│ decrease    │        │        │         │             │          │         │        │
│ coefficient │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 112    │ 10.00  │ 200.00  │ 91.21       │ 5.70     │ 10      │ 103    │
│ mutations   │        │        │         │             │          │         │        │
│ per         │        │        │         │             │          │         │        │
│ dynasty     │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 0.83   │ 0.00   │ 1.00    │ 3.91        │ 0.24     │ 10      │ 0.08   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.16   │ 0.00   │ 1.00    │ 2.56        │ 0.16     │ 10      │ 0.68   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ 0.01   │ -      │ -       │ -           │ -        │ -       │ 0.23   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 7      │ 1.00   │ 100.00  │ 148.60      │ 9.29     │ 10      │ 41     │
│ stale       │        │        │         │             │          │         │        │
│ iterations  │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 994    │ 1.00   │ 1000.00 │ 6105.97     │ 381.62   │ 10      │ 4      │
│ size        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 1315   │ 100.00 │ 2000.00 │ 1647.99     │ 103.00   │ 10      │ 997    │
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

 - execution time: 0.012s

 - number of nodes: 4

 - parameters: 

```
┌─────────────┬────────┬────────┬─────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min    │ max     │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.5856 │ 0.00   │ 1.00    │ 0.22        │ 0.01     │ 10      │ 1.0000 │
│ decrease    │        │        │         │             │          │         │        │
│ coefficient │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 63     │ 10.00  │ 200.00  │ 375.07      │ 22.06    │ 10      │ 113    │
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
│ max         │ 12     │ 1.00   │ 100.00  │ 180.15      │ 10.60    │ 10      │ 44     │
│ stale       │        │        │         │             │          │         │        │
│ iterations  │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 1      │ 1.00   │ 1.00    │ 0.00        │ 0.00     │ 0       │ 1      │
│ size        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 2185   │ 100.00 │ 5000.00 │ 26327.49    │ 1548.68  │ 10      │ 118    │
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

 - execution time: 0.072s

 - number of nodes: 4

 - parameters: 

```
┌─────────────┬────────┬────────┬─────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min    │ max     │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.9963 │ 0.00   │ 1.00    │ 0.02        │ 0.00     │ 9       │ 1.0000 │
│ decrease    │        │        │         │             │          │         │        │
│ coefficient │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 170    │ 10.00  │ 200.00  │ 1133.26     │ 49.27    │ 9       │ 35     │
│ mutations   │        │        │         │             │          │         │        │
│ per         │        │        │         │             │          │         │        │
│ dynasty     │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 0.39   │ 0.10   │ 1.00    │ 2.65        │ 0.12     │ 9       │ 0.13   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.81   │ 0.10   │ 1.00    │ 3.95        │ 0.17     │ 9       │ 0.28   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ -0.20  │ -      │ -       │ -           │ -        │ -       │ 0.59   │
│ rate        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 58     │ 1.00   │ 100.00  │ 559.76      │ 24.34    │ 9       │ 30     │
│ stale       │        │        │         │             │          │         │        │
│ iterations  │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 572    │ 10.00  │ 2000.00 │ 11617.22    │ 505.10   │ 9       │ 37     │
│ size        │        │        │         │             │          │         │        │
├─────────────┼────────┼────────┼─────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 1824   │ 100.00 │ 2000.00 │ 15481.88    │ 673.13   │ 9       │ 115    │
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
│ hybrid │ 0.9999      │ 103       │ 0.08     │ 0.68      │ 0.23    │ 41         │ 4          │ 997       │ 0.193s    │
├────────┼─────────────┼───────────┼──────────┼───────────┼─────────┼────────────┼────────────┼───────────┼───────────┤
│ SA     │ 1.0000      │ 113       │ 1.00     │ 0.00      │ 0.00    │ 44         │ 1          │ 118       │ 0.012s    │
├────────┼─────────────┼───────────┼──────────┼───────────┼─────────┼────────────┼────────────┼───────────┼───────────┤
│ GA     │ 1.0000      │ 35        │ 0.13     │ 0.28      │ 0.59    │ 30         │ 37         │ 115       │ 0.072s    │
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
