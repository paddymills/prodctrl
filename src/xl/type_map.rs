
// based off of SingletonCollection (https://www.jakobmeier.ch/blogging/Untapped-Rust.html)

use core::any::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct TypeMap {
    data: HashMap<TypeId, Box<dyn Any>>,
}

impl TypeMap {

    pub fn set<T: Any>(&mut self, value: T) {
        self.data.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn get<T: Any>(&self) -> &T {
        self.data[&TypeId::of::<T>]
            .downcast_ref()
            .as_ref()
            .unwrap()
    }

    pub fn update<T: Any>(&mut self, update_func: Box<dyn Fn(T)>) {
        update_func( self.data.get_mut(&TypeId::of::<T>).unwrap() );
    }

}
