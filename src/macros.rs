use crate::plugin::Plugin;
use crate::plugin::PLUGIN_REGISTRY;

#[macro_export]
macro_rules! register_plugin {
    ($plugin:ident) => {
        // Register the plugin instance in a global registry.
        fn register_plugin_instance() {
            use std::sync::Mutex;
            lazy_static::lazy_static! {
                static ref PLUGIN_REGISTRY: Mutex<Vec<Box<dyn $crate::Plugin>>> = Mutex::new(Vec::new());
            }

            let plugin_instance: Box<dyn $crate::Plugin> = Box::new($plugin::new());
            PLUGIN_REGISTRY.lock().unwrap().push(plugin_instance);
        }

        // Expose a function to create an instance of the plugin and add it to the registry.
        #[no_mangle]
        pub extern "C" fn register() {
            register_plugin_instance();
        }

        // Optionally, create a function to return all registered plugins.
        #[no_mangle]
        pub extern "C" fn get_plugins() -> *mut Vec<Box<dyn $crate::Plugin>> {
            let registry = PLUGIN_REGISTRY.lock().unwrap();
            Box::into_raw(Box::new(registry.clone()))
        }
    };
}

// #[macro_export]
// macro_rules! register_plugin {
//     ($plugin:ident) => {
//         #[no_mangle]
//         pub extern "C" fn $plugin() -> *mut dyn Plugin {
//             let plugin_instance: Box<dyn Plugin> = Box::new($plugin::new());
//             Box::into_raw(plugin_instance)
//         }
//     };
// }