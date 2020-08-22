# Chipeyte - a Chip-8 emulator

A Rust implementation of the Chip-8 emulator.

## Running the emulator

The most basic program [test_registers.c8](./programs/test_registers.c8) does not require any additional dependencies and can be run by the following command:

``` shell
$ cargo run ./programs/test_registers.c8
```

It will not render anything nor display anything, in order to do that you will have to enable the `logging` feature and provide a `RUST_LOG` level:

``` shell
$ RUST_LOG=debug cargo run --features "logging" ./programs/test_registers.c8
```

Now the program will print out all instructions it's running and end with a dump of the memory and register.

But most likely you want something more interesting...

### Enabling a UI via Sdl2

This feature requires you to have `sdl2` installed on your system.

``` shell
$ cargo run --features "sdl2_ui" ./programs/drawing02.c8
```

``` shell
$ cargo run --features "sdl2_ui" ./programs/controller.c8
```

#### Controls

Original Chip-8 keyboard had 16 buttons with the following layout:

``` asciidoc
,---------------.
| 1 | 2 | 3 | C |
|---|---|---|---|
| 4 | 5 | 6 | D |
|---|---|---|---|
| 7 | 8 | 9 | E |
|---|---|---|---|
| A | 0 | B | F |
`---------------´
```

In Chipeyte, this has been keyboard layout translated into:

``` asciidoc
,---------------.
| 6 | 7 | 8 | 9 |
|---|---|---|---|
| Y | U | I | O |
|---|---|---|---|
| H | J | K | L |
|---|---|---|---|
| N | M | , | . |
`---------------´
```

Chip-8 programs were controlled using a keyboard with 15 buttons 0-9 and A-F. Chipeyte maps the keys accordingly:

| Keyboard | Chipeyte key |
| -------- | ------------ |
| 0        | X            |
| 1        | Z            |
| 2        | S            |
| 3        | C            |
| 4        | A            |
| 5        | Space        |
| 6        | D            |
| 7        | Q            |
| 8        | W            |
| 9        | E            |
| A        | 1            |
| B        | 2            |
| C        | 3            |
| D        | 4            |
| E        | 5            |
| F        | 6            |

Basically, navigate with WASD and shoot/interact with space in most games, but ymmv.

## Implementation notes

This section contains various notes I found interesting/helpful when researching this project.

### What constitutes a CPU?

- **Arithmetic Logic Unit (ALU)** Performs arithmetic and logic operations
- **Processor registers** Supplies /operands/ to the ALU and store results of the ALU operations
- **Control unit** Orchestrates the fetching (from memory) and execution of instructions

The fundamental operation of most CPUs is to execute a sequence of stored instructions that is called a program. The instructions to be executed are kept in some kind of computer memory. Nearly all CPUs follow the *fetch*, *decode* and *execute* steps in their operation, which are collectively known as the instruction cycle.

- Instruction cycle

  1. Fetch
     Retrieve an instruction from program memory. The program counter (PC) determines the instructions location in memory. After an instruction is fetched, the PC is incremented by the length of the instruction so it contains the address of the next instruction.

  2. Decode
     The /instruction decoder/ converts the instruction into signals which control the CPU, based on the definition of the instruction set. An instruction usually contains group of bits called /fields/ which indicate the operation and information needed by the operation such as the operands.

  3. Execute
     Run the operation, often writing the result to an internal CPU register or to main memory.

## References

[How to write an emulator](http://www.emulation.org/EMUL8/HOWTO.html)

[Cowgod's Chip-8 reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)

<http://mattmik.com/files/chip8/mastering/chip8.html>

[Chip-8 manual](https://storage.googleapis.com/wzukusers/user-34724694/documents/5c83d6a5aec8eZ0cT194/CHIP-8%20Classic%20Manual%20Rev%201.3.pdf)

[Wikipedia: CPU](https://en.wikipedia.org/wiki/Central_processing_unit)

## Motivations

The purpose of this project is to learn the lower-level workings of a simple computer as well as basic systems programming in Rust.

The goal is to implement a working Chip-8 simulation not looking at any implementation examples, but solely through reading documentation about the workings of the Chip-8 language.
