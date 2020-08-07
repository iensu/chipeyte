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

| Code   | Operation          | Description                                                                                                                                                              |
| ------ | ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `0nnn` | SYS NNN            | Ignored.                                                                                                                                                                 |
| `00E0` | CLS                | Clear display.                                                                                                                                                           |
| `00EE` | RET                | Return from subroutine.                                                                                                                                                  |
| `1nnn` | JP NNN             | Jump to address NNN.                                                                                                                                                     |
| `2nnn` | CALL NNN           | Call subroutine at address NNN.                                                                                                                                          |
| `3xnn` | SE Vx, NN          | Skip next instruction if Vx == NN.                                                                                                                                       |
| `4xnn` | SNE Vx, NN         | Skip next instruction if Vx != NN.                                                                                                                                       |
| `5xy0` | SE Vx, Vy          | Skip next instruction if Vx == Vy.                                                                                                                                       |
| `6xnn` | LD Vx, NN          | Set Vx to NN.                                                                                                                                                            |
| `7xnn` | ADD Vx, NN         | Add NN to Vx **without** setting carry.                                                                                                                                  |
| `8xy0` | LD Vx, Vy          | Store value of Vy in Vx.                                                                                                                                                 |
| `8xy1` | OR Vx, Vy          | Bitwise OR on Vx and Vy, stores the result in Vx.                                                                                                                        |
| `8xy2` | AND Vx, Vy         | Bitwise AND on Vx and Vy, stores the result in Vx.                                                                                                                       |
| `8xy3` | XOR Vx, Vy         | Bitwise XOR on Vx and Vy, stores the result in Vx.                                                                                                                       |
| `8xy4` | ADD Vx, Vy         | Set Vx to Vx + Vy. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.                                                                      |
| `8xy5` | SUB Vx, Vy         | Set Vx to Vx - Vy, set VF = NOT borrow. If Vx > Vy, then VF is set to 1, else 0.                                                                                         |
| `8xy6` | SHR Vx {, Vy}      | Stores the least significant bit of VX in VF and then shifts VX to the right by 1.                                                                                       |
| `8xy7` | SUBN Vx, Vy        | Set Vx = Vy - Vx, set VF = NOT borrow. If Vy > Vx, then VF is set to 1, otherwise 0.                                                                                     |
| `8xyE` | SHL Vx {, Vy}      | Stores the most significant bit of VX in VF and then shifts VX to the left by 1.                                                                                         |
| `9xy0` | SNE Vx, Vy         | Skip next instruction if Vx != Vy.                                                                                                                                       |
| `Annn` | LD I, NNN          | Set I = NNN.                                                                                                                                                             |
| `Bnnn` | JP V0, NNN         | Jump to address NNN + V0.                                                                                                                                                |
| `Cxnn` | RND Vx, NN         | Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.                                                                        |
| `Dxyn` | DRW Vx, Vy, N      | Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.                                                                             |
| `Ex9E` | SKP Vx             | Skips the next instruction if the key stored in VX is pressed.                                                                                                           |
| `ExA1` | SKNP Vx            | Skips the next instruction if the key stored in VX isn't pressed.                                                                                                        |
| `Fx07` | LD Vx, DT          | Sets VX to the value of the delay timer.                                                                                                                                 |
| `Fx0A` | LD Vx, K           | A key press is awaited, and then stored in VX. (Blocking Operation. All instruction halted until next key event).                                                        |
| `Fx15` | LD DT, Vx          | Sets the delay timer to VX.                                                                                                                                              |
| `Fx18` | LD ST, Vx          | Sets the sound timer to VX.                                                                                                                                              |
| `Fx1E` | ADD I, Vx          | Adds VX to I. VF is not affected.                                                                                                                                        |
| `Fx29` | LD F, Vx           | Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.                                             |
| `Fx33` | LD B, Vx           | Stores the binary-coded decimal representation of VX.                                                                                                                    |
| `Fx55` | LD [I], Vx         | Stores V0 to VX (including VX) in memory starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified.             |
| `Fx65` | LD Vx, [I]         | Fills V0 to VX (including VX) with values from memory starting at address I. The offset from I is increased by 1 for each value written, but I itself is left unmodified |

#### Binary-coded decimal representation

[Wiki: Binary-coded decimal](https://en.m.wikipedia.org/wiki/Binary-coded_decimal)

The most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2. (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.)

## References

[How to write an emulator](http://www.emulation.org/EMUL8/HOWTO.html)

[Cowgod's Chip-8 reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)

<http://mattmik.com/files/chip8/mastering/chip8.html>

[Chip-8 manual](https://storage.googleapis.com/wzukusers/user-34724694/documents/5c83d6a5aec8eZ0cT194/CHIP-8%20Classic%20Manual%20Rev%201.3.pdf)

[Wikipedia: CPU](https://en.wikipedia.org/wiki/Central_processing_unit)
