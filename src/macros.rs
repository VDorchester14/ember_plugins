use crate::plugin::Plugin;
use std::sync::Mutex;

lazy_static::lazy_static! {
    pub static ref PLUGIN_REGISTRY: Mutex<Vec<Box<dyn crate::Plugin>>> = Mutex::new(Vec::new());
}

lazy_static::lazy_static!{
    pub static ref REGISTRATION_FUNCTIONS: Mutex<Vec<fn()>> = Mutex::new(Vec::new());
}

#[macro_export]
macro_rules! register_plugin_entry {
    ($plugin:ident, $register_func:ident) => {
        use ember_plugins::plugin::Plugin;
        use ember_plugins::PLUGIN_REGISTRY;
        use ember_plugins::REGISTRATION_FUNCTIONS;

        // Define the unique registration function
        fn $register_func() {
            let plugin_instance: Box<dyn ember_plugins::Plugin> = Box::new($plugin::new());
            PLUGIN_REGISTRY.lock().unwrap().push(plugin_instance);
        }
        
        // Append the registration function to a global list (pseudo-code)
        REGISTRATION_FUNCTIONS.push($register_func);
    };
}

#[macro_export]
macro_rules! register_plugin {
    ($plugin:ident) => {
        use std::sync::Mutex;
        use ember_plugins::plugin::Plugin;
        use ember_plugins::PLUGIN_REGISTRY;

        // Register the plugin instance in a global registry.
        fn register_plugin_instance() {
            let plugin_instance: Box<dyn ember_plugins::Plugin> = Box::new($plugin::new());
            PLUGIN_REGISTRY.lock().unwrap().push(plugin_instance);
        }

        // Expose a function to create an instance of the plugin and add it to the registry.
        #[no_mangle]
        pub extern "C" fn register() {
            register_plugin_instance();
        }

        // Function to safely access the plugins without exposing internal mutability.
        // This function avoids returning a mutable reference or pointer to the registry.
        #[no_mangle]
        pub extern "C" fn access_plugins<F: FnOnce(&[Box<dyn ember_plugins::plugin::Plugin>])>(f: F) {
            let registry = PLUGIN_REGISTRY.lock().unwrap();
            f(&registry);
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