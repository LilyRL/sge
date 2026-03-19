use std::{any::Any, pin::Pin};

use slotmap::SlotMap;

slotmap::new_key_type! {
    pub struct FutureKey;
}

pub struct AsyncRuntime {
    futures: SlotMap<FutureKey, Pin<Box<dyn Future<Output = Box<dyn Any>>>>>,
}

impl AsyncRuntime {
    pub fn update(&mut self) {

    }
}
