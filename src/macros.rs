#[macro_export]
macro_rules! c_str {
    ($string: expr) => { concat!($string, "\0") }
}
