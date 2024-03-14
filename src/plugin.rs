pub trait Plugin{
    fn init(&self);
    fn execute(&self);
    fn shutdown(&self);
    fn name(&self) -> String {
        String::from("Unnamed Plugin")
    }
}