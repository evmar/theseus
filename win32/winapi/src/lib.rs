#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(static_mut_refs)]

pub mod ddraw;
mod dllexport;
pub mod gdi32;
mod heap;
pub mod kernel32;
pub mod user32;
pub use dllexport::{ABIReturn, FromABIParam};

macro_rules! stub {
    ($arg:tt) => {{
        println!("{}:{}: stub: returning {:?}", file!(), line!(), $arg);
        $arg
    }};
}
pub(crate) use stub;
