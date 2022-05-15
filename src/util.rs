use gloo::console::log;


#[allow(unused_unsafe)]
pub fn log(message: String) -> bool {
    unsafe { log!(message) }
    true
}
