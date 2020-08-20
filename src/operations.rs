use crate::{
    cpu::{INSTRUCTION_LENGTH, PROGRAM_START},
    types::*,
    ChipeyteError, Memory, Registers,
};
use std::time::{SystemTime, UNIX_EPOCH};

const STACK_ENTRY_LENGTH: u8 = 2;

pub trait Callable {
    fn call(
        &self,
        register: &mut Registers,
        memory: &mut Memory,
        screen: &mut dyn crate::Drawable,
        controller: &mut dyn crate::Controllable,
    ) -> Result<(), ChipeyteError>;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Ops {
    UNKNOWN(u16),
    /// SYS `nnn`
    ///
    /// Op code: `0nnn`
    ///
    /// Ignored.
    SYS(Addr),

    /// CLS
    ///
    /// Op code: `00E0`
    ///
    /// Clear display.
    CLS,

    /// RET
    ///
    /// Op code: `00EE`
    ///
    /// Return from subroutine.
    RET,

    /// JP `nnn`
    ///
    /// Op code: `1nnn`
    ///
    /// Jump to address `nnn`.
    JP(Addr),

    /// CALL `nnn`
    ///
    /// Op code: `2nnn`
    ///
    /// Call subroutine at address `nnn`.
    CALL(Addr),

    /// SE `Vx`, `nn`
    ///
    /// Op code: `3xnn`
    ///
    /// Skip next instruction if `Vx` == `nn`.
    SE(V, Byte),

    /// SNE `Vx`, `nn`
    ///
    /// Op code: `4xnn`
    ///
    /// Skip next instruction if `Vx` != `nn`.
    SNE(V, Byte),

    /// SE `Vx`, `Vy`
    ///
    /// Op code: `5xy0`
    ///
    /// Skip next instruction if `Vx` == `Vy`.
    SEV(V, V),

    /// LD `Vx`, `nn`
    ///
    /// Op code: `6xnn`
    ///
    /// Set `Vx` to `nn`.
    LD(V, Byte),

    /// ADD `Vx`, `nn`
    ///
    /// Op code: `7xnn`
    ///
    /// Add `nn` to `Vx` **without** setting carry.
    ADD(V, Byte),

    /// LD `Vx`, `Vy`
    ///
    /// Op code: `8xy0`
    ///
    /// Store value of `Vy` in `Vx`.
    LDV(V, V),

    /// OR `Vx`, `Vy`
    ///
    /// Op code: `8xy1`
    ///
    /// Bitwise OR on `Vx` and `Vy`, stores the result in `Vx`.
    OR(V, V),

    /// AND `Vx`, `Vy`
    ///
    /// Op code: `8xy2`
    ///
    /// Bitwise AND on `Vx` and `Vy`, stores the result in `Vx`.
    AND(V, V),

    /// XOR `Vx`, `Vy`
    ///
    /// Op code: `8xy3`
    ///
    /// Bitwise XOR on `Vx` and `Vy`, stores the result in `Vx`.
    XOR(V, V),

    /// ADD `Vx`, `Vy`
    ///
    /// Op code: `8xy4`
    ///
    /// Set `Vx` to `Vx` + `Vy`.
    ADDV(V, V),

    /// SUB `Vx`, `Vy`
    ///
    /// Op code: `8xy5`
    ///
    /// Set `Vx` to `Vx` - `Vy`, set Vf = NOT borrow.
    SUB(V, V),

    /// SHR `Vx` {, `Vy`}
    ///
    /// Op code: `8xy6`
    ///
    /// Stores the least significant bit of `Vx` in Vf and then shifts `Vx` to the right by 1.
    SHR(V),

    /// SUBN `Vx`, `Vy`
    ///
    /// Op code: `8xy7`
    ///
    /// Set `Vx` = `Vy` - `Vx`, set Vf = NOT borrow.
    SUBN(V, V),

    /// SHL `Vx` {, `Vy`}
    ///
    /// Op code: `8xyE`
    ///
    /// Stores the most significant bit of `Vx` in Vf and then shifts `Vx` to the left by 1.
    SHL(V),

    /// SNE `Vx`, `Vy`
    ///
    /// Op code: `9xy0`
    ///
    /// Skip next instruction if `Vx` != `Vy`.
    SNEV(V, V),

    /// LD I, `nnn`
    ///
    /// Op code: `Annn`
    ///
    /// Set I = `nnn`.
    LDI(Addr),

    /// JP V0, `nnn`
    ///
    /// Op code: `Bnnn`
    ///
    /// Jump to address `nnn` + V0.
    JPV0(Addr),

    /// RND `Vx`, `nn`
    ///
    /// Op code: `Cxnn`
    ///
    /// Sets `Vx` to the result of a bitwise AND operation on a random number (0 to 255) and `nn`.
    RND(V, Byte),

    /// DRW `Vx`, `Vy`, `n`
    ///
    /// Op code: `Dxyn`
    ///
    /// Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
    ///
    /// The interpreter reads n bytes from memory, starting at the address stored in I. These bytes
    /// are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the
    /// existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set
    /// to 0. If the sprite is positioned so part of it is outside the coordinates of the display,
    /// it wraps around to the opposite side of the screen.
    DRW(V, V, Nibble),

    /// SKP `Vx`
    ///
    /// Op code: `Ex9E`
    ///
    /// Skips the next instruction if the key stored in `Vx` is pressed.
    SKP(V),

    /// SKNP `Vx`
    ///
    /// Op code: `ExA1`
    ///
    /// Skips the next instruction if the key stored in `Vx` isn't pressed.
    SKNP(V),

    /// LD `Vx`, DT
    ///
    /// Op code: `Fx07`
    ///
    /// Sets `Vx` to the value of the delay timer.
    LDVDT(V),

    /// LD `Vx` {, `k`}
    ///
    /// Op code: `Fx0A`
    ///
    /// A key press is awaited, and then stored in `Vx`.
    LDK(V),

    /// LD DT, `Vx`
    ///
    /// Op code: `Fx15`
    ///
    /// Sets the delay timer to `Vx`.
    LDDT(V),

    /// LD ST, `Vx`
    ///
    /// Op code: `Fx18`
    ///
    /// Sets the sound timer to `Vx`.
    LDST(V),

    /// ADD I, `Vx`
    ///
    /// Op code: `Fx1E`
    ///
    /// Adds `Vx` to I.
    ADDI(V),

    /// LD F, `Vx`
    ///
    /// Op code: `Fx29`
    ///
    /// Sets I to the location of the sprite for the character in `Vx`.
    LDF(V),

    /// LD B, `Vx`
    ///
    /// Op code: `Fx33`
    ///
    /// Stores the binary-coded decimal representation of `Vx`.
    ///
    /// # Binary-coded decimal representation
    ///
    /// The most significant of three digits at the address in I, the middle digit at I + 1, and
    /// the least significant digit at I + 2. (In other words, take the decimal representation
    /// of `Vx`, place the hundreds digit in memory at location in I, the tens digit at location I + 1,
    /// and the ones digit at location I + 2.).
    ///
    /// # References
    ///
    /// - http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#Fx33
    /// - https://en.m.wikipedia.org/wiki/Binary-coded_decimal
    LDB(V),

    /// LD [I], `Vx`
    ///
    /// Op code: `Fx55`
    ///
    /// Stores V0 to `Vx` (including `Vx`) in memory starting at address I.
    LDIV(V),

    /// LD `Vx`, [I]
    ///
    /// Op code: `Fx65`
    ///
    /// Fills V0 to `Vx` (including `Vx`) with values from memory starting at address I.
    LDVI(V),
}

impl Callable for Ops {
    fn call(
        &self,
        registers: &mut Registers,
        memory: &mut Memory,
        screen: &mut dyn crate::Drawable,
        controller: &mut dyn crate::Controllable,
    ) -> Result<(), ChipeyteError> {
        match &*self {
            Ops::UNKNOWN(op) => Err(ChipeyteError::OpFailed(
                *self,
                format!("Unknown operation: {:04x?}", op),
            )),

            Ops::SYS(_) => Ok(()),

            Ops::CLS => {
                screen.clear();
                Ok(())
            }

            Ops::RET => {
                registers.pc = memory.get_u16(registers.sp.into());
                registers.sp -= STACK_ENTRY_LENGTH;
                Ok(())
            }

            Ops::JP(addr) => {
                let address = *addr;

                if address > 0x0fff {
                    return Err(ChipeyteError::OpFailed(
                        *self,
                        format!("Memory address '{:04x?}' is out-of-bounds", address),
                    ));
                }

                registers.pc = address;
                Ok(())
            }

            Ops::CALL(addr) => {
                let address = *addr;

                if address > 0x0fff {
                    return Err(ChipeyteError::OpFailed(
                        *self,
                        format!("Memory address '{:04x?}' is out-of-bounds", address),
                    ));
                }

                registers.sp += STACK_ENTRY_LENGTH;
                memory.set_u16(registers.sp.into(), registers.pc);
                registers.pc = address;
                Ok(())
            }

            Ops::SE(v, byte) => {
                let value = registers.get_data_register_value(*v)?;

                if value == *byte {
                    registers.pc += INSTRUCTION_LENGTH;
                }
                Ok(())
            }

            Ops::SNE(v, byte) => {
                let value = registers.get_data_register_value(*v)?;

                if value != *byte {
                    registers.pc += INSTRUCTION_LENGTH;
                }
                Ok(())
            }

            Ops::SEV(vx, vy) => {
                let x = registers.get_data_register_value(*vx)?;
                let y = registers.get_data_register_value(*vy)?;

                if x == y {
                    registers.pc += INSTRUCTION_LENGTH;
                }
                Ok(())
            }

            Ops::LD(v, byte) => registers.set_data_register_value(*v, *byte),

            Ops::ADD(v, byte) => {
                let value = registers.get_data_register_value(*v)?;
                let result = byte.wrapping_add(value);

                registers.set_data_register_value(*v, result)
            }

            Ops::LDV(vx, vy) => {
                let y = registers.get_data_register_value(*vy)?;
                registers.set_data_register_value(*vx, y)
            }

            Ops::OR(vx, vy) => {
                let x = registers.get_data_register_value(*vx)?;
                let y = registers.get_data_register_value(*vy)?;

                registers.set_data_register_value(*vx, x | y)
            }

            Ops::AND(vx, vy) => {
                let x = registers.get_data_register_value(*vx)?;
                let y = registers.get_data_register_value(*vy)?;

                registers.set_data_register_value(*vx, x & y)
            }

            Ops::XOR(vx, vy) => {
                let x = registers.get_data_register_value(*vx)?;
                let y = registers.get_data_register_value(*vy)?;

                registers.set_data_register_value(*vx, x ^ y)
            }

            Ops::ADDV(vx, vy) => {
                let x = registers.get_data_register_value(*vx)?;
                let y = registers.get_data_register_value(*vy)?;
                let value = x as u16 + y as u16;

                registers.set_data_register_value(*vx, value as u8)?;

                if value > u8::MAX.into() {
                    registers.set_data_register_value(0x0f, 1)
                } else {
                    registers.set_data_register_value(0x0f, 0)
                }
            }

            Ops::SUB(vx, vy) => {
                let x = registers.get_data_register_value(*vx)?;
                let y = registers.get_data_register_value(*vy)?;

                registers.set_data_register_value(*vx, x.wrapping_sub(y))?;

                if x > y {
                    registers.set_data_register_value(0x0f, 1)
                } else {
                    registers.set_data_register_value(0x0f, 0)
                }
            }

            Ops::SHR(vx) => {
                let x = registers.get_data_register_value(*vx)?;

                let least_significant_bit = x & 0b0000_0001;

                registers.vf = least_significant_bit;

                registers.set_data_register_value(*vx, x >> 1)
            }

            Ops::SUBN(vx, vy) => {
                let x = registers.get_data_register_value(*vx)?;
                let y = registers.get_data_register_value(*vy)?;

                registers.set_data_register_value(*vx, y.wrapping_sub(x))?;

                if y > x {
                    registers.set_data_register_value(0x0f, 1)
                } else {
                    registers.set_data_register_value(0x0f, 0)
                }
            }

            Ops::SHL(vx) => {
                let x = registers.get_data_register_value(*vx)?;

                let most_significant_bit = x & 0b1000_0000;

                registers.vf = most_significant_bit;

                registers.set_data_register_value(*vx, x << 1)
            }

            Ops::SNEV(vx, vy) => {
                let x = registers.get_data_register_value(*vx)?;
                let y = registers.get_data_register_value(*vy)?;

                if x != y {
                    registers.pc += INSTRUCTION_LENGTH;
                }
                Ok(())
            }

            Ops::LDI(addr) => {
                let address = *addr;

                if address > 0x0fff {
                    return Err(ChipeyteError::OpFailed(
                        *self,
                        format!("Memory address '{:04x?}' is out-of-bounds", address),
                    ));
                }

                registers.i = address & 0x0fff;
                Ok(())
            }

            Ops::JPV0(value) => {
                let result = *value + registers.v0 as u16;

                if result < PROGRAM_START || result > 0x0fff {
                    return Err(ChipeyteError::OpFailed(
                        *self,
                        format!(
                            "Memory address '{:04x?}' is outside of program area {:04x?}-0fff",
                            result, PROGRAM_START
                        ),
                    ));
                }

                registers.pc = result;
                Ok(())
            }

            Ops::RND(vx, value) => {
                let rand = random_number(u8::MAX.into()) as u8;

                registers.set_data_register_value(*vx, value & rand)
            }

            Ops::DRW(vx, vy, n) => {
                let base_x = registers.get_data_register_value(*vx)?;
                let base_y = registers.get_data_register_value(*vy)?;
                let sprite_addr = registers.i;

                let bytes = (0..(*n as u16))
                    .map(move |offset| {
                        let addr = (sprite_addr + offset) as usize;
                        memory.get(addr)
                    })
                    .collect::<Vec<u8>>();

                let mut has_removed_pixel = false;

                for (y_offset, byte) in bytes.iter().enumerate() {
                    let mut mask = 0b1000_0000;

                    for x_offset in 0..8 {
                        let is_one = (byte & mask) > 0;
                        if is_one {
                            let x = ((base_x as u32 + x_offset as u32) % 64) as u8;
                            let y = ((base_y as u32 + (y_offset as u32)) % 32) as u8;

                            if screen.has_pixel(x, y) {
                                screen.remove_pixel(x, y);
                                has_removed_pixel = true;
                            } else {
                                screen.add_pixel(x, y);
                            }
                        }

                        mask >>= 1;
                    }
                }

                registers.vf = if has_removed_pixel { 1 } else { 0 };

                screen.render();

                Ok(())
            }

            Ops::SKP(vx) => {
                let key = registers.get_data_register_value(*vx)?;
                if controller.is_pressed(key) {
                    registers.pc += INSTRUCTION_LENGTH;
                }
                Ok(())
            }

            Ops::SKNP(vx) => {
                let key = registers.get_data_register_value(*vx)?;
                if !controller.is_pressed(key) {
                    registers.pc += INSTRUCTION_LENGTH;
                }
                Ok(())
            }

            Ops::LDVDT(vx) => registers.set_data_register_value(*vx, registers.dt),

            Ops::LDK(vx) => match controller.get_pressed_key() {
                Some(key) => registers.set_data_register_value(*vx, key),
                None => {
                    registers.pc -= INSTRUCTION_LENGTH;
                    Ok(())
                }
            },

            Ops::LDDT(vx) => {
                registers.dt = registers.get_data_register_value(*vx)?;
                Ok(())
            }

            Ops::LDST(vx) => {
                registers.st = registers.get_data_register_value(*vx)?;
                Ok(())
            }

            Ops::ADDI(vx) => {
                let x = registers.get_data_register_value(*vx)?;

                let address = registers.i + x as u16;

                if address > 0x0FFF {
                    return Err(ChipeyteError::OpFailed(
                        *self,
                        format!(
                            "Address '{:04x?}' is outside of program area {:04x?}-0fff",
                            address, PROGRAM_START
                        ),
                    ));
                }

                registers.i = address;

                Ok(())
            }

            Ops::LDF(vx) => {
                // A digit between 0-15
                let digit = registers.get_data_register_value(*vx)?;

                registers.i = Memory::get_sprite_location_for(digit)?;
                Ok(())
            }

            Ops::LDB(vx) => {
                let number = registers.get_data_register_value(*vx)?;
                let hundreds = (number / 100) % 10;
                let tens = (number / 10) % 10;
                let ones = number % 10;

                memory.set(registers.i.into(), hundreds);
                memory.set((registers.i + 1).into(), tens);
                memory.set((registers.i + 2).into(), ones);
                Ok(())
            }

            Ops::LDIV(vx) => {
                let base_addr = registers.i as usize;

                for reg in 0..=*vx {
                    let value = registers.get_data_register_value(reg)?;
                    memory.set(base_addr + reg as usize, value);
                }
                Ok(())
            }

            Ops::LDVI(vx) => {
                let base_addr = registers.i as usize;

                for reg in 0..=*vx {
                    let value = memory.get(base_addr + reg as usize);
                    registers.set_data_register_value(reg, value)?;
                }
                Ok(())
            }
        }
    }
}

fn random_number(max_val: u32) -> u32 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();

    return nanos % max_val;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{controller::Controllable, Drawable};
    use std::collections::HashSet;

    struct MockScreen {
        pixels: HashSet<(u8, u8)>,
    }

    impl MockScreen {
        pub fn init() -> Self {
            MockScreen {
                pixels: HashSet::new(),
            }
        }
    }

    impl crate::Drawable for MockScreen {
        fn get_pixels(&self) -> HashSet<(u8, u8)> {
            self.pixels.clone()
        }
        fn clear(&mut self) {
            self.pixels.clear();
        }
        fn add_pixel(&mut self, x: u8, y: u8) {
            self.pixels.insert((x, y));
        }
        fn remove_pixel(&mut self, x: u8, y: u8) {
            self.pixels.remove(&(x, y));
        }
        fn has_pixel(&self, x: u8, y: u8) -> bool {
            self.pixels.contains(&(x, y))
        }
        fn render(&mut self) {}
        fn poll_events(&mut self) -> Option<crate::graphics::UserAction> {
            None
        }
    }

    struct MockController {
        pressed_keys: HashSet<u8>,
    }

    impl MockController {
        fn new() -> Self {
            Self {
                pressed_keys: HashSet::new(),
            }
        }
    }

    impl Controllable for MockController {
        fn press_key(&mut self, key: u8) {
            self.pressed_keys.insert(key);
        }
        fn release_key(&mut self, key: u8) {
            self.pressed_keys.remove(&key);
        }
        fn is_pressed(&self, key: u8) -> bool {
            self.pressed_keys.contains(&key)
        }
        fn get_pressed_key(&mut self) -> Option<u8> {
            None
        }
    }

    #[test]
    fn op_sys_is_ignored() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::SYS(0x0aaa)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers, Registers::new(PROGRAM_START));
        assert_eq!(memory, Memory::new());
    }

    #[test]
    fn op_cls_clears_screen() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        screen.add_pixel(0, 0);
        screen.add_pixel(0, 1);

        Ops::CLS
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert!(screen.get_pixels().is_empty());
    }

    #[test]
    fn op_ret_returns() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::CALL(0x0aaa)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.sp, 0x0002);
        assert_eq!(registers.pc, 0x0aaa);

        Ops::RET
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(memory.get_u16(0x0002), 0x0200);
        assert_eq!(registers.sp, 0x00);
    }

    #[test]
    fn op_jp_jumps_to_addr() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::JP(0x0aaa)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.pc, 0x0aaa);
    }

    #[test]
    fn op_jp_must_be_within_memory_bounds() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        if let Err(ChipeyteError::OpFailed(op, msg)) =
            Ops::JP(0xf000).call(&mut registers, &mut memory, &mut screen, &mut controller)
        {
            assert_eq!(op, Ops::JP(0xf000));
            assert!(msg.contains("out-of-bounds"));
            return;
        }

        panic!("Test failed!");
    }

    #[test]
    fn op_call_calls_addr() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::CALL(0x0aaa)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.pc, 0x0aaa);
        assert_eq!(registers.sp, 0x0002);
        assert_eq!(memory.get_u16(0x0002), 0x0200);
    }

    #[test]
    fn op_call_addr_must_be_within_memory_bounds() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        if let Err(ChipeyteError::OpFailed(op, msg)) =
            Ops::CALL(0xf000).call(&mut registers, &mut memory, &mut screen, &mut controller)
        {
            assert_eq!(op, Ops::CALL(0xf000));
            assert!(msg.contains("out-of-bounds"));
            return;
        }

        panic!("Test failed!");
    }

    #[test]
    fn op_se_vkk_increments_pc_if_v_equals_kk() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x08, 0x42)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();
        Ops::SE(0x08, 0x42)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.sp, 0);
        assert_eq!(registers.pc, PROGRAM_START + INSTRUCTION_LENGTH);
    }

    #[test]
    fn op_se_vkk_does_not_increment_pc_if_v_not_equal_to_kk() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x08, 0x84)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();
        Ops::SE(0x08, 0x42)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.sp, 0);
        assert_eq!(registers.pc, PROGRAM_START);
    }

    #[test]
    fn op_sne_vkk_does_increment_pc_if_v_equals_kk() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x08, 0x42)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();
        Ops::SNE(0x08, 0x42)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.sp, 0);
        assert_eq!(registers.pc, PROGRAM_START);
    }

    #[test]
    fn op_sne_vkk_increments_pc_if_v_not_equal_to_kk() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x08, 0x42)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();
        Ops::SNE(0x08, 0x84)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.sp, 0);
        assert_eq!(registers.pc, PROGRAM_START + INSTRUCTION_LENGTH);
    }

    #[test]
    fn op_se_vxvy_increments_pc_if_vx_equals_vy() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x08, 0x42)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();
        Ops::LD(0x0a, 0x42)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();
        Ops::SEV(0x08, 0x0a)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.sp, 0);
        assert_eq!(registers.pc, PROGRAM_START + INSTRUCTION_LENGTH);
    }

    #[test]
    fn op_se_vxvy_does_not_increment_pc_if_vx_not_equal_to_vy() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x08, 0x42)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();
        Ops::LD(0x0a, 0x84)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();
        Ops::SE(0x08, 0x0a)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.sp, 0);
        assert_eq!(registers.pc, PROGRAM_START);
    }

    #[test]
    fn op_ld_vkk_sets_register_v_to_kk() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x0a, 0x66)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.va, 0x66);
    }

    #[test]
    fn op_add_vkk_adds_kk_to_v() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0, 30)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .expect("Failed to set register");
        Ops::ADD(0, 12)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .expect("Failed to add to register");

        assert_eq!(registers.v0, 42);
    }

    #[test]
    fn op_add_vkk_adds_kk_to_v_no_carry() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0, 200)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .expect("Failed to set register");
        Ops::ADD(0, 200)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .expect("Failed to add to register");

        assert_eq!(registers.v0, 144);
        assert_eq!(registers.vf, 0); // CARRY FLAG
    }

    #[test]
    fn op_ld_vxvy_stores_vx_in_vy() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x0b, 0x09)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();
        Ops::LDV(0x0a, 0x0b)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.va, 9);
    }

    #[test]
    fn op_or_vx_vy_stores_bitwise_or_in_vx() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x0a, 0b1001_0111)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();
        Ops::LD(0x0b, 0b0110_1001)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        Ops::OR(0x0a, 0x0b)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.va, 0b1111_1111);
    }

    #[test]
    fn op_and_vx_vy_stores_bitwise_and_in_vx() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x0a, 0b1001_0111)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();
        Ops::LD(0x0b, 0b0110_1001)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        Ops::AND(0x0a, 0x0b)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.va, 0b0000_0001);
    }

    #[test]
    fn op_xor_vx_vy_stores_bitwise_xor_in_vx() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x0a, 0b1001_0111)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();
        Ops::LD(0x0b, 0b0110_1001)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        Ops::XOR(0x0a, 0x0b)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.va, 0b1111_1110);
    }

    #[test]
    fn op_add_vx_vy_adds_vy_to_vx_and_sets_carry() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x0a, 0b1111_1111)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();
        Ops::LD(0x0b, 0b111_0000)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        Ops::ADDV(0x0a, 0x0b)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.va, 0b0110_1111);
        assert_eq!(registers.vf, 1);

        Ops::LD(0x0c, 0b0000_0011)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        Ops::ADDV(0x0b, 0x0c)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.vb, 0b0111_0011);
        assert_eq!(registers.vf, 0);
    }

    #[test]
    fn op_sub_vx_vy_subtract_vy_from_vx_and_set_not_borrow() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x0a, 7)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();
        Ops::LD(0x0b, 3)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();
        Ops::LD(0x0c, 5)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();
        Ops::LD(0x0d, 9)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        Ops::SUB(0x0a, 0x0b)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.va, 4); // 7 - 3 = 4
        assert_eq!(registers.vf, 1);

        Ops::SUB(0x0c, 0x0d)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.vc, 252); // 5 - 9 [(252 + 9) % 256 = 5]  256 = u8::MAX + 1
        assert_eq!(registers.vf, 0);
    }

    #[test]
    fn op_subn_vx_vy_subtract_vx_from_vy_and_set_not_borrow() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x0a, 7)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();
        Ops::LD(0x0b, 10)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();
        Ops::LD(0x0c, 12)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();
        Ops::LD(0x0d, 9)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        Ops::SUBN(0x0a, 0x0b)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.va, 3); // 10 - 7 = 3
        assert_eq!(registers.vf, 1);

        Ops::SUBN(0x0c, 0x0d)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        assert_eq!(registers.vc, 253); // 9 - 12 = [(253 + 12) % 256 = 9]
        assert_eq!(registers.vf, 0);
    }

    #[test]
    fn op_shr_vx_right_shifts() {
        let ops = vec![Ops::LD(0x0a, 0b1111_1111), Ops::SHR(0x0a)];
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        ops.iter().for_each(|op| {
            (*op)
                .call(&mut registers, &mut memory, &mut screen, &mut controller)
                .unwrap();
        });

        assert_eq!(registers.va, 0b0111_1111);
    }

    #[test]
    fn op_shr_vx_stores_least_significant_bit_in_vf() {
        let instructions = vec![Ops::LD(0x0a, 0b1111_1111), Ops::SHR(0x0a)];
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        instructions.iter().for_each(|instruction| {
            (*instruction)
                .call(&mut registers, &mut memory, &mut screen, &mut controller)
                .unwrap();
        });

        assert_eq!(registers.vf, 1);

        let instructions = vec![Ops::LD(0x0a, 0b0000_1110), Ops::SHR(0x0a)];

        instructions.iter().for_each(|instruction| {
            (*instruction)
                .call(&mut registers, &mut memory, &mut screen, &mut controller)
                .unwrap();
        });

        assert_eq!(registers.vf, 0);
    }

    #[test]
    fn op_shl_vx_left_shifts() {
        let ops = vec![Ops::LD(0x0a, 0b0111_1111), Ops::SHL(0x0a)];
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        ops.iter().for_each(|op| {
            (*op)
                .call(&mut registers, &mut memory, &mut screen, &mut controller)
                .unwrap();
        });

        assert_eq!(registers.va, 0b1111_1110);
    }

    #[test]
    fn op_shl_stores_most_significant_bit_in_vf() {
        let ops = vec![Ops::LD(0x0a, 0b1111_0000), Ops::SHL(0x0a)];
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        ops.iter().for_each(|op| {
            (*op)
                .call(&mut registers, &mut memory, &mut screen, &mut controller)
                .unwrap();
        });

        assert_eq!(registers.vf, 0b1000_0000);

        let ops = vec![Ops::LD(0x0a, 0b0111_0000), Ops::SHL(0x0a)];
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        ops.iter().for_each(|op| {
            (*op)
                .call(&mut registers, &mut memory, &mut screen, &mut controller)
                .unwrap();
        });

        assert_eq!(registers.vf, 0);
    }

    #[test]
    fn op_snev_increments_pc_if_vx_not_equals_vy() {
        let ops = vec![Ops::LD(0x0a, 42), Ops::LD(0x0b, 42), Ops::SNEV(0x0a, 0x0b)];
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        ops.iter().for_each(|op| {
            (*op)
                .call(&mut registers, &mut memory, &mut screen, &mut controller)
                .unwrap();
        });

        assert_eq!(registers.pc, PROGRAM_START);

        let ops = vec![Ops::LD(0x0a, 42), Ops::LD(0x0b, 24), Ops::SNEV(0x0a, 0x0b)];

        ops.iter().for_each(|op| {
            (*op)
                .call(&mut registers, &mut memory, &mut screen, &mut controller)
                .unwrap();
        });

        assert_eq!(registers.pc, PROGRAM_START + INSTRUCTION_LENGTH);
    }

    #[test]
    fn op_ldi_sets_i_register() {
        let ops = vec![Ops::LDI(0x0012)];
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        ops.iter().for_each(|op| {
            (*op)
                .call(&mut registers, &mut memory, &mut screen, &mut controller)
                .unwrap();
        });

        assert_eq!(registers.i, 0x0012);
    }

    #[test]
    fn op_ldi_addr_must_be_within_memory_bounds() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        if let Err(ChipeyteError::OpFailed(op, msg)) =
            Ops::LDI(0xf000).call(&mut registers, &mut memory, &mut screen, &mut controller)
        {
            assert_eq!(op, Ops::LDI(0xf000));
            assert!(msg.contains("out-of-bounds"));
            return;
        }

        panic!("Test failed!");
    }

    #[test]
    fn op_jpv0_jumps_to_nnn_plus_v0() {
        let ops = vec![Ops::LD(0x00, 0x10), Ops::JPV0(0x0220)];
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        ops.iter().for_each(|op| {
            (*op)
                .call(&mut registers, &mut memory, &mut screen, &mut controller)
                .unwrap();
        });

        assert_eq!(registers.pc, 0x0230);
    }

    #[test]
    fn op_jpv0_returns_error_if_resulting_address_is_out_of_bounds() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x00, 0xff)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        match Ops::JPV0(0x0fff).call(&mut registers, &mut memory, &mut screen, &mut controller) {
            Err(ChipeyteError::OpFailed(Ops::JPV0(0x0fff), msg)) => {
                assert!(msg.contains("outside of program area"));
            }
            _ => panic!("Did not return appropriate error!"),
        }
    }

    #[test]
    fn op_jpv0_returns_error_if_resulting_address_is_outside_of_program_area() {
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x00, 0xff)
            .call(&mut registers, &mut memory, &mut screen, &mut controller)
            .unwrap();

        match Ops::JPV0(0x0000).call(&mut registers, &mut memory, &mut screen, &mut controller) {
            Err(ChipeyteError::OpFailed(Ops::JPV0(0x0000), msg)) => {
                assert!(msg.contains("outside of program area"));
            }
            _ => panic!("Did not return appropriate error!"),
        }
    }

    #[test]
    fn op_rnd_sets_vx_to_a_random_number() {
        let ops = vec![Ops::RND(0x0c, 0xff)];
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        ops.iter().for_each(|op| {
            (*op)
                .call(&mut registers, &mut memory, &mut screen, &mut controller)
                .unwrap();
        });

        // This test might fail if the generated random number is 0
        assert!(registers.vc > 0);
    }

    #[test]
    fn op_drw_draws_8_by_n_sprite_at_pos_vx_vy() {
        let ops: Vec<Ops> = vec![];
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        ops.iter().for_each(|op| {
            (*op)
                .call(&mut registers, &mut memory, &mut screen, &mut controller)
                .unwrap();
        });

        todo!();
    }

    #[test]
    fn op_drw_wraps_around_screen_edges() {
        let ops: Vec<Ops> = vec![];
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        ops.iter().for_each(|op| {
            (*op)
                .call(&mut registers, &mut memory, &mut screen, &mut controller)
                .unwrap();
        });

        todo!();
    }

    #[test]
    fn op_drw_wraps_xor_drawn_pixels() {
        let ops: Vec<Ops> = vec![Ops::RND(0x0c, 0xff)];
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        ops.iter().for_each(|op| {
            (*op)
                .call(&mut registers, &mut memory, &mut screen, &mut controller)
                .unwrap();
        });

        todo!();
    }

    #[test]
    fn op_ldvdt_sets_the_vx_equal_to_dt() {
        let ops = vec![Ops::LDVDT(0x0d)];
        let mut memory = Memory::new();
        let mut screen = MockScreen::init();
        let mut controller = MockController::new();
        let mut registers = Registers::new(PROGRAM_START);

        registers.dt = 42;

        ops.iter().for_each(|op| {
            (*op)
                .call(&mut registers, &mut memory, &mut screen, &mut controller)
                .unwrap();
        });

        assert_eq!(registers.vd, 42);
    }
}
