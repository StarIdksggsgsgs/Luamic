mod luau_bindings;
use luau_bindings::*;
use std::ffi::CString;
use std::env;
use std::fs;

fn main() {
    unsafe {
        let L = luauL_newstate();
        luauL_openlibs(L);

        let args: Vec<String> = env::args().collect();
        let code = if args.len() > 1 {
            fs::read_to_string(&args[1]).expect("Failed to read file")
        } else {
            println!("Enter Lua code (Ctrl+D to finish):");
            let mut input = String::new();
            std::io::stdin().read_to_string(&mut input).unwrap();
            input
        };

        let script = CString::new(code).unwrap();

        if luauL_dostring(L, script.as_ptr()) != 0 {
            let err = luau_tolstring(L, -1, std::ptr::null_mut());
            println!("Error: {}", CString::from_raw(err as *mut i8).to_str().unwrap());
        }

        luau_close(L);
    }
}
