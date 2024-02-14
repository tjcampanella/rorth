# rorth
Its like [Porth](https://gitlab.com/tsoding/porth), but in Rust.
Project inspired by [Tsoding's](https://github.com/rexim) Porth series on [Youtube](https://www.youtube.com/playlist?list=PLpM-Dvs8t0VbMZA7wW9aR3EtBqe2kinu4).

## Usage simulation
```bash
$ cargo run sim examples/stack.rorth
```

## Usage compilation (Right now only Apple Silicon is supported for compilation)
```bash
$ cargo run com examples/stack.rorth
$ make compile_run
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

Pushes the value 34 onto the stack.
```pascal
34
```

Pushes the value 34 onto the stack and then prints 34 to stdout.
```pascal
34 print
```

Pushes the value 9 onto the stack and then duplicates it and prints 9 twice to stdout.
```pascal
9 dup print print
```

### Arithmetics

Pushes the values 34 and 35 onto the stack and then adds them together and prints result 69 to stdout.
```pascal
34 35 + print
```

Pushes the values 34 and 35 onto the stack and then subtracts them and prints the result 420 to stdout.
```pascal
440 20 - print
```
