# Commands

## Legend
- `<...>` - argument.

## Notations
- `A1 == Columns are denoted by letters (A, B, C, ...),
    Rows are denoted by numbers (1, 2, 3, ...).`
- `R1C1 == R represents the row (Row),
C represents the column (Column)`

### Header

```shell
gspread header --url <url> --tab <name>
```

### Rows
```shell
gspread rows --url <url> --tab <name>
```

### Cell
```shell
gspread cell get --url <url> --tab <name> --cel <notation>
gspread cell set --url <url> --tab <name> --cel <notation> --val <value>
```
