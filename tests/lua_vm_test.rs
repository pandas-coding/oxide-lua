#![allow(dead_code)]
#![allow(unused_imports)]

use std::{env, io};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use oxide_lua::binary;

mod common;

#[test]
fn lua_vm_test() -> io::Result<()> {
    use common::lua_vm_test;

    println!("running lua_vm_test root path: {:?}", env::current_dir()?);

    let filepath = env::current_dir()?.join(PathBuf::from(lua_vm_test::LUAC_OUT_FILE_PATH));
    println!("filepath: {:?}", filepath);
    let mut file = File::open(filepath)?;

    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    let proto = binary::undump(data);
    lua_vm_test::lua_main(proto);
    
    Ok(())
}