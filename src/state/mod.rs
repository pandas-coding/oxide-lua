mod lua_stack;
mod lua_state;
mod lua_value;

pub use lua_state::LuaState;

pub fn new_lua_state() -> LuaState {
    LuaState::new()
}
