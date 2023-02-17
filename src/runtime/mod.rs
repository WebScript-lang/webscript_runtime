mod console;

use std::{collections::HashMap, ops::DerefMut};

pub use console::Console;
use wasmtime::Func;

use super::engine::Store;

macro_rules! import_fn {
    ($imports:ident, $store:ident, $name:literal, $func:ident) => {
        $imports.insert(stringify!($name), Func::wrap($store.deref_mut(), &$func));
    };
}

pub fn get_global_imports(store: &mut Store) -> HashMap<&'static str, Func> {
    let mut imports: HashMap<&'static str, Func> = HashMap::new();

    import_fn!(imports, store, "print", print);
    import_fn!(imports, store, "printNum", print_num);

    imports
}

fn print(value: i32) {
    println!("{value}");
}

fn print_num(value: f64) {
    println!("{value}");
}
