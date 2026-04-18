use std::pin::Pin;

use slotmap::SlotMap;

slotmap::new_key_type! {
    pub struct FutureKey;
}

pub type Task = dyn Future<Output = ()>;

pub struct AsyncRuntime {
    tasks: SlotMap<FutureKey, Pin<Box<Task>>>,
}

impl AsyncRuntime {
    pub fn update(&mut self) {}
}
