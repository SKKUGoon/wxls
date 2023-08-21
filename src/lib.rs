pub mod cell;
pub mod range;

#[macro_use]
mod util;

pub use cell::*;
pub use range::*;
pub use util::*;

// Write unit test for each methods
#[cfg(test)]
mod test {
    mod test_cell;
    mod test_range;
}

mod math {
    mod func;
}
