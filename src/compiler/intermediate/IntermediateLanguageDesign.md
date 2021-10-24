# Basis
The intermediate language is meant to help the compiler improve and make more general optimizations that will speed the program up faster.

## The Syntax
The syntax will consist of 8 operators that will allows for wide expressions to be casted.

1. 'add' - Adds number to cell.
2. 'sub' - Subtracts number from cell.
3. 'lshift' - Shifts pointer to left cell.
4. 'rshift' - Shifts pointer to right cell.
5. 'in' - Takes input from user.
6. 'out' - Outputs current cell.
7. 'bloop' - Begins loop.
8. 'eloop' - Ends loop;

### Example
~~~~
rshift 2
add 32
out
lshift 1
add 83
out
lshift 1
in
out
~~~~

This example results in this bf (BrainFuck) code.

~~~~
>>
++++++++++++++++++++++++++++++++
.
<
+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
.
<
,
.
~~~~