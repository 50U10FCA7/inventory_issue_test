use super::{new_plugin, FACTORY, Meta, Plugin};

use linkme::distributed_slice;

#[distributed_slice(FACTORY)]
static MY_PLUGIN_2: fn() -> Meta = new_plugin::<MyPlugin2>;

pub struct MyPlugin2;

impl Plugin for MyPlugin2 {
    const NAME: &'static str = "MyPlugin2";
}
