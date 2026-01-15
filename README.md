# Rule 30

Rule 30 is a one-dimensional cellular automaton in which cell states evolve based on the current state of their three-cell-wide neighborhood[^1].
Cell states evolve according to the following rules:

| Neighbourhood | Denary | 'Live' Neighbours | Next State |
| ------------- | ------ | ----------------- | ---------- |
| 000           | 0      | 0                 | 0          |
| 001           | 1      | 1                 | 1          |
| 010           | 2      | 1                 | 1          |
| 011           | 3      | 2                 | 1          |
| 100           | 4      | 1                 | 1          |
| 101           | 5      | 2                 | 0          |
| 110           | 6      | 2                 | 0          |
| 111           | 7      | 3                 | 0          |

Rule 30 takes its name from the pattern produced in the 'Next State' from the table above (`00011110` is the binary value for 30).

## Simplification of the Rule

This program implements a simplified version of the above ruleset, checking only if: 
1. There is only 1 live neighbour, or
2. There are 2 live neighbours and the left neighbour is dead.

## Bibliography

[^1]: Wolfram, S. 1983. 'Statistical mechanics of cellular automata'. _Rev. Mod. Phys._, 55(3). pp. 601-644.
