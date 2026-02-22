mod runtime;
mod lua_bindings;
mod fs;

use runtime::*;
use lua_bindings::LuaRuntime;
use tokio::runtime::Builder;

fn main() {
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    rt.block_on(async {
        let lua = LuaRuntime::new();
        lua.run("print('Hello from Lune!')");

        spawn(async {
            delay(2).await;
            println!("2 seconds passed!");
        });
    });
}
