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

### Scalar Types

- Represents a single value
- Rust has four primary scalar types: integers, floating-point numbers, booleans, and characters.

#### Integer Types

- `u32`: unsigned integer, with 32 bits of space.
- `i32`: signed integer, stored using **two's complement** representation.
  
| Length | Signed | Unsigned |
| ------ | ------ | -------- |
| 8-bit  | `i8`   | `u8`     |
| 16-bit | `i16`  | `u16`    |
| 32-bit | `i32`  | `u32`    |
| 64-bit | `i64`  | `u64`    |
| 128-bit| `i128` | `u128`   |
| arch   | `isize`| `usize`  |

- `isize` and `usize` types depend on the computer architecture.
- Number literals that can be multiple numeric types allow a type suffix, such as `57u8`.
- `i32` is the default integer type.
- Integer overflow:
  - compiling in debug mode will check for integer overflow that cause your program to *panic* at runtime.
  - compiling in release mode with `--release` flag does not include check for overflow that cause panics. It handles overflow by performing two'scomplement wrpping.
  - to handle overflow possibility explicitly, refer to the standard library.

#### Floating-Point Types

- Two types: `f32` and `f64`. Both are signed.
- According to the iEEE-754 standard, `f32` is a single-precision float and `f64` has double precision.
- `f64` is the default floating-point type.

#### Boolean Type

- Size: 1 byte.
- Values: `true` or `false`

#### Char Type

- Specify `char` literals with single quotes as opposed to string literals, which use double quotes.
- Size: 4 bytes.
- Represents Unicode Scalar Value: `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF` inlusive.

### Compound Types

- Group multiple scalars into one type.
- Two primitive compound types: tuples and arrays.
- Printing compound types: `println!("{:?}", compound);`
  
#### Tuple Type

- Fixed length.
- Each position in the tuple has a type, and these can be different.
- Use pattern matching to destructure a tuple value.
- Access tuple elements directly using the period `.` followed by the index.
- A tuple without values is called a *unit*. This value and its corresponding type are both written `()` and represent an empty value of an empty return type.

#### Array Type

- Fixed length.
- Every element has the same type.
- Write values as a comma-separated list inside square brackets.
- Useful when you want data allocated on the stack rather than the heap.
- Access array elements by indexing: `array[i];`
- Accessing an index that does not exist during runtime will cause Rust to **panic** and exit with an error message.

## Functions

- Declared by keyword `fn`.
- Rust does not care where you define the function as long as it is in a scope that can *be seen* by the caller.
- Function signatures require type declarations for each parameter.

### Statements and Expressions

- Rust is an expression-based language.
- Statements: instructions that perform an action. Does not return a value. e.g. `let y = 6;`
- Expressions: evaluate to a result. e.g. `another_function();`
- Assignments do not return a value.
- Calling functions and macros are expressions.
- New scope blocks created with curly brackets are expressions.
- Expressions do not include ending semicolons. Adding a semicolon turns it into a statement.
- Functions can return values. Return type is indicated with `fn fname() -> type {}`.
- Return early from function using `return` otherwise the function will return the last expression.

## Control Flow

- `if` expression conditions are strictly of type `boolean`.
- All branches return types must be compatible.
- `loop` expressions are used for repeating code blocks.
  - `break` keyword indicates to exit the loop.
    - loops can return a value after `break`.
    - `return` can be used inside a loop and will always exit the current function.
  - `continue` keyword indicates to start the next loop iteration.
  - Loop labels `'loop_label: loop {...}` can be specified and used with `break` or `continue`.
- Conditional loops: `while cond {...}`.
  - `for` loops can be used to loop through elements of a collection.
  - `Range`: generates numbers in sequence starting from one number and ending before another. `for num in (1..4) {...}`
