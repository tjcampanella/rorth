# rorth
Its like [Porth](https://gitlab.com/tsoding/porth), but in Rust.

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
34 dump
```

### Arithmetics

Pushes the values 34 and 35 onto the stack and then adds them together and prints 69 to stdout.
```pascal
34 35 + dump
```
