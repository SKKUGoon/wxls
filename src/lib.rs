pub mod cell;
pub mod error;
pub mod range;

pub use cell::*;
pub use range::*;

pub mod util {
    #[macro_use]
    pub mod macros;
    pub mod cell_handle;
}

pub mod math {
    mod func;
}

#[cfg(test)]
mod test {
    mod test_cell;
    mod test_range;
    mod test_util;
}
