use std::env;

use mlua::{IntoLua, Lua, Result, Value};

fn run_require(lua: &Lua, path: impl IntoLua) -> Result<Value> {
    lua.load(r#"return require(...)"#).call(path)
}

#[track_caller]
fn get_str(value: &Value, key: impl IntoLua) -> String {
    value.as_table().unwrap().get::<String>(key).unwrap()
}

#[test]
fn test_require_small() {
    let lua = Lua::new();

    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let res = run_require(&lua, "./require/without_config/dependency").unwrap();
    assert_eq!("result from dependency", get_str(&res, 1));
}
