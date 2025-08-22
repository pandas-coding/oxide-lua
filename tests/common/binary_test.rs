#![allow(dead_code)]
use oxide_lua::binary;

// lua file path from project root
pub const LUA_FILE_PATH: &str = "tests/resources/hello_world/hello_world.lua";
// luac .out file path from project root
pub const LUAC_OUT_FILE_PATH: &str = "tests/resources/hello_world/hello_world.out";

pub fn list(f: &binary::chunk::Prototype) {
    print_header(f);
    print_code(f);
    print_detail(f);
    for p in &(f.protos) {
        list(p);
    }
}

/// print lua main header
/// main <@hello_world.lua:0,0> (4 instructions)
pub fn print_header(f: &binary::chunk::Prototype) {
    let func_type = if f.line_defined > 0 { "function" } else { "main" };
    let vararg_flag = if f.is_vararg > 0 { "+" } else { "" };
    let source = f.source.as_ref().map(|x| x.as_str()).unwrap_or("");
    //let source = f.source.clone().unwrap_or(String::new());

    print!("\n{}", func_type);
    print!(" <{}:{},{}>", source, f.line_defined, f.last_line_defined);
    print!(" ({} instructions)\n", f.code.len());
    print!("{}{} params", f.num_params, vararg_flag);
    print!(", {} slots", f.max_stack_size);
    print!(", {} upvalues", f.upvalues.len());
    print!(", {} locals", f.loc_vars.len());
    print!(", {} constants", f.constants.len());
    print!(", {} functions\n", f.protos.len());
}

pub fn print_code(f: &binary::chunk::Prototype) {
    for pc in 0..f.code.len() {
        let line = f.line_info.get(pc).map(|n| n.to_string()).unwrap_or(String::new());
        println!("\t{}\t[{}]\t0x{:08X}", pc + 1, line, f.code[pc]);
    }
}

pub fn print_detail(f: &binary::chunk::Prototype) {
    print_consts(f);
    print_locals(f);
    print_upvals(f)
}

pub fn print_consts(f: &binary::chunk::Prototype) {
    let n = f.constants.len();
    println!("constants ({}):", n);
    for i in 0..n {
        print_const(i + 1, &f.constants[i]);
    }
}

pub fn print_const(n: usize, k: &binary::chunk::Constant) {
    use binary::chunk::Constant::*;
    match k {
        Nil => println!("\t{}\tnil", n),
        Boolean(b) => println!("\t{}\t{}", n, b),
        Number(x) => println!("\t{}\t{}", n, x),
        Integer(i) => println!("\t{}\t{}", n, i),
        Str(s) => println!("\t{}\t{:?}", n, s),
    }
}

pub fn print_locals(f: &binary::chunk::Prototype) {
    let n = f.loc_vars.len();
    println!("locals ({}):", n);
    for i in 0..n {
        let var = &f.loc_vars[i];
        println!("\t{}\t{}\t{}\t{}", i, var.var_name, var.start_pc + 1, var.end_pc + 1);
    }
}

pub fn print_upvals(f: &binary::chunk::Prototype) {
    let n = f.upvalues.len();
    println!("upvalues ({}):", n);
    for i in 0..n {
        let upval = &f.upvalues[i];
        let name = f.upvalue_names.get(i).map(|x| x.as_str()).unwrap_or("");
        println!("\t{}\t{}\t{}\t{}", i, name, upval.instack, upval.idx);
    }
}