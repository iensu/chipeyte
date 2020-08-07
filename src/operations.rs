use crate::{cpu::registers::NumericRegister, types::*, ChipeyteError, Memory, Registers};
use std::convert::TryFrom;

const INSTRUCTION_LENGTH: u16 = 2;
const STACK_ENTRY_LENGTH: u8 = 2;

pub trait Callable {
    fn call(&self, register: &mut Registers, memory: &mut Memory) -> Result<(), ChipeyteError>;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Ops {
    UNKNOWN(u16),
    SYS(Addr),
    CLS,
    RET,
    JP(Addr),
    CALL(Addr),
    SE(V, Byte),
    SEV(V, V),
    SNE(V, Byte),
    LD(V, Byte),
    ADD(V, Byte),
    LDV(V, V),
    OR(V, V),
    AND(V, V),
    XOR(V, V),
    ADDV(V, V),
    SUB(V, V),
    SHR(V),
    SUBN(V, V),
    SHL(V),
    SNEV(V, V),
    LDI(Addr),
    JPV0(Addr),
    RND(V, Byte),
    DRW(V, V, Nibble),
    SKP(V),
    SKNP(V),
    LDVDT(V),
    LDK(V),
    LDDT(V),
    LDST(V),
    ADDI(V),
    LDF(V),
    LDB(V),
    LDIV(V),
    LDVI(V),
}

impl Callable for Ops {
    fn call(&self, registers: &mut Registers, memory: &mut Memory) -> Result<(), ChipeyteError> {
        match &*self {
            Ops::UNKNOWN(op) => Err(ChipeyteError::OpFailed(
                *self,
                format!("Unknown operation: {:04x?}", op),
            )),

            Ops::SYS(_) => Ok(()),

            Ops::CLS => Err(ChipeyteError::OpNotImplemented(*self)),

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
                let register = NumericRegister::try_from(*v)?;
                let value = registers.get_numeric_register(&register);

                if value == *byte {
                    registers.pc += INSTRUCTION_LENGTH;
                }
                Ok(())
            }

            Ops::SNE(v, byte) => {
                let register = NumericRegister::try_from(*v)?;
                let value = registers.get_numeric_register(&register);

                if value != *byte {
                    registers.pc += INSTRUCTION_LENGTH;
                }
                Ok(())
            }

            Ops::SEV(vx, vy) => {
                let reg_x = NumericRegister::try_from(*vx)?;
                let reg_y = NumericRegister::try_from(*vy)?;
                let x = registers.get_numeric_register(&reg_x);
                let y = registers.get_numeric_register(&reg_y);

                if x == y {
                    registers.pc += INSTRUCTION_LENGTH;
                }
                Ok(())
            }

            Ops::LD(v, byte) => {
                let register = NumericRegister::try_from(*v)?;

                registers.set_numeric_register(&register, *byte);
                Ok(())
            }

            Ops::ADD(v, byte) => {
                let register = NumericRegister::try_from(*v)?;

                let value = registers.get_numeric_register(&register);
                let result = byte.wrapping_add(value);

                registers.set_numeric_register(&register, result);
                Ok(())
            }

            Ops::LDV(vx, vy) => {
                let reg_x = NumericRegister::try_from(*vx).unwrap();
                let reg_y = NumericRegister::try_from(*vy).unwrap();

                registers.set_numeric_register(&reg_x, registers.get_numeric_register(&reg_y));
                Ok(())
            }

            Ops::OR(vx, vy) => {
                let reg_x = NumericRegister::try_from(*vx).unwrap();
                let reg_y = NumericRegister::try_from(*vy).unwrap();

                let x = registers.get_numeric_register(&reg_x);
                let y = registers.get_numeric_register(&reg_y);

                registers.set_numeric_register(&reg_x, x | y);
                Ok(())
            }

            Ops::AND(vx, vy) => {
                let reg_x = NumericRegister::try_from(*vx).unwrap();
                let reg_y = NumericRegister::try_from(*vy).unwrap();

                let x = registers.get_numeric_register(&reg_x);
                let y = registers.get_numeric_register(&reg_y);

                registers.set_numeric_register(&reg_x, x & y);
                Ok(())
            }

            Ops::XOR(vx, vy) => {
                let reg_x = NumericRegister::try_from(*vx).unwrap();
                let reg_y = NumericRegister::try_from(*vy).unwrap();

                let x = registers.get_numeric_register(&reg_x);
                let y = registers.get_numeric_register(&reg_y);

                registers.set_numeric_register(&reg_x, x ^ y);
                Ok(())
            }

            Ops::ADDV(vx, vy) => {
                let reg_x = NumericRegister::try_from(*vx).unwrap();
                let reg_y = NumericRegister::try_from(*vy).unwrap();

                let x = registers.get_numeric_register(&reg_x);
                let y = registers.get_numeric_register(&reg_y);
                let value = x as u16 + y as u16;

                registers.set_numeric_register(&reg_x, value as u8);

                if value > u8::MAX.into() {
                    registers.set_numeric_register(&NumericRegister::VF, 1);
                } else {
                    registers.set_numeric_register(&NumericRegister::VF, 0);
                }

                Ok(())
            }

            Ops::SUB(vx, vy) => {
                let reg_x = NumericRegister::try_from(*vx).unwrap();
                let reg_y = NumericRegister::try_from(*vy).unwrap();

                let x = registers.get_numeric_register(&reg_x);
                let y = registers.get_numeric_register(&reg_y);

                registers.set_numeric_register(&reg_x, x.wrapping_sub(y));

                if x > y {
                    registers.set_numeric_register(&NumericRegister::VF, 1);
                } else {
                    registers.set_numeric_register(&NumericRegister::VF, 0);
                }

                Ok(())
            }

            Ops::SHR(vx) => {
                let reg_x = NumericRegister::try_from(*vx).unwrap();

                let x = registers.get_numeric_register(&reg_x);

                let least_significant_bit = x & 0b0000_0001;

                registers.vf = least_significant_bit;

                registers.set_numeric_register(&reg_x, x >> 1);
                Ok(())
            }

            Ops::SUBN(vx, vy) => {
                let reg_x = NumericRegister::try_from(*vx).unwrap();
                let reg_y = NumericRegister::try_from(*vy).unwrap();

                let x = registers.get_numeric_register(&reg_x);
                let y = registers.get_numeric_register(&reg_y);

                registers.set_numeric_register(&reg_x, y.wrapping_sub(x));

                if y > x {
                    registers.set_numeric_register(&NumericRegister::VF, 1);
                } else {
                    registers.set_numeric_register(&NumericRegister::VF, 0);
                }

                Ok(())
            }

            Ops::SHL(vx) => {
                let reg_x = NumericRegister::try_from(*vx).unwrap();
                let x = registers.get_numeric_register(&reg_x);

                let most_significant_bit = x & 0b1000_0000;

                registers.vf = most_significant_bit;

                registers.set_numeric_register(&reg_x, x << 1);
                Ok(())
            }

            Ops::SNEV(vx, vy) => {
                let reg_x = NumericRegister::try_from(*vx).unwrap();
                let reg_y = NumericRegister::try_from(*vy).unwrap();

                let x = registers.get_numeric_register(&reg_x);
                let y = registers.get_numeric_register(&reg_y);

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

            Ops::JPV0(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::RND(_, _) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::DRW(_, _, _) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::SKP(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::SKNP(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::LDVDT(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::LDK(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::LDDT(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::LDST(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::ADDI(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::LDF(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::LDB(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::LDIV(_) => Err(ChipeyteError::OpNotImplemented(*self)),
            Ops::LDVI(_) => Err(ChipeyteError::OpNotImplemented(*self)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROGRAM_START: u16 = 0x0200;
    const INSTRUCTION_LENGTH: u16 = 2;

    #[test]
    fn op_sys_is_ignored() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::SYS(0x0aaa).call(&mut registers, &mut memory).unwrap();

        assert_eq!(registers, Registers::new(PROGRAM_START));
        assert_eq!(memory, Memory::new());
    }

    #[test]
    fn op_ret_returns() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::CALL(0x0aaa).call(&mut registers, &mut memory).unwrap();

        assert_eq!(registers.sp, 0x0002);
        assert_eq!(registers.pc, 0x0aaa);

        Ops::RET.call(&mut registers, &mut memory).unwrap();

        assert_eq!(memory.get_u16(0x0002), 0x0200);
        assert_eq!(registers.sp, 0x00);
    }

    #[test]
    fn op_jp_jumps_to_addr() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::JP(0x0aaa).call(&mut registers, &mut memory).unwrap();

        assert_eq!(registers.pc, 0x0aaa);
    }

    #[test]
    fn op_jp_must_be_within_memory_bounds() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        if let Err(ChipeyteError::OpFailed(op, msg)) =
            Ops::JP(0xf000).call(&mut registers, &mut memory)
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
        let mut registers = Registers::new(PROGRAM_START);

        Ops::CALL(0x0aaa).call(&mut registers, &mut memory).unwrap();

        assert_eq!(registers.pc, 0x0aaa);
        assert_eq!(registers.sp, 0x0002);
        assert_eq!(memory.get_u16(0x0002), 0x0200);
    }

    #[test]
    fn op_call_addr_must_be_within_memory_bounds() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        if let Err(ChipeyteError::OpFailed(op, msg)) =
            Ops::CALL(0xf000).call(&mut registers, &mut memory)
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
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x08, 0x42)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::SE(0x08, 0x42)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.sp, 0);
        assert_eq!(registers.pc, PROGRAM_START + INSTRUCTION_LENGTH);
    }

    #[test]
    fn op_se_vkk_does_not_increment_pc_if_v_not_equal_to_kk() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x08, 0x84)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::SE(0x08, 0x42)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.sp, 0);
        assert_eq!(registers.pc, PROGRAM_START);
    }

    #[test]
    fn op_sne_vkk_does_increment_pc_if_v_equals_kk() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x08, 0x42)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::SNE(0x08, 0x42)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.sp, 0);
        assert_eq!(registers.pc, PROGRAM_START);
    }

    #[test]
    fn op_sne_vkk_increments_pc_if_v_not_equal_to_kk() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x08, 0x42)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::SNE(0x08, 0x84)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.sp, 0);
        assert_eq!(registers.pc, PROGRAM_START + INSTRUCTION_LENGTH);
    }

    #[test]
    fn op_se_vxvy_increments_pc_if_vx_equals_vy() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x08, 0x42)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::LD(0x0a, 0x42)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::SEV(0x08, 0x0a)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.sp, 0);
        assert_eq!(registers.pc, PROGRAM_START + INSTRUCTION_LENGTH);
    }

    #[test]
    fn op_se_vxvy_does_not_increment_pc_if_vx_not_equal_to_vy() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x08, 0x42)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::LD(0x0a, 0x84)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::SE(0x08, 0x0a)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.sp, 0);
        assert_eq!(registers.pc, PROGRAM_START);
    }

    #[test]
    fn op_ld_vkk_sets_register_v_to_kk() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x0a, 0x66)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.va, 0x66);
    }

    #[test]
    fn op_add_vkk_adds_kk_to_v() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0, 30)
            .call(&mut registers, &mut memory)
            .expect("Failed to set register");
        Ops::ADD(0, 12)
            .call(&mut registers, &mut memory)
            .expect("Failed to add to register");

        assert_eq!(registers.v0, 42);
    }

    #[test]
    fn op_add_vkk_adds_kk_to_v_no_carry() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0, 200)
            .call(&mut registers, &mut memory)
            .expect("Failed to set register");
        Ops::ADD(0, 200)
            .call(&mut registers, &mut memory)
            .expect("Failed to add to register");

        assert_eq!(registers.v0, 144);
        assert_eq!(registers.vf, 0); // CARRY FLAG
    }

    #[test]
    fn op_ld_vxvy_stores_vx_in_vy() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x0b, 0x09)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::LDV(0x0a, 0x0b)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.va, 9);
    }

    #[test]
    fn op_or_vx_vy_stores_bitwise_or_in_vx() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x0a, 0b1001_0111)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::LD(0x0b, 0b0110_1001)
            .call(&mut registers, &mut memory)
            .unwrap();

        Ops::OR(0x0a, 0x0b)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.va, 0b1111_1111);
    }

    #[test]
    fn op_and_vx_vy_stores_bitwise_and_in_vx() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x0a, 0b1001_0111)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::LD(0x0b, 0b0110_1001)
            .call(&mut registers, &mut memory)
            .unwrap();

        Ops::AND(0x0a, 0x0b)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.va, 0b0000_0001);
    }

    #[test]
    fn op_xor_vx_vy_stores_bitwise_xor_in_vx() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x0a, 0b1001_0111)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::LD(0x0b, 0b0110_1001)
            .call(&mut registers, &mut memory)
            .unwrap();

        Ops::XOR(0x0a, 0x0b)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.va, 0b1111_1110);
    }

    #[test]
    fn op_add_vx_vy_adds_vy_to_vx_and_sets_carry() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x0a, 0b1111_1111)
            .call(&mut registers, &mut memory)
            .unwrap();
        Ops::LD(0x0b, 0b111_0000)
            .call(&mut registers, &mut memory)
            .unwrap();

        Ops::ADDV(0x0a, 0x0b)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.va, 0b0110_1111);
        assert_eq!(registers.vf, 1);

        Ops::LD(0x0c, 0b0000_0011)
            .call(&mut registers, &mut memory)
            .unwrap();

        Ops::ADDV(0x0b, 0x0c)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.vb, 0b0111_0011);
        assert_eq!(registers.vf, 0);
    }

    #[test]
    fn op_sub_vx_vy_subtract_vy_from_vx_and_set_not_borrow() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x0a, 7).call(&mut registers, &mut memory).unwrap();
        Ops::LD(0x0b, 3).call(&mut registers, &mut memory).unwrap();
        Ops::LD(0x0c, 5).call(&mut registers, &mut memory).unwrap();
        Ops::LD(0x0d, 9).call(&mut registers, &mut memory).unwrap();

        Ops::SUB(0x0a, 0x0b)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.va, 4); // 7 - 3 = 4
        assert_eq!(registers.vf, 1);

        Ops::SUB(0x0c, 0x0d)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.vc, 252); // 5 - 9 [(252 + 9) % 256 = 5]  256 = u8::MAX + 1
        assert_eq!(registers.vf, 0);
    }

    #[test]
    fn op_subn_vx_vy_subtract_vx_from_vy_and_set_not_borrow() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        Ops::LD(0x0a, 7).call(&mut registers, &mut memory).unwrap();
        Ops::LD(0x0b, 10).call(&mut registers, &mut memory).unwrap();
        Ops::LD(0x0c, 12).call(&mut registers, &mut memory).unwrap();
        Ops::LD(0x0d, 9).call(&mut registers, &mut memory).unwrap();

        Ops::SUBN(0x0a, 0x0b)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.va, 3); // 10 - 7 = 3
        assert_eq!(registers.vf, 1);

        Ops::SUBN(0x0c, 0x0d)
            .call(&mut registers, &mut memory)
            .unwrap();

        assert_eq!(registers.vc, 253); // 9 - 12 = [(253 + 12) % 256 = 9]
        assert_eq!(registers.vf, 0);
    }

    #[test]
    fn op_shr_vx_right_shifts() {
        let ops = vec![Ops::LD(0x0a, 0b1111_1111), Ops::SHR(0x0a)];
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        ops.iter().for_each(|op| {
            (*op).call(&mut registers, &mut memory).unwrap();
        });

        assert_eq!(registers.va, 0b0111_1111);
    }

    #[test]
    fn op_shr_vx_stores_least_significant_bit_in_vf() {
        let instructions = vec![Ops::LD(0x0a, 0b1111_1111), Ops::SHR(0x0a)];
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        instructions.iter().for_each(|instruction| {
            (*instruction).call(&mut registers, &mut memory).unwrap();
        });

        assert_eq!(registers.vf, 1);

        let instructions = vec![Ops::LD(0x0a, 0b0000_1110), Ops::SHR(0x0a)];

        instructions.iter().for_each(|instruction| {
            (*instruction).call(&mut registers, &mut memory).unwrap();
        });

        assert_eq!(registers.vf, 0);
    }

    #[test]
    fn op_shl_vx_left_shifts() {
        let ops = vec![Ops::LD(0x0a, 0b0111_1111), Ops::SHL(0x0a)];
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        ops.iter().for_each(|op| {
            (*op).call(&mut registers, &mut memory).unwrap();
        });

        assert_eq!(registers.va, 0b1111_1110);
    }

    #[test]
    fn op_op_shl_stores_most_significant_bit_in_vf() {
        let ops = vec![Ops::LD(0x0a, 0b1111_0000), Ops::SHL(0x0a)];
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        ops.iter().for_each(|op| {
            (*op).call(&mut registers, &mut memory).unwrap();
        });

        assert_eq!(registers.vf, 0b1000_0000);

        let ops = vec![Ops::LD(0x0a, 0b0111_0000), Ops::SHL(0x0a)];
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        ops.iter().for_each(|op| {
            (*op).call(&mut registers, &mut memory).unwrap();
        });

        assert_eq!(registers.vf, 0);
    }

    #[test]
    fn op_snev_increments_pc_if_vx_not_equals_vy() {
        let ops = vec![Ops::LD(0x0a, 42), Ops::LD(0x0b, 42), Ops::SNEV(0x0a, 0x0b)];
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        ops.iter().for_each(|op| {
            (*op).call(&mut registers, &mut memory).unwrap();
        });

        assert_eq!(registers.pc, PROGRAM_START);

        let ops = vec![Ops::LD(0x0a, 42), Ops::LD(0x0b, 24), Ops::SNEV(0x0a, 0x0b)];

        ops.iter().for_each(|op| {
            (*op).call(&mut registers, &mut memory).unwrap();
        });

        assert_eq!(registers.pc, PROGRAM_START + INSTRUCTION_LENGTH);
    }

    #[test]
    fn op_ldi_sets_i_register() {
        let ops = vec![Ops::LDI(0x0012)];
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        ops.iter().for_each(|op| {
            (*op).call(&mut registers, &mut memory).unwrap();
        });

        assert_eq!(registers.i, 0x0012);
    }

    #[test]
    fn op_ldi_addr_must_be_within_memory_bounds() {
        let mut memory = Memory::new();
        let mut registers = Registers::new(PROGRAM_START);

        if let Err(ChipeyteError::OpFailed(op, msg)) =
            Ops::LDI(0xf000).call(&mut registers, &mut memory)
        {
            assert_eq!(op, Ops::LDI(0xf000));
            assert!(msg.contains("out-of-bounds"));
            return;
        }

        panic!("Test failed!");
    }
}
