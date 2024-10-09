# Chapter 4

[Table of contents](../README.md#table-of-contents)

## Ownership

- Rust memory management. Ownership system with a set of rules that the compiler
checks.
- Heap vs. Stack.
  - Pushing to stack is faster than allocating on the heap.
  - Accessing data in the heap is slower than accessing data on the stack.
- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.
- Rust calls a function `drop` automatically at the end of a scope.
- Returning values can transfer ownership.
- References let us use values without transferring ownership.

### Double Free Error

- If both variables `s1` and `s2` store the same ptr to memory on the heap and go out of scope, they will both try to free the same memory. This is known as a **double free** error.
- Double free errors can lead to memory corruption, and security vulnerabilities.
- To ensure memory safety, after the line `let s2 = s1;`, Rust considers `s1` as no longer valid. Therefore, Rust doesn’t need to free anything when `s1` goes out of scope.
- Instead of making a shallow copy, Rust does what is called a *move*.
- Rust will not make any deep copies automatically.
  - Exception: integers are always cloned and not moved. This is because integers have a known size, so copies are easy to make.
  - Exception: annotated types with `Copy` trait that we want to be stored on the stack. For types that implement the `Copy` trait, variables with that type do not move, but rather are trivially copied (like integers).
