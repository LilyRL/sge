use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct UserStorage(HashMap<TypeId, Box<dyn Any>>);

sge_global::global!(UserStorage, user_storage);

pub fn init() {
    set_user_storage(UserStorage::new());
    log::info!("Initialized user storage");
}

impl UserStorage {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn store<T: Any>(&mut self, data: T) {
        self.0.insert(data.type_id(), Box::new(data));
    }

    pub fn exists<T: Any>(&self) -> bool {
        self.0.contains_key(&TypeId::of::<T>())
    }

    pub fn initialize<T: Any>(&mut self, data: T) {
        self.0
            .entry(data.type_id())
            .or_insert_with(|| Box::new(data));
    }

    pub fn try_get<T: Any>(&mut self) -> Option<&T> {
        self.0
            .get(&TypeId::of::<T>())
            .and_then(|data| data.downcast_ref::<T>())
    }

    pub fn try_get_mut<T: Any>(&mut self) -> Option<&mut T> {
        self.0
            .get_mut(&TypeId::of::<T>())
            .and_then(|data| data.downcast_mut::<T>())
    }

    pub fn get<T: Any>(&mut self) -> &T {
        self.try_get().unwrap()
    }

    pub fn get_mut<T: Any>(&mut self) -> &mut T {
        self.try_get_mut().unwrap()
    }
}
