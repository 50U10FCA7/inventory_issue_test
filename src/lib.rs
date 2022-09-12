mod my_plugin;
mod my_plugin2;

use std::any::TypeId;

use linkme::distributed_slice;

pub use self::{my_plugin::MyPlugin, my_plugin2::MyPlugin2};

#[distributed_slice]
pub static FACTORY: [fn() -> Meta] = [..];

#[derive(Clone, Copy, Debug)]
pub struct Meta {
    pub id: TypeId,

    pub name: &'static str,
}

impl Meta {
    pub fn collect() -> Vec<Self> {
        FACTORY
            .into_iter()
            .map(|factory| (factory)())
            .collect()
    }
}

pub trait Plugin {
    const NAME: &'static str;
}

pub fn new_plugin<T: Plugin + 'static>() -> Meta {
    println!("registered {}: {:?}", T::NAME, TypeId::of::<T>());

    Meta {
        id: TypeId::of::<T>(),
        name: T::NAME,
    }
}
