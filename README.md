# Chipeyte

A Rust implementation of the Chip-8 emulator.

The purpose of this project is to learn the lower-level workings of a computer as well as systems programming in Rust.

The goal is to implement a working Chip-8 simulation not looking at any implementation examples, but solely through reading documentation about the workings of the Chip-8 language.

## Implementation notes

This section contains various notes I found interesting/helpful when researching this project.

### What constitutes a CPU?

- Arithmetic Logic Unit (ALU) :: Performs arithmetic and logic operations
- Processor registers :: Supplies /operands/ to the ALU and store results of the ALU operations
- Control unit :: Orchestrates the fetching (from memory) and execution of instructions

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
