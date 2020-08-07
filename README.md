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


### Operations

| Code | Operation          | Description                                                                                                                    |
| ---- | ------------------ | ------------------------------------------------------------------------------------------------------------------------------ |
| 0nnn | SYS addr           | Ignored                                                                                                                        |
| 00E0 | CLS                | Clear display                                                                                                                  |
| 00EE | RET                | Return from subroutine                                                                                                         |
| 1nnn | JP addr            | Jump to addr                                                                                                                   |
| 2nnn | CALL addr          | Call subroutine at addr                                                                                                        |
| 3xkk | SE Vx, byte        | Skip next instruction if Vx == byte                                                                                            |
| 4xkk | SNE Vx, byte       | Skip next instruction if Vx != byte                                                                                            |
| 5xy0 | SE Vx, Vy          | Skip next instruction if Vx == Vy                                                                                              |
| 6xkk | LD Vx, byte        | Store byte in Vx                                                                                                               |
| 7xkk | ADD Vx, byte       | Add byte to Vx **without** setting carry                                                                                       |
| 8xy0 | LD Vx, Vy          | Store value of Vy in Vx                                                                                                        |
| 8xy1 | OR Vx, Vy          | Bitwise OR on the values of Vx and Vy, then stores the result in Vx.                                                           |
| 8xy2 | AND Vx, Vy         | Bitwise AND on the values of Vx and Vy, then stores the result in Vx.                                                          |
| 8xy3 | XOR Vx, Vy         | Bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.                                                 |
| 8xy4 | ADD Vx, Vy         | The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. . |
| 8xy5 | SUB Vx, Vy         | Set Vx = Vx - Vy, set VF = NOT borrow. If Vx > Vy, then VF is set to 1, else 0.                                                |
| 8xy6 | SHR Vx {, Vy}      | Set Vx = Vx SHR 1.                                                                                                             |
| 8xy7 | SUBN Vx, Vy        | Set Vx = Vy - Vx, set VF = NOT borrow. If Vy > Vx, then VF is set to 1, otherwise 0                                            |
| 8xyE | SHL Vx {, Vy}      | Set Vx = Vx SHL 1.                                                                                                             |
| 9xy0 | SNE Vx, Vy         | Skip next instruction if Vx != Vy.                                                                                             |
| Annn | LD I, addr         | Set I = nnn.                                                                                                                   |
| Bnnn | JP V0, addr        | Jump to location nnn + V0.                                                                                                     |
| Cxkk | RND Vx, byte       |                                                                                                                                |
| Dxyn | DRW Vx, Vy, nibble |                                                                                                                                |
| Ex9E | SKP Vx             |                                                                                                                                |
| ExA1 | SKNP Vx            |                                                                                                                                |
| Fx07 | LD Vx, DT          |                                                                                                                                |
| Fx0A | LD Vx, K           |                                                                                                                                |
| Fx15 | LD DT, Vx          |                                                                                                                                |
| Fx18 | LD ST, Vx          |                                                                                                                                |
| Fx1E | ADD I, Vx          |                                                                                                                                |
| Fx29 | LD F, Vx           |                                                                                                                                |
| Fx33 | LD B, Vx           |                                                                                                                                |
| Fx55 | LD [I], Vx         |                                                                                                                                |
| Fx65 | LD Vx, [I]         |

## References

[How to write an emulator](http://www.emulation.org/EMUL8/HOWTO.html)

[Cowgod's Chip-8 reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)

<http://mattmik.com/files/chip8/mastering/chip8.html>

[Chip-8 manual](https://storage.googleapis.com/wzukusers/user-34724694/documents/5c83d6a5aec8eZ0cT194/CHIP-8%20Classic%20Manual%20Rev%201.3.pdf)

[Wikipedia: CPU](https://en.wikipedia.org/wiki/Central_processing_unit)
