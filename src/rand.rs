extern "C" {
    fn rand() -> i32;
}

pub const RAND_MAX: i32 = 2147483647;

pub fn random_double() -> f64 {
    unsafe { rand() as f64 / (RAND_MAX as f64 + 1.0) }
}
pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
