use lazy_static::lazy_static;
use std::sync::Mutex;

pub trait Plugin: Send + Sync {
    fn init(&self);
    fn execute(&self);
    fn shutdown(&self);
    fn name(&self) -> String {
        String::from("Unnamed Plugin")
    }
}