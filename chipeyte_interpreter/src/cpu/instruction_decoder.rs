use crate::operations::Ops;
use crate::types::*;

pub fn decode(instruction: u16) -> Ops {
    match to_nibbles(instruction) {
        (0x0, 0x0, 0xE, 0x0) => Ops::CLS,
        (0x0, 0x0, 0xE, 0xE) => Ops::RET,
        (0x0, x, y, z) => Ops::SYS(to_addr(x, y, z)),
        (0x1, x, y, z) => Ops::JP(to_addr(x, y, z)),
        (0x2, x, y, z) => Ops::CALL(to_addr(x, y, z)),
        (0x3, vx, hi, lo) => Ops::SE(vx, nibbles_to_byte(hi, lo)),
        (0x4, vx, hi, lo) => Ops::SNE(vx, nibbles_to_byte(hi, lo)),
        (0x5, vx, vy, 0x0) => Ops::SEV(vx, vy),
        (0x6, vx, hi, lo) => Ops::LD(vx, nibbles_to_byte(hi, lo)),
        (0x7, vx, hi, lo) => Ops::ADD(vx, nibbles_to_byte(hi, lo)),
        (0x8, vx, vy, 0x0) => Ops::LDV(vx, vy),
        (0x8, vx, vy, 0x1) => Ops::OR(vx, vy),
        (0x8, vx, vy, 0x2) => Ops::AND(vx, vy),
        (0x8, vx, vy, 0x3) => Ops::XOR(vx, vy),
        (0x8, vx, vy, 0x4) => Ops::ADDV(vx, vy),
        (0x8, vx, vy, 0x5) => Ops::SUB(vx, vy),
        (0x8, vx, _, 0x6) => Ops::SHR(vx),
        (0x8, vx, vy, 0x7) => Ops::SUBN(vx, vy),
        (0x8, vx, _, 0xE) => Ops::SHL(vx),
        (0x9, vx, vy, 0x0) => Ops::SNEV(vx, vy),
        (0xA, x, y, z) => Ops::LDI(to_addr(x, y, z)),
        (0xB, x, y, z) => Ops::JPV0(to_addr(x, y, z)),
        (0xC, vx, hi, lo) => Ops::RND(vx, nibbles_to_byte(hi, lo)),
        (0xD, vx, vy, n) => Ops::DRW(vx, vy, n),
        (0xE, vx, 0x9, 0xE) => Ops::SKP(vx),
        (0xE, vx, 0xA, 0x1) => Ops::SKNP(vx),
        (0xF, vx, 0x0, 0x7) => Ops::LDVDT(vx),
        (0xF, vx, 0x0, 0xA) => Ops::LDK(vx),
        (0xF, vx, 0x1, 0x5) => Ops::LDDT(vx),
        (0xF, vx, 0x1, 0x8) => Ops::LDST(vx),
        (0xF, vx, 0x1, 0xE) => Ops::ADDI(vx),
        (0xF, vx, 0x2, 0x9) => Ops::LDF(vx),
        (0xF, vx, 0x3, 0x3) => Ops::LDB(vx),
        (0xF, vx, 0x5, 0x5) => Ops::LDIV(vx),
        (0xF, vx, 0x6, 0x5) => Ops::LDVI(vx),
        _ => Ops::UNKNOWN(instruction),
    }
}

fn to_nibbles(x: u16) -> (Nibble, Nibble, Nibble, Nibble) {
    let [hi_byte, lo_byte] = x.to_be_bytes();
    (
        (hi_byte & 0xF0) >> 4,
        hi_byte & 0x0F,
        (lo_byte & 0xF0) >> 4,
        lo_byte & 0x0F,
    )
}

fn nibbles_to_byte(hi: Nibble, lo: Nibble) -> Byte {
    hi << 4 | lo
}

fn to_addr(hi: Nibble, mid: Nibble, lo: Nibble) -> Addr {
    (u16::from(hi) << 8) | (u16::from(mid) << 4) | u16::from(lo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_returns_correct_op() {
        assert_eq!(decode(0x00E0), Ops::CLS);
        assert_eq!(decode(0x00EE), Ops::RET);
        assert_eq!(decode(0x0ABC), Ops::SYS(0x0ABC));
        assert_eq!(decode(0x1CBA), Ops::JP(0x0CBA));
        assert_eq!(decode(0x2BAC), Ops::CALL(0x0BAC));
        assert_eq!(decode(0x30AB), Ops::SE(0x0, 0xAB));
        assert_eq!(decode(0x40AB), Ops::SNE(0x0, 0xAB));
        assert_eq!(decode(0x5AB0), Ops::SEV(0xA, 0xB));
        assert_eq!(decode(0x6AB0), Ops::LD(0xA, 0xB0));
        assert_eq!(decode(0x7D01), Ops::ADD(0xD, 0x01));
        assert_eq!(decode(0xEEEE), Ops::UNKNOWN(0xEEEE));
    }
}
