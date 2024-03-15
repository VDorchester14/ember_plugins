pub mod plugin;
pub mod plugin_registry;

pub use plugin_registry::PLUGIN_REGISTRY;
pub use plugin_registry::register_plugin;
pub use plugin::Plugin;