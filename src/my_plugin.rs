use std::marker::PhantomData;

use linkme::distributed_slice;

use super::{new_plugin, FACTORY, Meta, Plugin};

#[distributed_slice(FACTORY)]
static MY_PLUGIN: fn() -> Meta = new_plugin::<MyPlugin<()>>;

pub struct MyPlugin<T>(PhantomData<T>);

impl<T> Plugin for MyPlugin<T> {
    const NAME: &'static str = "MyPlugin";
}
