# rorth
Its like [Porth](https://gitlab.com/tsoding/porth), but in Rust

## Usage
```bash
$ cargo run examples/stack.rorth
$ make compile_run
```

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

### Arithmatics

Pushes the values 34 and 35 onto the stack and then adds them together and prints 69 to stdout.
```pascal
34 35 + dump
```
