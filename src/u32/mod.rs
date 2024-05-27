pub mod gadgets;
pub mod gates;
pub mod witness;

pub use gadgets::*;

pub const fn ceil_div_usize(a: usize, b: usize) -> usize {
    (a + b - 1) / b
}
