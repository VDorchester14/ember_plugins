use crate::Plugin;
use std::sync::Mutex;
use std::sync::Arc;

lazy_static::lazy_static! {
    pub static ref PLUGIN_REGISTRY: Arc<Mutex<Vec<Box<dyn crate::Plugin>>>> = Arc::new(Mutex::new(Vec::new()));
}

pub fn registry() -> Arc<Mutex<Vec<Box<dyn crate::Plugin>>>> {
    PLUGIN_REGISTRY.clone()
}

pub fn register_plugin(plugin: Box<dyn Plugin>){
    let mut registry = PLUGIN_REGISTRY.lock().unwrap();
    registry.push(plugin);
    println!("Registered a plugin. new len {}", registry.len());
}