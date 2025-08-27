use crate::api::{self};
use crate::number;

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

    // http://www.lua.org/manual/5.3/manual.html#3.4.3
    pub fn to_number(&self) -> Option<f64> {
        match self {
            LuaValue::Integer(i) => Some(*i as f64),
            LuaValue::Number(n) => Some(*n),
            LuaValue::Str(s) => s.parse::<f64>().ok(), // TODO
            _ => None,
        }
    }

    // http://www.lua.org/manual/5.3/manual.html#3.4.3
    pub fn to_integer(&self) -> Option<i64> {
        match self {
            LuaValue::Integer(i) => Some(*i),
            LuaValue::Number(n) => number::math::float_to_integer(*n),
            LuaValue::Str(s) => number::math::string_to_integer(s),
            _ => None,
        }
    }
}
