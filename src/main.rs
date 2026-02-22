mod runtime;
mod lua_bindings;
mod fs;

use runtime::*;
use lua_bindings::LuaRuntime;
use tokio::runtime::Builder;
use std::env;
use std::fs::read_to_string;

fn main() {
    // Get CLI arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <lua_script>", args[0]);
        return;
    }

    let script_path = &args[1];

    // Read Lua script
    let script_content = match read_to_string(script_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read {}: {}", script_path, e);
            return;
        }
    };

    // Create async runtime
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    rt.block_on(async {
        // Spawn the Lua runtime
        let lua = LuaRuntime::new();
        lua.run(&script_content);

        // Example async task (optional)
        spawn(async {
            delay(2).await;
            println!("Example delay finished!");
        });
    });
}
