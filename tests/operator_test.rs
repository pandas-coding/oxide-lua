use common::operator_test::print_stack;
use oxide_lua::{api::LuaAPI, api::consts, state};
use std::io;

mod common;

#[test]
fn operator_test() -> io::Result<()> {
    let mut ls = state::new_lua_state();

    ls.push_integer(1);
    ls.push_string("2.0".to_string());
    ls.push_string("3.0".to_string());
    ls.push_number(4.0);
    print_stack(&ls);

    ls.arith(consts::LUA_OPADD);
    print_stack(&ls);
    ls.arith(consts::LUA_OPBNOT);
    print_stack(&ls);
    ls.len(2);
    print_stack(&ls);
    ls.concat(3);
    print_stack(&ls);
    let x = ls.compare(1, 2, consts::LUA_OPEQ);
    ls.push_boolean(x);
    print_stack(&ls);
    Ok(())
}
