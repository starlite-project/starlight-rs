#![feature(negative_impls, once_cell)]
#![warn(clippy::pedantic, clippy::nursery, clippy::suspicious)]
#![deny(clippy::all)]
#![allow(
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions
)]

pub mod components;
pub mod ext_traits;
pub mod slashies;
pub mod state;

pub use ext_traits::*;

#[macro_export]
macro_rules! debug_unreachable {
    () => {
        $crate::debug_unreachable!("entered unreachable code")
    };
    ($e:expr) => {
        if cfg!(not(debug_assertions)) {
            unsafe { std::hint::unreachable_unchecked() };
        } else {
            panic!($e)
        }
    };
}

#[macro_export]
macro_rules! model {
    ($request:expr) => {{
        $request.exec().await?.model().await?
    }};
}
