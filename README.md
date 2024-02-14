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
x1 -- x1 x1
```pascal
9 dup 
```

x1 x2 -- x2 x1
```pascal
9 10 swap 
```

x1 x2 x3 -- x2 x3 x1
```pascal
9 10 11 rot
```

x1 -- 
```pascal
9 drop 
```


Prints the value on top of the stack and consumes it.
x1 -- 
```pascal
print
```
### Arithmetics

x1 x2 -- x3
```pascal
34 35 + 
```

x1 x2 -- x3
```pascal
440 20 - 
```
