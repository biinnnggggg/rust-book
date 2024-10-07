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
- Different from marking a variable as `mut` as shadowing effectively creates
a new variable.
- This means that we can also change the **type** of the value but reuse the
same name. For example

```rust
let spaces = "   ";
let spaces = spaces.len();
```

- Using mutation does not work here as you are **not allowed** to mutate the type of a variable.

```rust
let mut spaces = "   ";
spaces = spaces.len(); // type: usize
```

## Data Types

- Rust is statically typed. It must know the types of all variables at **compile time**.
- Rust has type inference.
- When many types are possible, we must add type annotations so the compiler has sufficient information to infer the type correctly.