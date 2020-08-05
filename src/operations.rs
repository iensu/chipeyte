use crate::{types::*, Memory, Registers};

pub trait Callable {
    fn call(register: &mut Registers, memory: &mut Memory);
}

#[derive(Debug, PartialEq)]
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
