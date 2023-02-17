use anyhow::{bail, Ok, Result};
use wasmtime::MemoryType;
use webscript_core::Environment;

use crate::runtime::get_global_imports;

use super::{Loader, Store, StoreData};

pub struct Engine {
    env: Environment,
    loader: Loader,
    store: Store,
}

impl Engine {
    pub fn create(env: Environment) -> Result<Self> {
        let store = Store::new(MemoryType::new(128, None))?;

        Ok(Self {
            env,
            loader: Loader::new(),
            store,
        })
    }

    fn store_mut(&mut self) -> &mut wasmtime::Store<StoreData> {
        &mut self.store
    }

    pub fn load_main(&mut self, url: String) -> Result<wasmtime::Module> {
        // Loading
        let code = self.loader.load_module(&url)?;
        let output = webscript_core::load_main(url, code)?;

        // Load the module
        let module = wasmtime::Module::from_binary(self.store_mut().engine(), &output)?;

        // Load imports
        let global_imports = get_global_imports(&mut self.store);

        let mut imports = vec![];

        for import in module.imports() {
            if import.module() == "global" {
                if let Some(func) = global_imports.get(import.name()) {
                    imports.push(func.clone().into());
                } else {
                    bail!("Import not found: {}", import.name());
                }
            }
        }

        // Run the module
        let instance = wasmtime::Instance::new(self.store_mut(), &module, &imports)?;
        let fn_main = instance.get_typed_func::<(), ()>(self.store_mut(), "main")?;

        fn_main.call(self.store_mut(), ())?;

        Ok(module)
    }
}
