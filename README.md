# rorth
Its like [Porth](https://gitlab.com/tsoding/porth), but in Rust.
Project inspired by [Tsoding's](https://github.com/rexim) Porth series on [Youtube](https://www.youtube.com/playlist?list=PLpM-Dvs8t0VbMZA7wW9aR3EtBqe2kinu4).

## Usage simulation
```bash
$ cargo run sim examples/stack.rorth
```

## Usage compilation (Right now only Apple Silicon is supported for compilation)
```bash
$ cargo run com -r -s examples/stack.rorth
```

## Development Milestones

- [x] Compiled to a native instruction set (only Apple Silicon arm64 for now)
- [ ] Turing-complete
- [ ] Statically typed (the type checking is inspired by [WASM validation](https://binji.github.io/posts/webassembly-type-checking/))
- [ ] [Self-hosted](https://en.wikipedia.org/wiki/Self-hosting_(compilers)) 
- [ ] More or else close in convenience to C
- [ ] Optimized
- [ ] Crossplatform

## Language Reference

### Stack Operations

| Name    | Signature                | Description                                                                                  |
| ---     | ---                      | ---                                                                                          |
| `dup`   | `a -- a a`               | duplicate an element on top of the stack.                                                    |
| `swap`  | `a b -- b a`             | swap 2 elements on the top of the stack.                                                     |
| `drop`  | `a b -- a`               | drops the top element of the stack.                                                          |
| `print` | `a b -- a`               | print the element on top of the stack in a free form to stdout and remove it from the stack. |
| `over`  | `a b -- a b a`           | copy the element below the top of the stack                                                  |
| `rot`   | `a b c -- b c a`         | rotate the top three stack elements.                                                         |

### Comparison

| Name | Signature                                   | Description                                                  |
| ---  | ---                                         | ---                                                          |
| `=` | `[a: int] [b: int] -- [a == b : bool]`       | checks if two elements on top of the stack are equal.        |
| `>` | `[a: int] [b: int] -- [a > b : bool]`        | checks if a is greater than b.                               |
| `<` | `[a: int] [b: int] -- [a < b : bool]`        | checks if a is less than b.                                  |

### Arithmetic

| Name     | Signature                                        | Description                                      |
| ---      | ---                                              | ---                                              |
| `+`      | `[a: int] [b: int] -- [a + b: int]`              | sums up two elements on the top of the stack.    |
| `-`      | `[a: int] [b: int] -- [a - b: int]`              | subtracts two elements on the top of the stack . |
