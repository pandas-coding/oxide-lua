use oxide_lua::{
    api::{self, LuaAPI, LuaVM, consts},
    binary,
    binary::chunk::Prototype,
    state::{self, LuaState},
    vm::{self, instruction::Instruction},
};
use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

pub use crate::common::binary_test::{LUA_FILE_PATH, LUAC_OUT_FILE_PATH};
use crate::common::lua_vm_test;

pub fn get_proto() -> io::Result<Prototype, > {
    let filepath = env::current_dir()?.join(PathBuf::from(lua_vm_test::LUAC_OUT_FILE_PATH));
    println!("filepath: {:?}", filepath);
    let mut file = File::open(filepath)?;

    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    let proto = binary::undump(data);
    return Ok(proto);
}

pub fn lua_main(proto: Prototype) {
    let nregs = proto.max_stack_size;
    let mut ls = state::new_lua_state((nregs + 8) as usize, proto);
    ls.set_top(nregs as isize);
    loop {
        let pc = ls.pc();
        let instr = ls.fetch();
        if instr.opcode() != vm::opcodes::OP_RETURN {
            instr.execute(&mut ls);
            print!("[{:04}] {} ", pc + 1, instr.opname());
            print_stack(&ls);
        } else {
            break;
        }
    }
}

fn print_stack(ls: &LuaState) {
    let top = ls.get_top();
    for i in 1..top + 1 {
        let t = ls.type_id(i);
        match t {
            LUA_TBOOLEAN => print!("[{}]", ls.to_boolean(i)),
            LUA_TNUMBER => print!("[{}]", ls.to_number(i)),
            LUA_TSTRING => print!("[{:?}]", ls.to_string(i)),
            _ => print!("[{}]", ls.type_name(t)), // other values
        }
    }
    println!();
}
