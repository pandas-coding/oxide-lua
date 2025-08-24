use super::binary_test;
use oxide_lua::{
    binary,
    vm::{self, instruction::Instruction, opcodes},
};

pub use binary_test::{
    LUA_FILE_PATH,
    LUAC_OUT_FILE_PATH,
};

pub fn list(f: &binary::chunk::Prototype) {
    binary_test::print_header(f);
    print_code(f);
    binary_test::print_detail(f);
    for p in &(f.protos) {
        list(p);
    }
}

fn print_code(f: &binary::chunk::Prototype) {
    for pc in 0..f.code.len() {
        let line = f
            .line_info
            .get(pc)
            .map(|n| n.to_string())
            .unwrap_or(String::from("-"));
        let instr = f.code[pc];
        print!("\t{}\t[{}]\t{} \t", pc + 1, line, instr.opname());
        print_operands(instr);
        println!();
    }
}

fn print_operands(i: u32) {
    use vm::opcodes::*;
    match i.opmode() {
        OP_MODE_ABC => print_abc(i),
        OP_MODE_ABX => print_abx(i),
        OP_MODE_ASBX => print_asbx(i),
        OP_MODE_AX => print_ax(i),
        _ => (),
    }
}

fn print_abc(i: u32) {
    let (a, b, c) = i.abc();
    print!("{}", a);
    if i.b_mode() != opcodes::OP_ARG_N {
        if b > 0xFF {
            print!(" {}", -1 - (b & 0xFF))
        } else {
            print!(" {}", b)
        }
    }
    if i.c_mode() != opcodes::OP_ARG_N {
        if c > 0xFF {
            print!(" {}", -1 - (c & 0xFF))
        } else {
            print!(" {}", c)
        }
    }
}

fn print_abx(i: u32) {
    let (a, bx) = i.a_bx();
    print!("{}", a);
    if i.b_mode() == opcodes::OP_ARG_K {
        print!(" {}", -1 - bx)
    } else if i.b_mode() == opcodes::OP_ARG_U {
        print!(" {}", bx)
    }
}

fn print_asbx(i: u32) {
    let (a, sbx) = i.a_sbx();
    print!("{} {}", a, sbx);
}

fn print_ax(i: u32) {
    let ax = i.ax();
    print!("{}", -1 - ax);
}
