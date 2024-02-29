# Traveling Salesman Problem

## For hybrid:

 - execution time: 0.173s

 - number of nodes: 4

 - parameters: 

```
┌─────────────┬────────┬─────────┬────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min     │ max    │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.1471 │ 1.00    │ 0.00   │ 0.65        │ 0.04     │ 10      │ 0.9999 │
│ decrease    │        │         │        │             │          │         │        │
│ coefficient │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 112    │ 200.00  │ 10.00  │ 91.21       │ 5.70     │ 10      │ 103    │
│ mutations   │        │         │        │             │          │         │        │
│ per         │        │         │        │             │          │         │        │
│ dynasty     │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 0.83   │ 1.00    │ 0.00   │ 3.91        │ 0.24     │ 10      │ 0.08   │
│ rate        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.16   │ 1.00    │ 0.00   │ 2.56        │ 0.16     │ 10      │ 0.68   │
│ rate        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ 0.01   │ -       │ -      │ -           │ -        │ -       │ 0.23   │
│ rate        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 7      │ 100.00  │ 1.00   │ 148.60      │ 9.29     │ 10      │ 41     │
│ stale       │        │         │        │             │          │         │        │
│ iterations  │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 994    │ 1000.00 │ 1.00   │ 6105.97     │ 381.62   │ 10      │ 4      │
│ size        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 1315   │ 2000.00 │ 100.00 │ 1647.99     │ 103.00   │ 10      │ 997    │
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

 - execution time: 0.013s

 - number of nodes: 4

 - parameters: 

```
┌─────────────┬────────┬─────────┬────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min     │ max    │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.4533 │ 1.00    │ 0.00   │ 0.28        │ 0.02     │ 10      │ 0.9997 │
│ decrease    │        │         │        │             │          │         │        │
│ coefficient │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 54     │ 200.00  │ 10.00  │ 468.92      │ 29.31    │ 10      │ 136    │
│ mutations   │        │         │        │             │          │         │        │
│ per         │        │         │        │             │          │         │        │
│ dynasty     │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 1.00   │ 1.00    │ 1.00   │ 0.00        │ 0.00     │ 0       │ 1.00   │
│ rate        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.00   │ 0.00    │ 0.00   │ 0.00        │ 0.00     │ 1       │ 0.00   │
│ rate        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ -0.00  │ -       │ -      │ -           │ -        │ -       │ 0.00   │
│ rate        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 91     │ 100.00  │ 1.00   │ 771.46      │ 48.22    │ 10      │ 88     │
│ stale       │        │         │        │             │          │         │        │
│ iterations  │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 1      │ 1.00    │ 1.00   │ 0.00        │ 0.00     │ 0       │ 1      │
│ size        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 2849   │ 5000.00 │ 100.00 │ 29790.62    │ 1861.91  │ 10      │ 145    │
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

 - execution time: 0.213s

 - number of nodes: 4

 - parameters: 

```
┌─────────────┬────────┬─────────┬────────┬─────────────┬──────────┬─────────┬────────┐
│             │ start  │ min     │ max    │ sum of diff │ expected │ changes │ final  │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ temperature │ 0.9963 │ 1.00    │ 0.00   │ 0.01        │ 0.00     │ 10      │ 0.9999 │
│ decrease    │        │         │        │             │          │         │        │
│ coefficient │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 170    │ 200.00  │ 10.00  │ 681.91      │ 45.46    │ 10      │ 49     │
│ mutations   │        │         │        │             │          │         │        │
│ per         │        │         │        │             │          │         │        │
│ dynasty     │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ mutation    │ 0.39   │ 1.00    │ 0.10   │ 2.48        │ 0.17     │ 10      │ 0.15   │
│ rate        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ crossover   │ 0.81   │ 1.00    │ 0.10   │ 2.26        │ 0.15     │ 10      │ 0.35   │
│ rate        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ elitism     │ -0.20  │ -       │ -      │ -           │ -        │ -       │ 0.50   │
│ rate        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ max         │ 58     │ 100.00  │ 1.00   │ 335.34      │ 22.36    │ 10      │ 10     │
│ stale       │        │         │        │             │          │         │        │
│ iterations  │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ population  │ 572    │ 2000.00 │ 10.00  │ 10018.42    │ 667.89   │ 10      │ 57     │
│ size        │        │         │        │             │          │         │        │
├─────────────┼────────┼─────────┼────────┼─────────────┼──────────┼─────────┼────────┤
│ dynasties   │ 1824   │ 2000.00 │ 100.00 │ 9890.14     │ 659.34   │ 10      │ 193    │
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
│ hybrid │ 0.9999      │ 103       │ 0.08     │ 0.68      │ 0.23    │ 41         │ 4          │ 997       │ 0.173s    │
├────────┼─────────────┼───────────┼──────────┼───────────┼─────────┼────────────┼────────────┼───────────┼───────────┤
│ SA     │ 0.9997      │ 136       │ 1.00     │ 0.00      │ 0.00    │ 88         │ 1          │ 145       │ 0.013s    │
├────────┼─────────────┼───────────┼──────────┼───────────┼─────────┼────────────┼────────────┼───────────┼───────────┤
│ GA     │ 0.9999      │ 49        │ 0.15     │ 0.35      │ 0.50    │ 10         │ 57         │ 193       │ 0.213s    │
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
