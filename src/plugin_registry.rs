use crate::Plugin;
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref PLUGIN_REGISTRY: Mutex<Vec<Box<dyn crate::Plugin>>> = Mutex::new(Vec::new());
}

pub fn registry() -> Mutex<Vec<Box<dyn crate::Plugin>>>{
    PLUGIN_REGISTRY
}

pub fn register_plugin(plugin: Box<dyn Plugin>){
    let mut registry = PLUGIN_REGISTRY.lock().unwrap();
    registry.push(plugin);
}