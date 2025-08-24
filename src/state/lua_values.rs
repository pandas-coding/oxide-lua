use crate::api::{self};

#[derive(Clone)]
pub enum LuaValue {
    Nil,
    Boolean(bool),
    Number(f64),
    Integer(i64),
    Str(String),
}

impl LuaValue {
    pub fn type_id(&self) -> i8 {
        use api::consts::*;
        
        match self {
            LuaValue::Nil => LUA_TNIL,
            LuaValue::Boolean(_) => LUA_TBOOLEAN,
            LuaValue::Number(_) => LUA_TNUMBER,
            LuaValue::Integer(_) => LUA_TNUMBER,
            LuaValue::Str(_) => LUA_TSTRING,
        }
    }

    pub fn to_boolean(&self) -> bool {
        match self {
            LuaValue::Nil => false,
            LuaValue::Boolean(b) => *b, // TODO
            _ => true,
        }
    }
}