use oxide_lua::{
    self, 
    api::{self, LuaAPI},
    state::LuaState,
};

pub fn print_stack(ls: &LuaState) {
    use api::consts::*;
    
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
