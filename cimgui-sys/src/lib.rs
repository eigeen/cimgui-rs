#![allow(
    non_snake_case,
    non_upper_case_globals,
    dead_code,
    unused_variables,
    non_camel_case_types
)]

pub extern crate chlorine as cty;

mod bindings;
pub mod traits;

pub use bindings::*;
