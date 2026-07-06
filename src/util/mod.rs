pub mod interval;
pub mod vec3;

use wasm_bindgen::JsCast;

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (::web_sys::console::log_1(&format!($($t)*).into()))
}

pub fn worker_scope() -> web_sys::DedicatedWorkerGlobalScope {
    js_sys::global()
        .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
        .unwrap()
}
