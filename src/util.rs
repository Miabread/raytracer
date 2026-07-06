use js_sys::Math;
use wasm_bindgen::JsCast;
use web_sys::Performance;

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (::web_sys::console::log_1(&format!($($t)*).into()))
}

pub fn worker_scope() -> web_sys::DedicatedWorkerGlobalScope {
    js_sys::global()
        .dyn_into::<web_sys::DedicatedWorkerGlobalScope>()
        .unwrap()
}

pub fn performance() -> Performance {
    worker_scope().performance().unwrap()
}

pub const fn interval(min: f64, max: f64) -> Interval {
    Interval::new(min, max)
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const EMPTY: Interval = interval(f64::INFINITY, f64::NEG_INFINITY);
    pub const FULL: Interval = interval(f64::NEG_INFINITY, f64::INFINITY);
    pub const UNIT: Interval = interval(0.0, 1.0);
    pub const HALF: Interval = interval(0.0, 0.5);
    pub const DIAM: Interval = interval(-1.0, 1.0);

    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub const fn size(&self) -> f64 {
        self.max - self.min
    }

    pub const fn contains(&self, a: f64) -> bool {
        self.min <= a && a <= self.max
    }

    pub const fn surrounds(&self, a: f64) -> bool {
        self.min < a && a < self.max
    }

    pub fn clamp(&self, a: f64) -> f64 {
        if a < self.min {
            self.min
        } else if a > self.max {
            self.max
        } else {
            a
        }
    }

    pub fn random_double(&self) -> f64 {
        Math::random() * self.size() + self.min
    }

    pub fn random_integer(&self) -> usize {
        interval(self.min, self.max + 1.0).random_double().round() as usize
    }
}
