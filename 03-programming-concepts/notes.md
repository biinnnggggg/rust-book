# Chapter 3

## Constants

- Constants are like immutable variables, but you must always annotate the type
of the value.  
- Constants are declared with the `const` keyword.
- Constants can be declared in any scope.
- Constants may only be set to a constant expression, not a computed result at
runtime.
- Example: `const UPPER_BOUND: u32 = 256;`

## Shadowing

- You can declare a new variable with the same name as a previous variable using
`let` keyword again. `let x = x + 1;` for example.
- The first variable is *shadowed* by the second.
- Different from marking a variable as `mut`.
