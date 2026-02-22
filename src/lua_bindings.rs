use std::ffi::CString;
use luau_sys::{lua_State, luaL_newstate, luaL_openlibs, luaL_loadstring, lua_pcall};

pub struct LuaRuntime {
    pub state: *mut lua_State,
}

impl LuaRuntime {
    pub fn new() -> Self {
        unsafe {
            let L = luaL_newstate();
            luaL_openlibs(L);
            LuaRuntime { state: L }
        }
    }

    pub fn run(&self, script: &str) {
        unsafe {
            let c_script = CString::new(script).unwrap();
            if luaL_loadstring(self.state, c_script.as_ptr()) == 0 {
                if lua_pcall(self.state, 0, 0, 0) != 0 {
                    eprintln!("Error running script");
                }
            } else {
                eprintln!("Failed to load script");
            }
        }
    }
}
