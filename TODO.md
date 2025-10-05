Rust panics occur on a single thread at a time so we can keep state in thread-local storage. (I think?)

# Goals

- Minimum performance impact when values are never printed
- Minimize the chance of causing a secondary panic or error

# To do


- [ ] Keep values on all threads?
- [ ] Print values for all threads?
- [ ] Record source line, variable name, a message?


# `panic-context`

<https://docs.rs/panic-context/0.1.0/panic_context/>

<https://github.com/kriomant/panic-context>

It's basically the same idea? Perhaps it could be faster, to try to make things cheap particularly for values that are never printed:

* Don't stringify until they're printed?
* Do we need an explicit context object?
