use super::opcodes::OPCODES;

const MAXARG_BX: isize = (1 << 18) - 1; // 262143
const MAXARG_SBX: isize = MAXARG_BX >> 1; // 131071

/*
 31       22       13       5    0
  +-------+^------+-^-----+-^-----
  |b=9bits |c=9bits |a=8bits|op=6|
  +-------+^------+-^-----+-^-----
  |    bx=18bits    |a=8bits|op=6|
  +-------+^------+-^-----+-^-----
  |   sbx=18bits    |a=8bits|op=6|
  +-------+^------+-^-----+-^-----
  |    ax=26bits            |op=6|
  +-------+^------+-^-----+-^-----
 31      23      15       7      0
*/
pub trait Instruction {
    fn opname(self) -> &'static str;
    fn opmode(self) -> u8;
    fn b_mode(self) -> u8;
    fn c_mode(self) -> u8;
    fn opcode(self) -> u8;
    fn abc(self) -> (isize, isize, isize);
    fn a_bx(self) -> (isize, isize);
    fn a_sbx(self) -> (isize, isize);
    fn ax(self) -> isize;
}

impl Instruction for u32 {
    /// get operation code name
    fn opname(self) -> &'static str {
        OPCODES[self.opcode() as usize].name
    }

    /// get operation code mode
    fn opmode(self) -> u8 {
        OPCODES[self.opcode() as usize].opmode
    }

    /// get operation number B mode
    fn b_mode(self) -> u8 {
        OPCODES[self.opcode() as usize].bmode
    }

    /// get operation number C mode
    fn c_mode(self) -> u8 {
        OPCODES[self.opcode() as usize].cmode
    }

    /// extract operation code from instruction
    fn opcode(self) -> u8 {
        self as u8 & 0x3F
    }

    /// extract params from iABC mode instruction
    fn abc(self) -> (isize, isize, isize) {
        let a = (self >> 6 & 0xFF) as isize;
        let c = (self >> 14 & 0x1FF) as isize;
        let b = (self >> 23 & 0x1FF) as isize;
        (a, b, c)
    }

    /// extract params from iABx mode instruction
    fn a_bx(self) -> (isize, isize) {
        let a = (self >> 6 & 0xFF) as isize;
        let bx = (self >> 14) as isize;
        (a, bx)
    }

    /// extract params from iAsBx mode instruction
    fn a_sbx(self) -> (isize, isize) {
        let (a, bx) = self.a_bx();
        (a, bx - MAXARG_SBX)
    }

    /// extract params from iAx mode instruction
    fn ax(self) -> isize {
        (self >> 6) as isize
    }
}
