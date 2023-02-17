use std::ops::{Deref, DerefMut};

use anyhow::Result;

pub struct Store(wasmtime::Store<StoreData>);

impl Store {
    pub fn new(ty: wasmtime::MemoryType) -> Result<Self> {
        let engine = wasmtime::Engine::default();
        let mut store = wasmtime::Store::new(&engine, StoreData::default());
        let memory = wasmtime::Memory::new(&mut store, ty)?;

        store.data_mut().set_memory(memory);

        Ok(Store(store))
    }
}

impl Deref for Store {
    type Target = wasmtime::Store<StoreData>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Store {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct StoreData {
    memory: Option<wasmtime::Memory>,
}

impl StoreData {
    pub fn memory(&self) -> &wasmtime::Memory {
        self.memory.as_ref().unwrap()
    }

    pub fn memory_mut(&mut self) -> &mut wasmtime::Memory {
        self.memory.as_mut().unwrap()
    }

    fn set_memory(&mut self, memory: wasmtime::Memory) {
        self.memory = Some(memory);
    }
}

impl Default for StoreData {
    fn default() -> Self {
        Self { memory: None }
    }
}
