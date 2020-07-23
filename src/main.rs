#[derive(Debug)]
struct CPU {
    pub counter: u32,
    pub pc: usize,
}

/// # Chip-8 Memory Map
///
/// | Hex Range   | Decimal    | Function     |
/// |-------------|------------|--------------|
/// | 0000 - 003F |   0 -   63 | Stack        |
/// | 0040 - 004C |  64 -   76 | Scratchpad   |
/// | 004D - 00FF |  76 -  255 | Unused       |
/// | 0100 - 01FF | 256 -  511 | Display      |
/// | 0200 - 0FFF | 512 - 4095 | Program area |
/// |-------------|------------|--------------|
///
/// ## Scratchpad area
///
/// 0040H - Firmware Revision (2 bytes)
/// 0048H - EEPROM Unique ID (8 bytes)
#[derive(Debug)]
struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        let mut mem = Memory {
            memory: vec![0; 4095],
        };
        mem.initialize_display_memory();
        mem
    }

    fn initialize_display_memory(&mut self) {
        // 0
        self.memory[0x0100] = 0b11110000;
        self.memory[0x0101] = 0b10010000;
        self.memory[0x0102] = 0b10010000;
        self.memory[0x0103] = 0b10010000;
        self.memory[0x0104] = 0b11110000;

        self.memory[0x0110] = 0b00100000;
        self.memory[0x0111] = 0b01100000;
        self.memory[0x0112] = 0b00100000;
        self.memory[0x0113] = 0b00100000;
        self.memory[0x0114] = 0b01110000;

        self.memory[0x0120] = 0b11110000;
        self.memory[0x0121] = 0b00010000;
        self.memory[0x0122] = 0b11110000;
        self.memory[0x0123] = 0b10000000;
        self.memory[0x0124] = 0b11110000;

        self.memory[0x0130] = 0b11110000;
        self.memory[0x0131] = 0b00010000;
        self.memory[0x0132] = 0b11110000;
        self.memory[0x0133] = 0b00010000;
        self.memory[0x0134] = 0b11110000;

        self.memory[0x0140] = 0b10010000;
        self.memory[0x0141] = 0b10010000;
        self.memory[0x0142] = 0b11110000;
        self.memory[0x0143] = 0b00010000;
        self.memory[0x0144] = 0b00010000;

        self.memory[0x0150] = 0b11110000;
        self.memory[0x0151] = 0b10000000;
        self.memory[0x0152] = 0b11110000;
        self.memory[0x0153] = 0b00010000;
        self.memory[0x0154] = 0b11110000;

        self.memory[0x0160] = 0b11110000;
        self.memory[0x0161] = 0b10000000;
        self.memory[0x0162] = 0b11110000;
        self.memory[0x0163] = 0b10010000;
        self.memory[0x0164] = 0b11110000;

        self.memory[0x0170] = 0b11110000;
        self.memory[0x0171] = 0b00010000;
        self.memory[0x0172] = 0b00100000;
        self.memory[0x0173] = 0b01000000;
        self.memory[0x0174] = 0b01000000;

        self.memory[0x0180] = 0b11110000;
        self.memory[0x0181] = 0b10010000;
        self.memory[0x0182] = 0b11110000;
        self.memory[0x0183] = 0b10010000;
        self.memory[0x0184] = 0b11110000;

        self.memory[0x0190] = 0b11110000;
        self.memory[0x0191] = 0b10010000;
        self.memory[0x0192] = 0b11110000;
        self.memory[0x0193] = 0b00010000;
        self.memory[0x0194] = 0b11110000;

        self.memory[0x01A0] = 0b11110000;
        self.memory[0x01A1] = 0b10010000;
        self.memory[0x01A2] = 0b11110000;
        self.memory[0x01A3] = 0b10010000;
        self.memory[0x01A4] = 0b10010000;

        self.memory[0x01B0] = 0b11100000;
        self.memory[0x01B1] = 0b10010000;
        self.memory[0x01B2] = 0b11100000;
        self.memory[0x01B3] = 0b10010000;
        self.memory[0x01B4] = 0b11100000;

        self.memory[0x01C0] = 0b11110000;
        self.memory[0x01C1] = 0b10000000;
        self.memory[0x01C2] = 0b10000000;
        self.memory[0x01C3] = 0b10000000;
        self.memory[0x01C4] = 0b11110000;

        self.memory[0x01D0] = 0b11100000;
        self.memory[0x01D1] = 0b10010000;
        self.memory[0x01D2] = 0b10010000;
        self.memory[0x01D3] = 0b10010000;
        self.memory[0x01D4] = 0b11100000;

        self.memory[0x01E0] = 0b11110000;
        self.memory[0x01E1] = 0b10000000;
        self.memory[0x01E2] = 0b11110000;
        self.memory[0x01E3] = 0b10000000;
        self.memory[0x01E4] = 0b11110000;

        self.memory[0x01F0] = 0b11110000;
        self.memory[0x01F1] = 0b10000000;
        self.memory[0x01F2] = 0b11110000;
        self.memory[0x01F3] = 0b10000000;
        self.memory[0x01F4] = 0b10000000;
    }
}

impl CPU {
    pub fn new(interrupt_period: u32, initial_pc: usize) -> CPU {
        CPU {
            counter: interrupt_period,
            pc: initial_pc,
        }
    }

    pub fn tick(&self, _memory: &mut Memory) {
        todo!();
    }
}

fn main() {
    let cpu = CPU::new(1024, 0x0200);
    let mut memory = Memory::new();

    cpu.tick(&mut memory);
}
