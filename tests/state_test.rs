use common::state_test::print_stack;
use oxide_lua::{api::LuaAPI, state};
use std::io;

mod common;

#[test]
fn state_test() -> io::Result<()> {
    use common::lua_vm_test::get_proto;
    
    let proto = get_proto()?;
    let nregs = proto.max_stack_size;
    let mut ls = state::new_lua_state((nregs + 8) as usize, proto);

    ls.push_boolean(true);
    print_stack(&ls);
    ls.push_integer(10);
    print_stack(&ls);
    ls.push_nil();
    print_stack(&ls);
    ls.push_string("hello".to_string());
    print_stack(&ls);
    ls.push_value(-4);
    print_stack(&ls);
    ls.replace(3);
    print_stack(&ls);
    ls.set_top(6);
    print_stack(&ls);
    ls.remove(-3);
    print_stack(&ls);
    ls.set_top(-5);
    print_stack(&ls);

    Ok(())
}
