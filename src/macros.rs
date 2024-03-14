#[macro_export]
macro_rules! register_plugin {
    ($plugin:ident) => {
        #[no_mangle]
        pub extern "C" fn $plugin() -> *mut dyn Plugin {
            let plugin_instance: Box<dyn Plugin> = Box::new($plugin::new());
            Box::into_raw(plugin_instance)
        }
    };
}