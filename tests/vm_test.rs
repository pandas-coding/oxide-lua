use oxide_lua::binary;
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;

mod common;

#[test]
fn vm_test() -> io::Result<()> {
    use common::vm_test;

    println!("running vm_test root path: {:?}", env::current_dir()?);

    let filepath = env::current_dir()?.join(PathBuf::from(vm_test::LUAC_OUT_FILE_PATH));
    println!("filepath: {:?}", filepath);
    let mut file = File::open(filepath)?;

    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let proto = binary::undump(data);
    vm_test::list(&proto);

    Ok(())
}
